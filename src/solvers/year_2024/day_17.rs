use anyhow::{bail, Result};
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::{char, line_ending, u32, u8}, combinator::map_res, multi::separated_list0, sequence::{delimited, preceded, separated_pair, tuple}, Parser};
use yuki::parsing::{parse, NomError, Parsable, ParsingResult};

use crate::SolverResult;

fn parse_register<'a>(name: char) -> impl Parser<&'a str, u32, NomError<'a>> {
    delimited(
        tuple((
            tag("Register "),
            char(name),
            tag(": ")
        )),
        u32,
        line_ending
    )
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8)
}

impl TryFrom<(u8, u8)> for Instruction {
    type Error = anyhow::Error;

    fn try_from((opcode, value): (u8, u8)) -> Result<Self, Self::Error> {
        Ok(match opcode {
            0 => Self::Adv(value),
            1 => Self::Bxl(value),
            2 => Self::Bst(value),
            3 => Self::Jnz(value),
            4 => Self::Bxc,
            5 => Self::Out(value),
            6 => Self::Bdv(value),
            7 => Self::Cdv(value),
            _ => bail!("Invalid opcode: {opcode}")
        })
    }
}

#[derive(Debug)]
struct Cpu {
    ip: usize,
    a: u32,
    b: u32,
    c: u32,
    program: Vec<Instruction>,
    stdout: Vec<u8>
}

impl<'a> Parsable<'a> for Cpu {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        separated_pair(
            tuple((
                parse_register('A'),
                parse_register('B'),
                parse_register('C'),
            )),
            line_ending,
            preceded(
                tag("Program: "),
                separated_list0(
                    char(','),
                    map_res(
                        separated_pair(u8, char(','), u8),
                        Instruction::try_from
                    )
                )
            )
        )
        .map(|((a, b, c), program)| Self {
            a, b, c, program, ip: 0, stdout: Vec::new()
        })
        .parse(input)
    }
}

enum ExecutionState {
    Running,
    Halted
}

impl FromIterator<Instruction> for Cpu {
    fn from_iter<T: IntoIterator<Item = Instruction>>(iter: T) -> Self {
        Self {
            ip: 0,
            a: 0,
            b: 0,
            c: 0,
            stdout: Vec::new(),
            program: iter.into_iter().collect()
        }
    }
}

impl Cpu {
    fn combo(&self, value: u8) -> Result<u32> {
        Ok(match value {
            value if value <= 3 => u32::from(value),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => bail!("Invalid combo operand: {value}")
        })
    }

    fn execute_instruction(&mut self) -> Result<ExecutionState> {
        let Some(&instruction) = self.program.get(self.ip) else {
            return Ok(ExecutionState::Halted)
        };

        dbg!(&self);

        match instruction {
            Instruction::Adv(value) => self.a /= 2u32.pow(self.combo(value)?),
            Instruction::Bxl(value) => self.b ^= u32::from(value),
            Instruction::Bst(value) => self.b = self.combo(value)? % 8,
            Instruction::Jnz(_) if self.a == 0 => {},
            Instruction::Jnz(value) => {
                self.ip = value as usize;
                return Ok(ExecutionState::Running)
            },
            Instruction::Bxc => self.b ^= self.c,
            Instruction::Out(value) => self.stdout.push(u8::try_from(self.combo(value)? % 8)?),
            Instruction::Bdv(value) => self.b = self.a / 2u32.pow(self.combo(value)?),
            Instruction::Cdv(value) => self.c = self.a / 2u32.pow(self.combo(value)?),
        }

        self.ip += 1;
        Ok(ExecutionState::Running)
    }

    fn compute(&mut self) -> Result<Vec<u8>> {
        while matches!(self.execute_instruction()?, ExecutionState::Running) {};
        Ok(self.flush())
    }

    fn flush(&mut self) -> Vec<u8> {
        let mut result = Vec::new();
        std::mem::swap(&mut self.stdout, &mut result);
        result
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut cpu: Cpu = parse(input)?;

    let result = cpu
        .compute()?
        .into_iter()
        .join(",");

    Ok(Box::new(result))
}

#[cfg(test)]
mod test {
    use std::iter::once;
    use super::*;

    #[test]
    fn test_instructions() {        
        let mut cpu: Cpu = once(Instruction::Bst(6)).collect();
        cpu.c = 9;
        cpu.execute_instruction().unwrap();
        assert_eq!(1, cpu.b);

        cpu = Cpu::from_iter([
            Instruction::Out(0),
            Instruction::Out(1),
            Instruction::Out(4)
        ]);
        cpu.a = 10;
        assert_eq!(vec![0, 1, 2], cpu.compute().unwrap());

        cpu = Cpu::from_iter([
            Instruction::Adv(1),
            Instruction::Out(4),
            Instruction::Jnz(0)
        ]);
        cpu.a = 2024;
        assert_eq!(vec![4,2,5,6,7,7,7,7,3,1,0], cpu.compute().unwrap());
        assert_eq!(0, cpu.a);

        cpu = once(Instruction::Bxl(7)).collect();
        cpu.b = 29;
        cpu.execute_instruction().unwrap();
        assert_eq!(26, cpu.b);

        cpu = once(Instruction::Bxc).collect();
        cpu.b = 2024;
        cpu.c = 43690;
        cpu.execute_instruction().unwrap();
        assert_eq!(cpu.b, 44354);
    }
}