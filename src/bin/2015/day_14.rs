use std::{error::Error, fs, cmp::min, collections::HashMap};

use itertools::Itertools;
use nom::{sequence::{tuple, preceded}, bytes::complete::tag, character::complete::{self, alpha1}, Parser, error::VerboseError};

struct Reindeer<'a> {
    name: &'a str,
    speed: u32,
    stamina: u32,
    recovery: u32
}

impl Reindeer<'_> {
    fn parse(input: &str) -> Result<Reindeer, String> {
        let speed = preceded(tag(" can fly "), complete::u32);
        let stamina = preceded(tag(" km/s for "), complete::u32);
        let recovery = preceded(tag(" seconds, but then must rest for "), complete::u32);

        Ok(tuple((alpha1, speed, stamina, recovery))
            .map(|(name, speed, stamina, recovery)| Reindeer { name, speed, stamina, recovery })
            .parse(input).map_err(|err: nom::Err<VerboseError<&str>>| err.to_string())?.1)
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

fn most_points_after_seconds(reindeers: &Vec<Reindeer>, duration: u32) -> u32 {
    let mut points: HashMap<&str, u32> = HashMap::from_iter(
        reindeers.iter().map(|reindeer| (reindeer.name, 0)));
    
    for i in 1..=duration {
        let winners = reindeers.iter()
            .max_set_by_key(|reindeer| reindeer.distance_after_seconds(i));

        for winner in winners {
            *points.get_mut(winner.name).expect("Winner did not participate") += 1
        }
    }

    *points.values().max().expect("There are no participants")
}

fn main() -> Result<(), Box<dyn Error>> {
    const DURATION: u32 = 2503;
    
    let content = fs::read_to_string("inputs/2015/day_14.txt")?;
    let reindeers: Vec<Reindeer> = content.lines()
        .map(Reindeer::parse)
        .collect::<Result<Vec<Reindeer>, String>>()?;

    let winning_distance = reindeers.iter()
        .map(|reindeer| reindeer.distance_after_seconds(DURATION))
        .max().ok_or("Nobody won due to not having a participant")?;

    println!("The winner flew {} km", winning_distance);

    let most_points = most_points_after_seconds(&reindeers, DURATION);
    println!("The winner had {} points", most_points);

    Ok(())
}