use std::{cmp::min, collections::HashMap};

use aoc_lib::{parsing::{parse_lines, Runnable, ParseError}, NoSolutionError};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{sequence::{tuple, preceded}, bytes::complete::tag, character::complete::{self, alpha1}, Parser};

struct Reindeer<'a> {
    name: &'a str,
    speed: u32,
    stamina: u32,
    recovery: u32
}

impl Reindeer<'_> {
    fn parse(input: &str) -> Result<Reindeer, ParseError> {
        let speed = preceded(tag(" can fly "), complete::u32);
        let stamina = preceded(tag(" km/s for "), complete::u32);
        let recovery = preceded(tag(" seconds, but then must rest for "), complete::u32);

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

fn most_points_after_seconds(reindeers: &Vec<Reindeer>, duration: u32) -> Result<u32, NoSolutionError> {
    let mut points: HashMap<&str, u32> = HashMap::from_iter(
        reindeers.iter().map(|reindeer| (reindeer.name, 0)));
    
    for i in 1..=duration {
        let winners = reindeers.iter()
            .max_set_by_key(|reindeer| reindeer.distance_after_seconds(i));

        for winner in winners {
            *points.get_mut(winner.name).expect("Winner did not participate") += 1
        }
    }

    Ok(*points.values().max().ok_or(NoSolutionError)?)
}

const DURATION: u32 = 2503;

pub fn solve_part_1(input: &str) -> SolverResult {
    let reindeers = parse_lines(Reindeer::parse, input)?;
    let winning_distance = reindeers.iter()
        .map(|reindeer| reindeer.distance_after_seconds(DURATION))
        .max().ok_or(NoSolutionError)?;

    Ok(Box::new(winning_distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let reindeers = parse_lines(Reindeer::parse, input)?;
    let most_points = most_points_after_seconds(&reindeers, DURATION)?;

    Ok(Box::new(most_points))
}