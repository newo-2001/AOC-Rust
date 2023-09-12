use aoc_lib::NoSolutionError;
use aoc_runner_api::SolverResult;
use num::Integer;

use super::assembunny::{Cpu, REG_A, ExecutationError};

pub fn solve_part_1(input: &str) -> SolverResult {
    let cpu = Cpu::parse(input)?;

    let first_clock = (0..).try_find(|&n| {
        let mut cpu = cpu.clone();
        *cpu.get_register_mut(REG_A) = n;

        let is_clock_signal = cpu.execute()
            .take(10)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .enumerate()
            .all(|(i, v)| if i.is_even() { v == 0 } else { v == 1 });

        Result::<bool, ExecutationError>::Ok(is_clock_signal)
    })?.ok_or(NoSolutionError)?;

    Ok(Box::new(first_clock))
}