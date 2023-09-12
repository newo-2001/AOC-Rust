use aoc_runner_api::SolverResult;

use super::assembunny::{Register, Cpu};

const REG_A: Register = Register('a');

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut cpu = Cpu::parse(input)?;
    cpu.execute()?;

    Ok(Box::new(cpu.get_register(REG_A)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut cpu = Cpu::parse(input)?;
    *cpu.get_register_mut(Register('c')) = 1;
    cpu.execute()?;

    Ok(Box::new(cpu.get_register(REG_A)))
}