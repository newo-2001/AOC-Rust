use ahash::{HashMap, HashMapExt};
use nom::Parser;
use std::hash::Hash;

use crate::parsing::{Parsable, TextParserResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CpuState {
    Initial,
    Executing,
    Terminated
}

#[derive(Debug, Clone)]
pub struct Cpu<'a, I, R, D> {
    ip: usize,
    registers: HashMap<R, D>,
    instructions: &'a Vec<I>,
    state: CpuState
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Jump {
    Absolute(usize),
    Relative(isize)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Value<R, D> {
    Register(R),
    Constant(D)
}

impl<'a, R: Parsable<'a>, D: Parsable<'a>> Parsable<'a> for Value<R, D> {
    fn parse(input: &'a str) -> TextParserResult<'a, Self> {
        Parser::or(
            R::parse.map(Value::Register),
            D::parse.map(Value::Constant)
        ).parse(input)
    }
}

pub trait Instruction<R, D>: Sized {
    type Interrupt;

    fn execute(&self, cpu: &mut Cpu<Self, R, D>) -> ControlFlow<Self::Interrupt>;
}

pub enum ControlFlow<I> {
    Continue,
    Interrupt(I),
    Jump(Jump),
    Exit
}

impl<'a, I, R, D> Cpu<'a, I, R, D> {
    #[must_use]
    pub fn new(instructions: &'a Vec<I>) -> Self {
        Self {
            ip: 0,
            registers: HashMap::new(),
            state: CpuState::Initial,
            instructions,
        }
    }

    pub fn jump(&mut self, jump: Jump) {
        self.ip = match jump {
            Jump::Absolute(address) => address,
            Jump::Relative(offset) => self.ip.wrapping_add_signed(offset)
        }
    }

    #[must_use]
    pub const fn state(&self) -> CpuState { self.state }

    #[must_use]
    pub fn has_terminated(&self) -> bool { self.state == CpuState::Terminated }
}

impl<I, R, D> Cpu<'_, I, R, D>
    where R: Eq + Hash
{
    pub fn write_register(&mut self, register: R, data: D) {
        self.registers.insert(register, data);
    }
}

impl<'a, I, R, D> Cpu<'a, I, R, D>
    where R: Eq + Hash
{
    pub fn with_registers(instructions: &'a Vec<I>, registers: impl IntoIterator<Item=(R, D)>) -> Self {
        Self {
            ip: 0,
            registers: registers.into_iter().collect(),
            state: CpuState::Initial,
            instructions,
        }
    }
}

impl<I, R, D> Cpu<'_, I, R, D>
    where R: Hash + Eq,
          D: Default + Clone
{
    pub fn read_register(&self, register: &R) -> D {
        self.registers.get(register)
            .cloned()
            .unwrap_or_default()
    }

    pub fn map_register(&mut self, register: R, mapper: impl FnOnce(D) -> D) {
        let value = self.registers.remove(&register)
            .unwrap_or_default();

        self.write_register(register, mapper(value));
    }

    pub fn move_register(&mut self, from: &R, to: R) {
        self.write_register(to, self.read_register(from));
    }

    pub fn evaluate(&self, value: &Value<R, D>) -> D {
        match value {
            Value::Register(register) => self.read_register(register),
            Value::Constant(constant) => constant.clone()
        }
    }
}

impl<'a, I, R, D> Cpu<'a, I, R, D>
    where I: Instruction<R, D> + Clone
{
    pub fn execute(&mut self) -> Option<I::Interrupt> {
        match self.state {
            CpuState::Initial => self.state = CpuState::Executing,
            CpuState::Terminated => return None,
            CpuState::Executing => {}
        }

        while let Some(instruction) = self.instructions.get(self.ip).cloned() {   
            let flow = instruction.execute(self);

            self.jump(match flow {
                ControlFlow::Continue | ControlFlow::Interrupt(_) => Jump::Relative(1),
                ControlFlow::Jump(jump) => jump,
                ControlFlow::Exit => break
            });

            if let ControlFlow::Interrupt(interrupt) = flow {
                return Some(interrupt);
            }
        }

        self.state = CpuState::Terminated;
        None
    }

    #[must_use]
    pub const fn stream(self) -> InterruptStream<'a, I, R, D> {
        InterruptStream { cpu: self }
    }
}

pub struct InterruptStream<'a, I, R, D> {
    cpu: Cpu<'a, I, R, D>
}

impl<I, R, D> Iterator for InterruptStream<'_, I, R, D>
    where I: Instruction<R, D> + Clone,
{
    type Item = I::Interrupt;

    fn next(&mut self) -> Option<Self::Item> {
        self.cpu.execute()
    }
}