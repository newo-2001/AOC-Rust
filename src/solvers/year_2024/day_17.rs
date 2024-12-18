use anyhow::{bail, Result};
use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::{char, line_ending, u64, u8}, combinator::verify, multi::separated_list0, sequence::{delimited, preceded, separated_pair, tuple}, Parser};
use num::Integer;
use recursive::recursive;
use yuki::{errors::NoSolution, parsing::{parse, NomError, Parsable, ParsingResult}};

use crate::SolverResult;

fn parse_register<'a>(name: char) -> impl Parser<&'a str, u64, NomError<'a>> {
    delimited(
        tuple((
            tag("Register "),
            char(name),
            tag(": ")
        )),
        u64,
        line_ending
    )
}

#[derive(Debug)]
struct Cpu {
    ip: usize,
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u8>,
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
                verify(
                    separated_list0(
                        char(','),
                        verify(u8, |&byte| byte < 8)
                    ),
                    |bytes: &Vec<u8>| bytes.len().is_even()
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

impl FromIterator<u8> for Cpu {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
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
    fn combo(&self, value: u8) -> Result<u64> {
        Ok(match value {
            value if value <= 3 => u64::from(value),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => bail!("Invalid combo operand: {value}")
        })
    }

    fn execute_instruction(&mut self) -> Result<ExecutionState> {
        let Some(&[opcode, operand]) = self.program.get(self.ip..self.ip+2) else {
            return Ok(ExecutionState::Halted)
        };

        match opcode {
            0 => self.a /= 2u64.pow(self.combo(operand)?.try_into()?),
            1 => self.b ^= u64::from(operand),
            2 => self.b = self.combo(operand)? % 8,
            3 if self.a == 0 => {},
            3 => {
                self.ip = operand as usize;
                return Ok(ExecutionState::Running)
            },
            4 => self.b ^= self.c,
            5 => self.stdout.push(u8::try_from(self.combo(operand)? % 8)?),
            6 => self.b = self.a / 2u64.pow(self.combo(operand)?.try_into()?),
            7 => self.c = self.a / 2u64.pow(self.combo(operand)?.try_into()?),
            _ => bail!("Invalid opcode: {opcode}")
        }

        self.ip += 2;
        Ok(ExecutionState::Running)
    }

    fn compute(&mut self) -> Result<&[u8]> {
        while matches!(self.execute_instruction()?, ExecutionState::Running) {};
        Ok(&self.stdout)
    }

    fn iteration(&mut self, a: u64) -> Result<u8> {
        self.a = a;

        loop {
            self.execute_instruction()?;
            if let Some(output) = self.stdout.pop() { return Ok(output); }
        }
    }
    
    #[recursive]
    fn search(&mut self, input: u64, index: usize) -> Result<Option<u64>> {
        for a in 0..1 << 3 {
            let output = (input << 3) | a;
            if self.iteration(output)? != self.program[index] { continue; }

            let value = index
                .checked_sub(1)
                .map_or(Ok(Some(output)), |index| self.search(output, index))?;

            if let Some(value) = value {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut cpu: Cpu = parse(input)?;

    let result = cpu
        .compute()?
        .iter()
        .join(",");

    Ok(Box::new(result))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut cpu: Cpu = parse(input)?;
    let a = (0..(1 << 7))
        .map(|input| cpu.search(input, cpu.program.len() - 1))
        .find_map(|result| match result {
            Ok(Some(value)) => Some(Ok(value)),
            Err(err) => Some(Err(err)),
            Ok(None) => None
        })
        .ok_or(NoSolution)??;

    Ok(Box::new(a))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instructions() {        
        let mut cpu = Cpu::from_iter([2,6]);
        cpu.c = 9;
        cpu.execute_instruction().unwrap();
        assert_eq!(1, cpu.b);

        cpu = Cpu::from_iter([5,0,5,1,5,4]);
        cpu.a = 10;
        assert_eq!(vec![0, 1, 2], cpu.compute().unwrap());

        cpu = Cpu::from_iter([0,1,5,4,3,0]);
        cpu.a = 2024;
        assert_eq!(vec![4,2,5,6,7,7,7,7,3,1,0], cpu.compute().unwrap());
        assert_eq!(0, cpu.a);

        cpu = Cpu::from_iter([1,7]);
        cpu.b = 29;
        cpu.execute_instruction().unwrap();
        assert_eq!(26, cpu.b);

        cpu = Cpu::from_iter([4,0]);
        cpu.b = 2024;
        cpu.c = 43690;
        cpu.execute_instruction().unwrap();
        assert_eq!(cpu.b, 44354);
    }
}