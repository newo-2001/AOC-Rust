use aoc_lib::parsing::parse_lines;
use aoc_runner_api::SolverResult;

fn iterations(jumps: &mut[isize], mutator: impl Fn(isize) -> isize) -> usize {
    let mut ip = 0;

    #[allow(clippy::maybe_infinite_iter)]
    (0..).find(|_| {
        // I don't like this
        if let Some(instruction) = jumps.get_mut(ip) {
            if let Some(updated_ip) = ip.checked_add_signed(*instruction) {
                *instruction += (mutator)(*instruction);
                ip = updated_ip;
                return false;
            }
        }
        true
    }).unwrap()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut jumps = parse_lines(str::parse, input)?;
    let answer = iterations(&mut jumps, |_| 1);

    Ok(Box::new(answer))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut jumps = parse_lines(str::parse, input)?;
    let answer = iterations(&mut jumps, |i| if i >= 3 { -1 } else { 1 });

    Ok(Box::new(answer))
}