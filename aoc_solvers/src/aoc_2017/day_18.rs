use std::{collections::VecDeque, fmt::Debug};
use anyhow::anyhow;
use aoc_lib::{NoSolutionError, cpu::{self, Jump, ControlFlow}, parsing::{Parsable, TextParserResult, TextParser, Map2, lines}};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{branch::alt, bytes::complete::tag, character::complete::{satisfy, char}, Parser, sequence::{separated_pair, preceded}};
use thiserror::Error;

type Cpu<'a> = cpu::Cpu<'a, Instruction, Register, i64>;
type Value = cpu::Value<Register, i64>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Register {
    GeneralPurpose(char),
    Sound
}

impl Parsable<'_> for Register {
    fn parse(input: &str) -> TextParserResult<Self> {
        satisfy(|c| c.is_ascii_lowercase())
            .map(Register::GeneralPurpose)
            .parse(input)
    }
}

#[derive(Debug)]
enum Interrupt {
    Send(i64),
    Receive(Register)
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Sound(Value),
    Set(Register, Value),
    Add(Register, Value),
    Multiply(Register, Value),
    Modulo(Register, Value),
    Recover(Value),
    JumpGreaterThanZero(Value, Value),
    Send(Value),
    Receive(Register)
}

impl Parsable<'_> for Instruction {
    fn parse(input: &str) -> TextParserResult<Self> {
        fn binary_operator<'a, X, Y, O, M>(name: &'a str, mapper: M) -> impl TextParser<'a, O>
            where M: Fn(X, Y) -> O,
                  X: Parsable<'a>, Y: Parsable<'a>
        {
            preceded(
                tag(name).and(char(' ')),
                separated_pair(X::parse, char(' '), Y::parse)
            ).map2(mapper)
        }

        alt((
            preceded(tag("snd "), Value::parse).map(Self::Sound),
            preceded(tag("rcv "), Value::parse).map(Self::Recover),
            binary_operator("set", Self::Set),
            binary_operator("add", Self::Add),
            binary_operator("mul", Self::Multiply),
            binary_operator("mod", Self::Modulo),
            binary_operator("jgz", Self::JumpGreaterThanZero),
        )).parse(input)
    }
}

impl cpu::Instruction<Register, i64> for Instruction {
    type Interrupt = Interrupt;

    fn execute(&self, cpu: &mut Cpu) -> ControlFlow<Self::Interrupt> {
        match self {
            Self::Sound(value) => cpu.write_register(Register::Sound, cpu.evaluate(value)),
            Self::Set(register, value) => cpu.write_register(*register, cpu.evaluate(value)),
            Self::Add(register, value) => {
                let value = cpu.evaluate(value);
                cpu.map_register(*register, |acc| acc + value);
            },
            Self::Multiply(register, value) => {
                let value = cpu.evaluate(value);
                cpu.map_register(*register, |acc| acc * value);
            },
            Self::Modulo(register, value) => {
                let value = cpu.evaluate(value);
                cpu.map_register(*register, |acc| acc.rem_euclid(value));
            },
            Self::Recover(value) => {
                if cpu.evaluate(value) != 0 {
                    let sound = cpu.read_register(&Register::Sound);
                    return ControlFlow::Interrupt(Interrupt::Send(sound));
                }
            },
            Self::JumpGreaterThanZero(value, offset) => {
                if cpu.evaluate(value) > 0 {
                    return match isize::try_from(cpu.evaluate(offset)) {
                        Ok(offset) => ControlFlow::Jump(Jump::Relative(offset)),
                        Err(_) => ControlFlow::Exit
                    }
                }
            },
            Self::Send(value) => {
                return ControlFlow::Interrupt(Interrupt::Send(cpu.evaluate(value)))
            },
            Self::Receive(target) => {
                return ControlFlow::Interrupt(Interrupt::Receive(*target))
            }
        };

        ControlFlow::Continue
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    NoSolution(#[from] NoSolutionError),
    #[error("Unexpected interrupt: {0:?}")]
    UnexpectedInterrupt(Interrupt),
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let program = lines(Instruction::parse).run(input)?;
    let mut cpu = Cpu::new(&program);
    let sound = match cpu.execute() {
        Some(Interrupt::Send(result)) => Ok(result),
        Some(interrupt @ Interrupt::Receive(_)) => Err(Error::UnexpectedInterrupt(interrupt)),
        None => Err(Error::NoSolution(NoSolutionError))
    }?;

    Ok(Box::new(sound))
}

#[derive(Debug)]
struct Thread<'a> {
    cpu: Cpu<'a>,
    stdin: VecDeque<i64>,
    awaiting: Option<Register>,
    sends: u32
}

impl<'a> Thread<'a> {
    fn new(cpu: Cpu<'a>) -> Self {
        Self { cpu, stdin: VecDeque::new(), awaiting: None, sends: 0 }
    }

    fn execute(&mut self, stdout: &mut Thread) {
        if self.cpu.has_terminated() { return; }
        
        match (self.awaiting, self.stdin.front()) {
            (Some(target), Some(&data)) => {
                self.stdin.pop_front();
                self.cpu.write_register(target, data);
                self.awaiting = None;
            }
            (Some(_), None) => return,
            (None, _) => {}
        }

        match self.cpu.execute() {
            Some(Interrupt::Receive(register)) => self.awaiting = Some(register),
            Some(Interrupt::Send(data)) => {
                stdout.stdin.push_back(data);
                self.sends += 1;
            },
            None => {}
        }
    }
    
    fn is_waiting(&self) -> bool { self.awaiting.is_some() && self.stdin.is_empty() }
}

pub fn solve_part_2(input: &str) -> SolverResult {
    const PID_REGISTER: Register = Register::GeneralPurpose('p');

    let program = lines(Instruction::parse).run(input)?
        .into_iter()
        .map(|instruction| match instruction {
            Instruction::Sound(value) => Ok(Instruction::Send(value)),
            Instruction::Recover(Value::Register(register)) => Ok(Instruction::Receive(register)),
            Instruction::Recover(Value::Constant(_)) => Err(anyhow!("Failed to convert rcv instruction, it has to reference a register")),
            instruction => Ok(instruction)
        }).try_collect()?;
    
    let (mut cpu_0, mut cpu_1) = (Cpu::new(&program), Cpu::new(&program));
    cpu_0.write_register(PID_REGISTER, 0);
    cpu_1.write_register(PID_REGISTER, 1);

    let (mut thread_0, mut thread_1) = (Thread::new(cpu_0), Thread::new(cpu_1));
    
    // Could be fun to try to make this actually concurrent
    while !thread_0.cpu.has_terminated() || !thread_1.cpu.has_terminated() {
        thread_0.execute(&mut thread_1);
        thread_1.execute(&mut thread_0);

        if thread_0.is_waiting() && thread_1.is_waiting() { break; }
    }

    Ok(Box::new(thread_1.sends))
}