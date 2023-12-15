use aoc_lib::{iteration::ExtraIter, parsing::{TextParser, ParseError}};
use aoc_runner_api::SolverResult;
use arr_macro::arr;
use composing::compose_fn;
use indexmap::IndexMap;
use nom::{sequence::{terminated, separated_pair}, character::complete::{alpha1, char, u8}, Parser};

fn hash(input: &[u8]) -> u8 {
    input.iter().fold(0, |acc, &char| {
        ((acc + u32::from(char)) * 17) % 256
    }).try_into().unwrap()
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_1(input: &str) -> SolverResult {
    let sum: u32 = input.split(',')
        .sum_by(compose_fn!(str::as_bytes => hash => Into::<u32>::into));

    Ok(Box::new(sum))
}

enum Operation<'a> {
    Remove(&'a str),
    Insert {
        label: &'a str,
        focal_length: u8
    }
}

impl<'a> Operation<'a> {
    fn parse(input: &'a str) -> Result<Self, ParseError> {
        Parser::or(
            terminated(alpha1, char('-')).map(Self::Remove),
            separated_pair(alpha1, char('='), u8)
                .map(|(label, focal_length)| Self::Insert { label, focal_length })
        ).run(input)
    }
}

struct Map<'a> {
    buckets: [IndexMap<&'a str, u8>; 256]
}

impl<'a> Map<'a> {
    fn new() -> Self {
        Self { buckets: arr![IndexMap::<&str, u8>::new(); 256] }
    }

    fn insert(&mut self, label: &'a str, focal_length: u8) {
        self.buckets[usize::from(hash(label.as_bytes()))]
            .insert(label, focal_length);
    }

    fn remove(&mut self, label: &'a str) {
        self.buckets[usize::from(hash(label.as_bytes()))]
            .shift_remove(label);
    }
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut map: Map = Map::new();

    let operations = input.split(',').map(Operation::parse);
    for operation in operations {
        match operation? {
            Operation::Remove(label) => map.remove(label),
            Operation::Insert { label, focal_length } => map.insert(label, focal_length)
        }
    }

    let focussing_power_sum: usize = map.buckets
        .into_iter()
        .enumerate()
        .flat_map(|(box_number, lenses)| {
            lenses.into_iter().enumerate().map(move |(index, (_, focal_length))| {
                (box_number + 1) * (index + 1) * usize::from(focal_length)
            })
        }).sum();

    Ok(Box::new(focussing_power_sum))
}