use std::iter::once;

use aoc_lib::{parsing::{TextParserResult, TextParser, isize, Parsable, Map2, lines}, cpu::{self, Jump, ControlFlow}};
use crate::SolverResult;
use nom::{
    Parser,
    character::complete::char,
    sequence::{preceded, separated_pair},
    branch::alt, combinator::value,
    bytes::complete::tag
};

type Cpu<'a> = cpu::Cpu<'a, Instruction, Register, u32>;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Register {
    A, B
}

impl Parsable<'_> for Register {
    fn parse(input: &str) -> TextParserResult<'_, Self> {
        Parser::or(
        value(Self::A, char('a')),
            value(Self::B, char('b'))       
        ).parse(input)
    }
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

impl Parsable<'_> for Instruction {
    fn parse(input: &str) -> TextParserResult<'_, Self> {
        alt((
            preceded(tag("hlf "), Register::parse).map(Instruction::Half),
            preceded(tag("tpl "), Register::parse).map(Instruction::Triple),
            preceded(tag("inc "), Register::parse).map(Instruction::Increment),
            preceded(tag("jmp "), isize).map(Instruction::Jump),
            preceded(tag("jie "), separated_pair(Register::parse, tag(", "), isize))
                .map2(Instruction::JumpIfEven),
            preceded(tag("jio "), separated_pair(Register::parse, tag(", "), isize))
                .map2(Instruction::JumpIfOne)
        )).parse(input)
    }
}

impl cpu::Instruction<Register, u32> for Instruction {
    type Interrupt = !;
    
    fn execute(&self, cpu: &mut Cpu) -> ControlFlow<Self::Interrupt> {
        match self {
            Self::Half(register) => cpu.map_register(*register, |x| x / 2),
            Self::Triple(register) => cpu.map_register(*register, |x| x * 3),
            Self::Increment(register) => cpu.map_register(*register, |x| x + 1),
            Self::Jump(offset) => return ControlFlow::Jump(Jump::Relative(*offset)),
            Self::JumpIfEven(register, offset) => {
                if cpu.read_register(register).is_multiple_of(2) {
                    return ControlFlow::Jump(Jump::Relative(*offset));
                }
            },
            Self::JumpIfOne(register, offset) => {
                if cpu.read_register(register) == 1 {
                    return ControlFlow::Jump(Jump::Relative(*offset));
                }
            },
        };

        ControlFlow::Continue
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let program = lines(Instruction::parse).run(input)?;
    let mut cpu = Cpu::new(&program);
    cpu.execute();

    Ok(Box::new(cpu.read_register(&Register::B)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let program = lines(Instruction::parse).run(input)?;
    let mut cpu = Cpu::with_registers(&program, once((Register::A, 1)));
    cpu.execute();

    Ok(Box::new(cpu.read_register(&Register::B)))
}