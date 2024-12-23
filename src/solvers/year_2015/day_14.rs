use std::cmp::min;

use ahash::HashMap;
use aoc_lib::parsing::{parse_lines, TextParser, ParseError};
use yuki::errors::NoSolution;
use crate::SolverResult;
use itertools::Itertools;
use nom::{sequence::{tuple, preceded, delimited}, bytes::complete::tag, character::complete::{alpha1, u32}, Parser};

struct Reindeer<'a> {
    name: &'a str,
    speed: u32,
    stamina: u32,
    recovery: u32
}

impl Reindeer<'_> {
    fn parse(input: &str) -> Result<Reindeer, ParseError> {
        let speed = preceded(tag(" can fly "), u32);
        let stamina = preceded(tag(" km/s for "), u32);
        let recovery = delimited(tag(" seconds, but then must rest for "), u32, tag(" seconds."));

        tuple((alpha1, speed, stamina, recovery))
            .map(|(name, speed, stamina, recovery)| Reindeer { name, speed, stamina, recovery })
            .run(input)    
    }

    fn distance_after_seconds(&self, duration: u32) -> u32 {
        let cycle_duration = self.stamina + self.recovery;

        let period = duration % cycle_duration;
        let seconds_flown_partial_cycle = min(period, self.stamina);
        let seconds_flown_completed_cycles = duration / cycle_duration * self.stamina;

        let seconds_flown = seconds_flown_completed_cycles + seconds_flown_partial_cycle;
        seconds_flown * self.speed
    }
}

fn most_points_after_seconds(reindeers: &[Reindeer], duration: u32) -> Result<u32, NoSolution> {
    let mut points: HashMap<&str, u32> = reindeers.iter()
        .map(|reindeer| (reindeer.name, 0))
        .collect();
    
    for i in 1..=duration {
        let winners = reindeers.iter()
            .max_set_by_key(|reindeer| reindeer.distance_after_seconds(i));

        for winner in winners {
            *points.get_mut(winner.name).expect("Winner did not participate") += 1;
        }
    }

    Ok(*points.values().max().ok_or(NoSolution)?)
}

const DURATION: u32 = 2503;

pub fn solve_part_1(input: &str) -> SolverResult {
    let reindeers = parse_lines(Reindeer::parse, input)?;
    let winning_distance = reindeers.iter()
        .map(|reindeer| reindeer.distance_after_seconds(DURATION))
        .max().ok_or(NoSolution)?;

    Ok(Box::new(winning_distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let reindeers = parse_lines(Reindeer::parse, input)?;
    let most_points = most_points_after_seconds(&reindeers, DURATION)?;

    Ok(Box::new(most_points))
}