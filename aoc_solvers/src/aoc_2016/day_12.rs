use aoc_lib::parsing::{Parsable, TextParser};
use aoc_runner_api::SolverResult;
use itertools::Itertools;

use super::assembunny::{Register, Cpu, REG_A};

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut cpu = Cpu::parse.run(input)?;
    let _: Vec<_> = cpu.execute().try_collect()?;

    Ok(Box::new(cpu.get_register(REG_A)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut cpu = Cpu::parse.run(input)?;
    *cpu.get_register_mut(Register('c')) = 1;
    let _: Vec<_> = cpu.execute().try_collect()?;

    Ok(Box::new(cpu.get_register(REG_A)))
}