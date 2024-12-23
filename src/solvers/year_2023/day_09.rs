use aoc_lib::{parsing::{parse_lines, ParseError, TextParser}, iteration::generate};
use crate::SolverResult;
use itertools::Itertools;
use nom::{multi::separated_list0, character::complete::{char, i32}, Parser};

struct History(Vec<i32>);

impl History {
    fn parse(input: &str) -> Result<Self, ParseError> {
        separated_list0(char(' '), i32).map(Self).run(input)
    }

    fn difference(&self) -> Self {
        let history = self.0.iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        Self(history)
    }

    fn differences(self) -> impl Iterator<Item=Self> {
        generate(self, |history| {
            (!history.0.iter().all(|&x| x == 0))
                .then(|| history.difference())
        })
    }

    fn next(self) -> i32 {
        self.differences()
            .map(|history| *history.0.last().unwrap())
            .sum()
    }

    fn prev(mut self) -> i32 {
        self.0.reverse();
        self.next()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let result: i32 = parse_lines(History::parse, input)?
        .into_iter()
        .map(History::next)
        .sum();

    Ok(Box::new(result))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let result: i32 = parse_lines(History::parse, input)?
        .into_iter()
        .map(History::prev)
        .sum();

    Ok(Box::new(result))
}