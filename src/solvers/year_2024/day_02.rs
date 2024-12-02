use anyhow::Result;
use itertools::Itertools;
use nom::{character::complete::{space1, u32}, multi::separated_list0, Parser};
use yuki::parsing::{parse_lines, Parsable, ParsingResult};

use crate::SolverResult;

#[derive(Clone)]
struct Report(Vec<u32>);

impl Parsable<'_> for Report {
    fn parse(input: &str) -> ParsingResult<Self> {
        separated_list0(
            space1,
            u32
        )
        .map(Report)
        .parse(input)
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut iter = self.0.iter().tuple_windows();
        let diff = iter.clone().all(|(left, right)| (1..=3).contains(&left.abs_diff(*right)));
        let ascending = iter.clone().all(|(left, right)| left < right);
        let descending = iter.all(|(left, right)| left > right);

        diff && (ascending || descending)
    }

    fn is_safe_with_tolerance(&self) -> bool {
        let mut report = self.clone();

        (0..self.0.len()).any(|index| {
            let value = report.0.remove(index);
            let safe = report.is_safe();
            report.0.insert(index, value);

            safe
        })
    }
}

fn count_safe_reports(input: &str, predicate: impl Fn(&Report) -> bool) -> Result<usize> {
    let safe = parse_lines::<Report>(input)?
        .into_iter()
        .filter(predicate)
        .count();

    Ok(safe)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let safe = count_safe_reports(input, Report::is_safe)?;
    Ok(Box::new(safe))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let safe = count_safe_reports(input, Report::is_safe_with_tolerance)?;
    Ok(Box::new(safe))
}