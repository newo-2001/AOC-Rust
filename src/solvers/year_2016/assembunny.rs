use ahash::{HashMap, HashMapExt};
use aoc_lib::parsing::{isize, Parsable, TextParserResult, Map2, lines};
use derive_more::Display;
use nom::{character::complete::{anychar, char}, Parser, bytes::complete::tag, sequence::{preceded, separated_pair}, branch::alt};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Display, Debug)]
pub struct Register(pub char);

impl Parsable<'_> for Register {
    fn parse(input: &str) -> TextParserResult<Self> {
        anychar.map(Self).parse(input)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Value {
    Register(Register),
    Constant(isize)
}

impl Parsable<'_> for Value {
    fn parse(input: &str) -> TextParserResult<Self> {
        Parser::or(
            isize.map(Value::Constant),
            Register::parse.map(Value::Register)
        ).parse(input)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
    Copy(Value, Register),
    Increment(Register),
    Decrement(Register),
    JumpNotZero(Value, Value),
    Toggle(Register),
    NoOperation(Value),
    NoOperation2(Value, Value),
    Output(Value)
}

impl Parsable<'_> for Instruction {
    fn parse(input: &str) -> TextParserResult<Self> {
        alt((
            preceded(tag("cpy "), separated_pair(Value::parse, char(' '), Register::parse)).map2(Self::Copy),
            preceded(tag("jnz "), separated_pair(Value::parse, char(' '), Value::parse)).map2(Self::JumpNotZero),
            preceded(tag("inc "), Register::parse).map(Self::Increment),
            preceded(tag("dec "), Register::parse).map(Self::Decrement),
            preceded(tag("tgl "), Register::parse).map(Self::Toggle),
            preceded(tag("out "), Value::parse).map(Self::Output)
        )).parse(input)
    }
}

#[derive(Clone)]
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
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { 
            instructions,
            registers: HashMap::new(),
            ip: 0
        }
    }

    pub fn execute(&mut self) -> CpuOutput {
        CpuOutput { cpu: self }
    }

    fn execute_till_interrupt(&mut self) -> Result<Option<isize>, ExecutationError> {
        while let Some(&instruction) = self.instructions.get(self.ip) {
            if let Some(output) = self.execute_instruction(instruction)? {
                return Ok(Some(output))
            }
            
            self.optimize();
        }

        Ok(None)
    }

    fn execute_instruction(&mut self, instruction: Instruction) -> Result<Option<isize>, ExecutationError> {
        self.ip += 1;

        match instruction {
            | Instruction::NoOperation(..)
            | Instruction::NoOperation2(..) => {},
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
                        | Instruction::Output(Value::Register(register))
                        | Instruction::Toggle(register) => Instruction::Increment(register),
                        | Instruction::NoOperation(value)
                        | Instruction::Output(value) => Instruction::NoOperation(value),
                        | Instruction::NoOperation2(register, offset) => Instruction::JumpNotZero(register, offset),
                        | Instruction::Copy(from, to) => Instruction::JumpNotZero(from, Value::Register(to)),
                        | Instruction::JumpNotZero(from, offset) => match offset {
                            Value::Register(to) => Instruction::Copy(from, to),
                            Value::Constant(_) => Instruction::NoOperation2(from, offset)
                        },
                    }
                }
            },
            Instruction::Output(value) => {
                return Ok(Some(self.resolve_value(value)));
            }
        }

        Ok(None)
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
}

impl Parsable<'_> for Cpu {
    fn parse(input: &str) -> TextParserResult<Self> {
        lines(Instruction::parse)
            .map(Self::new)
            .parse(input)
    }
}

pub struct CpuOutput<'a> {
    cpu: &'a mut Cpu
}

impl Iterator for CpuOutput<'_> {
    type Item = Result<isize, ExecutationError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cpu.execute_till_interrupt() {
            Err(err) => Some(Err(err)),
            Ok(Some(value)) => Some(Ok(value)),
            Ok(None) => None,
        }
    }
}

pub const REG_A: Register = Register('a');