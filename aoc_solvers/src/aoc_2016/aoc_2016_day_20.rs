use aoc_lib::{parsing::{ParseError, Runnable, sep_by, parse_lines}, math::Range, NoSolutionError};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{character::complete, Parser};

// Use u64 because range is represented using an exclusive upper bound
// u32::MAX appears in the input and would require u32::MAX + 1 to represent this
fn parse_range(input: &str) -> Result<Range<u64>, ParseError> {
    sep_by(complete::u64, complete::char('-'), complete::u64)
        .map(|(start, end)| Range::inclusive(start, end))
        .run(input)
}

pub fn merge_ranges(ranges: impl IntoIterator<Item=Range<u64>>) -> impl Iterator<Item=Range<u64>> {
    ranges.into_iter()
        .sorted_by_key(|range| range.start)
        .coalesce(Range::merge)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let ranges = parse_lines(parse_range, input)?;
    let first_valid_ip = match merge_ranges(ranges).next() {
        Some(Range { start, .. }) if start > 0 => 0,
        Some(Range { end, .. }) => end,
        None => Err(NoSolutionError)?
    };

    Ok(Box::new(first_valid_ip))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let ranges = parse_lines(parse_range, input)?;
    let valid_ips: u64 = merge_ranges(ranges)
        .tuple_windows()
        .map(|(Range { end, .. }, Range { start, ..})| {
            Range::exclusive(end, start).interval()
        }).sum();

    Ok(Box::new(valid_ips))
}
