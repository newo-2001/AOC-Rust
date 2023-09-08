use aoc_runner_api::SolverResult;
use num::PrimInt;

// https://www.youtube.com/watch?v=uCsD3ZGzMgE
pub fn solve_part_1(input: &str) -> SolverResult {
    let num_elves: usize = input.parse()?;
    let index = num_elves.ilog2();
    let last_bit: usize = num_elves >> index as usize;
    let leading = (num_elves & (2.pow(index) - 1)) << 1;

    Ok(Box::new(last_bit | leading))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let num_elves: usize = input.parse()?;
    let power_3 = (0..).map(|n| 3.pow(n))
        .take_while(|&n| n <= num_elves)
        .last()
        .unwrap();

    // There is probably a way to simplyify this expression
    let start = num_elves - power_3;
    let result = if num_elves == power_3 { num_elves }
    else if start <= power_3 { start }
    else { power_3 + (num_elves - power_3 * 2) * 2 };

    Ok(Box::new(result))
}