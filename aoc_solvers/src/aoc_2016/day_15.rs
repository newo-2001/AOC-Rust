use aoc_lib::{parsing::{ParseError, TextParser, parse_lines, usize}, errors::NoSolution};
use aoc_runner_api::SolverResult;
use nom::{bytes::complete::tag, sequence::{delimited, terminated, tuple}, character::complete::char, Parser};
use num::Integer;

struct Disc {
    positions: usize,
    starting_position: usize,
    index: usize
}

impl Disc {
    fn parse(input: &str) -> Result<Disc, ParseError> {
        let disc_number = delimited(tag("Disc #"), usize, tag(" has "));
        let positions = terminated(usize, tag(" positions; at time="));
        let time = terminated(usize, tag(", it is at position "));
        let starting_position = delimited(time, usize, char('.'));

        tuple((disc_number, positions, starting_position))
            .map(|(index, positions, starting_position)| Disc { positions, starting_position, index })
            .run(input)
    }

    fn is_aligned_at(&self, time: usize) -> bool {
        (self.starting_position + self.index + time).is_multiple_of(&self.positions)
    }
}

struct Machine(Vec<Disc>);

impl Machine {
    fn parse(input: &str) -> Result<Machine, ParseError> {
        Ok(Machine(parse_lines(Disc::parse, input)?))
    }

    fn first_aligned_time(&self) -> Option<usize> {
        let upper_bound = self.0.iter().map(|disc| disc.positions).product();
        (0..upper_bound).find(|&time| self.0.iter().all(|disc| disc.is_aligned_at(time)))
    }

    fn add_disc(&mut self, disc: Disc) { self.0.push(disc) }
    fn total_discs(&self) -> usize { self.0.len() }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let machine = Machine::parse(input)?;
    let time = machine.first_aligned_time().ok_or(NoSolution)?;
    Ok(Box::new(time))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut machine = Machine::parse(input)?;
    machine.add_disc(Disc { index: machine.total_discs() + 1, positions: 11, starting_position: 0 });
    
    let time = machine.first_aligned_time().ok_or(NoSolution)?;
    Ok(Box::new(time))
}