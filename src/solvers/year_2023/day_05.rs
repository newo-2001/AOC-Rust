use aoc_lib::{math::{Range, InvalidRangeError}, between, parsing::{TextParserResult, ParseError, TextParser, skip_until}};
use crate::SolverResult;
use itertools::Itertools;
use nom::{character::complete::{char, u64, line_ending}, sequence::{tuple, preceded, delimited}, Parser, multi::separated_list1, bytes::complete::tag, combinator::map_res};
use anyhow::{Context, Result};
use rayon::iter::{ParallelBridge, ParallelIterator, IntoParallelIterator};

struct Almanac(Vec<Map>);

impl Almanac {
    fn parse(input: &str) -> Result<(Vec<u64>, Self), ParseError> {
        Parser::and(
            delimited(tag("seeds: "), separated_list1(char(' '), u64), line_ending.and(line_ending)),
            separated_list1(line_ending, Map::parse).map(Self)
        ).run(input)
    }

    fn min_distance(&self, seeds: impl IntoParallelIterator<Item=u64>) -> Result<u64> {
        seeds.into_par_iter().map(|seed| self.0
            .iter()
            .fold(seed, |value, map| map.map(value))
        )
        .min()
        .context("No seeds in input")
    }
}

struct Map(Vec<MappingRange>);

impl Map {
    fn map(&self, n: u64) -> u64 {
        self.0.iter()
            .find_map(|range| range.map(n))
            .unwrap_or(n)
    }

    fn parse(input: &str) -> TextParserResult<Self> {
        preceded(
            skip_until(tag("map:")).and(line_ending),
            separated_list1(line_ending, MappingRange::parse)
        ).map(Self).parse(input)
    }
}

struct MappingRange {
    from: Range<u64>,
    to: Range<u64>
}

impl MappingRange {
    fn map(&self, n: u64) -> Option<u64> {
        self.from.contains(&n).then(|| {
            let offset = n - self.from.start;
            self.to.start + offset
        })
    }

    fn parse(input: &str) -> TextParserResult<Self> {
        map_res(
            tuple((u64, between!(char(' '), u64), u64)),
            |(dest_start, source_start, length)| {
                Result::<_, InvalidRangeError<_>>::Ok(Self {
                    from: Range::exclusive(source_start, source_start + length)?,
                    to: Range::exclusive(dest_start, dest_start + length)?
                })
            }
        ).parse(input)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (seeds, almanac) = Almanac::parse(input)?;

    let min_distance = almanac.min_distance(seeds)?;
    Ok(Box::new(min_distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (seeds, almanac) = Almanac::parse(input)?;
    let seeds = seeds.into_iter()
        .tuples()
        .par_bridge()
        .flat_map(|(start, length)| start..start + length);

    let min_distance = almanac.min_distance(seeds)?;
    Ok(Box::new(min_distance))
}