use std::collections::HashMap;

use aoc_lib::parsing::{parse_lines, ParseError, Runnable, self};
use aoc_runner_api::SolverResult;
use nom::{character::complete, Parser, bytes::complete::tag, sequence::preceded, combinator::all_consuming, branch::alt};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Register(char);

#[derive(Clone, Copy)]
enum Value {
    Register(Register),
    Constant(i32)
}

#[derive(Clone, Copy)]
enum Instruction {
    Copy(Value, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Value, isize)
}

impl Instruction {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let constant = || complete::i32.map(Value::Constant);
        let register = || complete::anychar.map(Register);
        let value = || constant().or(register().map(Value::Register));

        let copy = preceded(tag("cpy "), value().and(preceded(complete::char(' '), register())))
            .map(|(value, register)| Instruction::Copy(value, register));

        let inc = preceded(tag("inc "), register()).map(Instruction::Increment);
        let dec = preceded(tag("dec "), register()).map(Instruction::Decrement);

        let jnz = preceded(tag("jnz "), value().and(preceded(complete::char(' '), parsing::isize)))
            .map(|(value, offset)| Instruction::JumpNotZero(value, offset));

        all_consuming(alt((copy, inc, dec, jnz))).run(input)
    }
}

struct Cpu {
    instructions: Vec<Instruction>,
    registers: HashMap<Register, i32>,
    ip: usize
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Cpu {
        Cpu { instructions, registers: HashMap::new(), ip: 0 }
    }

    fn execute(&mut self) {
        while let Some(&instruction) = self.instructions.get(self.ip) {
            self.execute_instruction(instruction);
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        self.ip += 1;
        match instruction {
            Instruction::Copy(value, register) => {
                let value = self.resolve_value(value);
                *self.get_register_mut(register) = value;
            },
            Instruction::Increment(register) => {
                *self.get_register_mut(register) += 1;
            },
            Instruction::Decrement(register) => {
                *self.get_register_mut(register) -= 1;
            }
            Instruction::JumpNotZero(value, offset) => {
                if self.resolve_value(value) != 0 {
                    self.ip = (self.ip as isize + offset - 1) as usize;
                }
            }
        }
    }

    fn resolve_value(&self, value: Value) -> i32 {
        match value {
            Value::Constant(constant) => constant,
            Value::Register(register) => self.get_register(register)
        }
    }

    fn get_register(&self, register: Register) -> i32 {
        *self.registers.get(&register).unwrap_or(&0)
    }

    fn get_register_mut(&mut self, register: Register) -> &mut i32 {
        self.registers.entry(register).or_insert(0);
        self.registers.get_mut(&register).unwrap()
    }
}

impl<'a> TryFrom<&'a str> for Cpu {
    type Error = ParseError<'a>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let instructions = parse_lines(Instruction::parse, input)?;
        Ok(Cpu::new(instructions))
    }
}

const REG_A: Register = Register('a');

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut cpu: Cpu = input.try_into()?;
    cpu.execute();

    Ok(Box::new(cpu.get_register(REG_A)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut cpu: Cpu = input.try_into()?;
    *cpu.get_register_mut(Register('c')) = 1;
    cpu.execute();

    Ok(Box::new(cpu.get_register(REG_A)))
}