use aoc_runner_api::SolverResult;

use super::assembunny::{Cpu, Register};

const REG_A: Register = Register('a');

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut cpu = Cpu::parse(input)?;
    *cpu.get_register_mut(REG_A) = 7;
    cpu.execute()?;

    Ok(Box::new(cpu.get_register(REG_A)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut cpu = Cpu::parse(input)?;
    *cpu.get_register_mut(REG_A) = 12;
    cpu.execute()?;

    Ok(Box::new(cpu.get_register(REG_A)))
}