use aoc_lib::parsing::{lines, Map3, Parsable, TextParserResult, TextParser};
use itertools::Itertools;
use nom::{character::complete::{char, u32}, sequence::{tuple, preceded}, Parser};

use crate::SolverResult;

struct Present(u32, u32, u32);

impl Parsable<'_> for Present {
    fn parse(input: &str) -> TextParserResult<Present> {
        tuple((
            u32,
            preceded(char('x'), u32),
            preceded(char('x'), u32)
        )).map3(Present)
            .parse(input)
    }
}

impl IntoIterator for &Present {
    type Item = u32;
    type IntoIter = core::array::IntoIter<u32, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.0, self.1, self.2].into_iter()
    }
}

impl Present {
    fn required_wrapping_paper(&self) -> u32 {
        let areas: Vec<u32> = self.into_iter()
            .combinations(2)
            .map(|sides| sides.into_iter().product())
            .collect();

        let min_area: &u32 = areas.iter().min().unwrap_or(&0);
        let total_area = areas.iter().sum::<u32>() * 2;
        min_area + total_area
    }

    fn required_ribbon(&self) -> u32 {
        let smallest_perimeter: u32 = self.into_iter()
            .combinations(2)
            .map(|sides| sides.into_iter().sum::<u32>() * 2)
            .min().unwrap_or(0);

        let volume: u32 = self.into_iter().product();
        
        smallest_perimeter + volume
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let wrapping_paper: u32 = lines(Present::parse).run(input)?
        .iter()
        .map(Present::required_wrapping_paper)
        .sum();

    Ok(Box::new(wrapping_paper))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let ribbon: u32 = lines(Present::parse).run(input)?
        .iter()
        .map(Present::required_ribbon)
        .sum();

    Ok(Box::new(ribbon))
}