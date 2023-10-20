use aoc_lib::parsing::{TextParserResult, parse_lines, TextParser, ParseError, isize};
use aoc_runner_api::SolverResult;
use nom::{
    Parser,
    character::complete::char,
    sequence::{preceded, terminated},
    branch::alt, combinator::value,
    bytes::complete::tag
};
use num::Integer;
use thiserror::Error;

#[derive(Clone, Copy)]
enum Register {
    A, B
}

#[derive(Clone, Copy)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize)
}

struct State {
    a: u32,
    b: u32,
    ip: usize
}

#[derive(Debug, Error)]
enum ExecutationError {
    #[error("Jumped to a negative instruction")]
    NegativeInstructionPointer
}

impl State {
    fn execute(&mut self, instruction: Instruction) -> Result<(), ExecutationError> {
        match instruction {
            Instruction::Half(register) => { self.map_register(register, |x| x / 2) }
            Instruction::Triple(register) => { self.map_register(register, |x| x * 3) }
            Instruction::Increment(register) => { self.map_register(register, |x| x + 1) }
            Instruction::Jump(offset) => {
                self.ip = self.ip.checked_add_signed(offset)
                    .ok_or(ExecutationError::NegativeInstructionPointer)?;
                return Ok(());
            },
            Instruction::JumpIfEven(register, offset) => {
                if self.read_register(register).is_even() {
                    return self.execute(Instruction::Jump(offset));
                }
            },
            Instruction::JumpIfOne(register, offset) => {
                if self.read_register(register) == 1 {
                    return self.execute(Instruction::Jump(offset));
                }
            }
        }

        self.ip += 1;
        Ok(())
    }
}

impl State {
    fn read_register(&self, register: Register) -> u32 {
        match register {
            Register::A => self.a,
            Register::B => self.b
        }
    }

    fn write_register(&mut self, register: Register, value: u32) {
        *match register {
            Register::A => &mut self.a,
            Register::B => &mut self.b
        } = value;
    }
    
    fn map_register(&mut self, register: Register, mapper: impl Fn(u32) -> u32) {
        self.write_register(register, mapper(self.read_register(register)));
    }
}

fn parse_instruction(input: &str) -> Result<Instruction, ParseError> {
    fn register(input: &str) -> TextParserResult<Register> {
        value(Register::A, char('a'))
            .or(value(Register::B, char('b')))
            .parse(input)
    }
    
    alt((
        preceded(tag("hlf "), register).map(Instruction::Half),
        preceded(tag("tpl "), register).map(Instruction::Triple),
        preceded(tag("inc "), register).map(Instruction::Increment),
        preceded(tag("jmp "), isize).map(Instruction::Jump),
        preceded(tag("jie "), terminated(register, tag(", ")).and(isize))
            .map(|(register, isize)| Instruction::JumpIfEven(register, isize)),
        preceded(tag("jio "), terminated(register, tag(", ")).and(isize))
            .map(|(register, isize)| Instruction::JumpIfOne(register, isize))
    )).run(input)
}

fn run_program(program: &[Instruction], mut state: State) -> Result<State, ExecutationError> {
    while let Some(&instruction) = program.get(state.ip) {
        state.execute(instruction)?;
    }

    Ok(state)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let program = parse_lines(parse_instruction, input)?;
    let State { b, .. } = run_program(&program, State { a: 0, b: 0, ip: 0 })?;

    Ok(Box::new(b))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let program = parse_lines(parse_instruction, input)?;
    let State { b, .. } = run_program(&program, State { a: 1, b: 0, ip: 0 })?;

    Ok(Box::new(b))
}