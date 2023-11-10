use aoc_lib::{parsing::{ParseError, TextParser, parse_lines}, errors::{NoSolution, NoInput}};
use aoc_runner_api::SolverResult;
use nom::{character::complete::u32, sequence::separated_pair, bytes::complete::tag, combinator::map};
use num::Integer;

struct Scanner {
    depth: u32,
    range: u32
}

impl Scanner {
    fn parse(input: &str) -> Result<Scanner, ParseError> {
        map(
            separated_pair(u32, tag(": "), u32),
            |(depth, range)| Scanner { depth, range }
        ).run(input)
    }

    fn severity(&self) -> u32 {
        self.depth * self.range
    }

    fn cycle(&self) -> u32 {
        (self.range - 1) * 2
    }
}

fn trip_severity<'a>(scanners: impl IntoIterator<Item=&'a Scanner>, time: u32) -> Option<u32> {
    let mut severities = scanners.into_iter()
        .filter_map(|scanner| {
            (scanner.depth + time)
                .is_multiple_of(&scanner.cycle())
                .then_some(scanner.severity())
        }).peekable();
        
    severities.peek()
        .is_some()
        .then(|| severities.sum())
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let scanners = parse_lines(Scanner::parse, input)?;
    let severity = trip_severity(&scanners, 0).unwrap_or(0);
    Ok(Box::new(severity))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let scanners = parse_lines(Scanner::parse, input)?;
    
    let upper_bound = scanners.iter()
        .map(Scanner::cycle)
        .reduce(|a, b| a.lcm(&b))
        .ok_or(NoInput)?;

    let first_safe_trip = (0..upper_bound)
        .find(|&time| trip_severity(&scanners, time).is_none())
        .ok_or(NoSolution)?;

    Ok(Box::new(first_safe_trip))
}