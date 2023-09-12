use std::collections::HashMap;

use aoc_lib::parsing::{parse_lines, ParseError, Runnable, self};
use derive_more::Display;
use nom::{character::complete, Parser, bytes::complete::tag, sequence::preceded, combinator::all_consuming, branch::alt};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug)]
pub struct Register(pub char);

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Register(Register),
    Constant(isize)
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Copy(Value, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Value, Value),
    Toggle(Register),
    NoOperation(Value, Value)
}

impl Instruction {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let constant = || parsing::isize.map(Value::Constant);
        let register = || complete::anychar.map(Register);
        let value = || constant().or(register().map(Value::Register));

        let copy = preceded(tag("cpy "), value().and(preceded(complete::char(' '), register())))
            .map(|(value, register)| Instruction::Copy(value, register));

        let inc = preceded(tag("inc "), register()).map(Instruction::Increment);
        let dec = preceded(tag("dec "), register()).map(Instruction::Decrement);

        let jnz = preceded(tag("jnz "), value().and(preceded(complete::char(' '), value())))
            .map(|(value, offset)| Instruction::JumpNotZero(value, offset));
        
        let tgl = preceded(tag("tgl "), register()).map(Instruction::Toggle);

        all_consuming(alt((copy, inc, dec, jnz, tgl))).run(input)
    }
}

pub struct Cpu {
    instructions: Vec<Instruction>,
    registers: HashMap<Register, isize>,
    ip: usize
}

#[derive(Debug, Error)]
pub enum ExecutationError {
    #[error("Jumped to negative instruction")]
    NegativeInstructionPointer,
    #[error("Register {0} overflowed")]
    RegisterOverflow(Register)
}

impl Cpu {
    pub fn new(instructions: Vec<Instruction>) -> Cpu {
        Cpu { instructions, registers: HashMap::new(), ip: 0 }
    }

    pub fn execute(&mut self) -> Result<(), ExecutationError> {
        while let Some(&instruction) = self.instructions.get(self.ip) {
            self.execute_instruction(instruction)?;
            self.optimize();
        }

        Ok(())
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), ExecutationError> {
        self.ip += 1;

        match instruction {
            Instruction::NoOperation(..) => {},
            Instruction::Copy(value, register) => {
                let value = self.resolve_value(value);
                *self.get_register_mut(register) = value;
            },
            Instruction::Increment(register) => {
                let value = self.get_register_mut(register);
                *value = value.checked_add(1)
                    .ok_or(ExecutationError::RegisterOverflow(register))?;
            },
            Instruction::Decrement(register) => {
                let value = self.get_register_mut(register);
                *value = value.checked_sub(1)
                    .ok_or(ExecutationError::RegisterOverflow(register))?;
            }
            Instruction::JumpNotZero(value, offset) => {
                if self.resolve_value(value) != 0 {
                    let offset = self.resolve_value(offset);
                    self.ip = self.ip.checked_add_signed(offset - 1)
                        .ok_or(ExecutationError::NegativeInstructionPointer)?;
                }
            },
            Instruction::Toggle(register) => {
                let offset = self.get_register(register);
                let index = self.ip.checked_add_signed(offset - 1);
                if let Some(target) = index.and_then(|index| self.instructions.get_mut(index)) {
                    *target = match *target {
                        | Instruction::Increment(register) => Instruction::Decrement(register),
                        | Instruction::Decrement(register)
                        | Instruction::Toggle(register) => Instruction::Increment(register),
                        | Instruction::NoOperation(register, offset) => Instruction::JumpNotZero(register, offset),
                        | Instruction::Copy(from, to) => Instruction::JumpNotZero(from, Value::Register(to)),
                        | Instruction::JumpNotZero(from, offset) => match offset {
                            Value::Register(to) => Instruction::Copy(from, to),
                            Value::Constant(_) => Instruction::NoOperation(from, offset)
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn optimize(&mut self) {
        let (_, after) = self.instructions.split_at(self.ip);
        match *after {
            [
                Instruction::Increment(target),
                Instruction::Decrement(a),
                Instruction::JumpNotZero(Value::Register(check_a), Value::Constant(-2)),
                Instruction::Decrement(b),
                Instruction::JumpNotZero(Value::Register(check_b), Value::Constant(-5)),
                ..
            ] if check_a == a && check_b == b => {
                let result = self.get_register(a) * self.get_register(b);
                *self.get_register_mut(target) += result;
                *self.get_register_mut(a) = 0;
                *self.get_register_mut(b) = 0;
                self.ip += 5;
            },
            _ => {}
        }
    }

    fn resolve_value(&self, value: Value) -> isize {
        match value {
            Value::Constant(constant) => constant,
            Value::Register(register) => self.get_register(register)
        }
    }

    pub fn get_register(&self, register: Register) -> isize {
        *self.registers.get(&register).unwrap_or(&0)
    }

    pub fn get_register_mut(&mut self, register: Register) -> &mut isize {
        self.registers.entry(register).or_insert(0);
        self.registers.get_mut(&register).unwrap()
    }

    pub fn parse(input: &str) -> Result<Cpu, ParseError> {
        let instructions = parse_lines(Instruction::parse, input)?;
        Ok(Cpu::new(instructions))
    }
}