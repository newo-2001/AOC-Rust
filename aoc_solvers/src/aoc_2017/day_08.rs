use std::{collections::HashMap, cmp::Ordering};

use aoc_lib::{parsing::{ParseError, TextParserResult, Runnable, parse_lines}, NoSolutionError};
use aoc_runner_api::SolverResult;
use nom::{combinator::value, Parser, bytes::complete::tag, character::complete::{alpha1, i32}, sequence::{preceded, tuple}, branch::alt};

#[derive(Clone, Copy)]
enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual
}

impl ComparisonOperator {
    fn parse(input: &str) -> TextParserResult<Self> {
        alt((
            value(Self::Equal, tag(" == ")),
            value(Self::NotEqual, tag(" != ")),
            value(Self::GreaterThan, tag(" > ")),
            value(Self::GreaterThanEqual, tag( " >= ")),
            value(Self::LessThan, tag( " < ")),
            value(Self::LessThanEqual, tag( " <= ")),
        )).parse(input)
    }

    fn evaluate<T>(self, left: &T, right: &T) -> bool where T: Ord {
        match (self, left.cmp(right)) {
            | (Self::Equal, Ordering::Equal)
            | (Self::NotEqual, Ordering::Greater | Ordering::Less)
            | (Self::GreaterThan, Ordering::Greater)
            | (Self::GreaterThanEqual, Ordering::Greater | Ordering::Equal)
            | (Self::LessThan, Ordering::Less)
            | (Self::LessThanEqual, Ordering::Less | Ordering::Equal) => true,
            | _ => false
        }
    }
}

struct Condition<'a> {
    left: &'a str,
    right: i32,
    operator: ComparisonOperator
}

impl<'a> Condition<'a> {
    fn parse(input: &'a str) -> TextParserResult<Self>{
        tuple((
            preceded(tag(" if "), alpha1),
            ComparisonOperator::parse,
            i32
        )).map(|(left, operator, right)| Condition {
            left, right, operator
        }).parse(input)
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Increment(i32),
    Decrement(i32)
}

impl Operation {
    fn parse(input: &str) -> TextParserResult<Self> {
        Parser::or(
            preceded(tag(" inc "), i32).map(Self::Increment),
            preceded(tag(" dec "), i32).map(Self::Decrement)
        ).parse(input)
    }
}

struct Instruction<'a> {
    target: &'a str,
    condition: Condition<'a>,
    operation: Operation
}

impl Instruction<'_> {
    fn parse(input: &str) -> Result<Instruction, ParseError> {
        tuple((
            alpha1,
            Operation::parse,
            Condition::parse
        )).map(|(target, operation, condition)| Instruction {
            target, condition, operation
        }).run(input)
    }
}

struct Cpu<'a> {
    registers: HashMap<&'a str, i32>
}

impl<'a> Cpu<'a> {
    fn new() -> Self {
        Cpu { registers: HashMap::new() }
    }

    fn execute(&mut self, Instruction { condition, target, operation }: &Instruction<'a>) {
        let left = self.get_register(condition.left);
        if !condition.operator.evaluate(&left, &condition.right) { return; }
        
        let target = self.get_register_mut(target);
        match operation {
            Operation::Increment(amount) => *target += amount,
            Operation::Decrement(amount) => *target -= amount
        }
    }

    fn get_register(&self, register: &str) -> i32 {
        *self.registers.get(register).unwrap_or(&0)
    }

    fn get_register_mut(&mut self, register: &'a str) -> &mut i32 {
        if !self.registers.contains_key(register) {
            self.registers.insert(register, 0);
        }

        self.registers.get_mut(register).unwrap()
    }

    fn max_register(&self) -> Option<i32> {
        self.registers.values().max().copied()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let instructions = parse_lines(Instruction::parse, input)?;
    let mut cpu = Cpu::new();

    instructions.iter().for_each(|instruction| cpu.execute(instruction));

    let max_register = cpu.max_register().ok_or(NoSolutionError)?;
    Ok(Box::new(max_register))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let instructions = parse_lines(Instruction::parse, input)?;
    let mut cpu = Cpu::new();

    let max_register = instructions.iter().fold(None, |max, instruction| {
        cpu.execute(instruction);
        max.max(cpu.max_register())
    }).ok_or(NoSolutionError)?;

    Ok(Box::new(max_register))
}