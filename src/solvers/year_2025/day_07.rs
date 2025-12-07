use std::{collections::BTreeSet, iter::once};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use anyhow::{Result, Context};
use yuki::{iterators::Enumerate2D, spatial::Point};

use crate::SolverResult;

fn parse_splitters(input: &str) -> BTreeSet<Point<i32>> {
    input
        .lines()
        .map(|line| line.chars())
        .enumerate2d()
        .filter_map(|(pos, c)|
            (c == '^').then(|| pos.cast::<i32>().unwrap())
        )
        .collect()
}

fn find_start(input: &str) -> Result<Point<i32>> {
    input
        .lines()
        .map(|line| line.chars())
        .enumerate2d()
        .find_map(|(pos, c)|
            (c == 'S').then(|| pos.cast::<i32>().unwrap())
        )
        .context("Input does not contain a starting position")
}

fn split_beam(pos: Point<i32>) -> [Point<i32>; 2]{
    [ Point::new(-1, 0), Point::new(1, 0) ]
        .map(move |offset| pos + offset)
}

fn count_splits(splitters: &BTreeSet<Point<i32>>, start: Point<i32>) -> u32 {
    let mut beams: BTreeSet<Point<i32>> = once(start).collect();
    let mut seen = HashSet::<Point<i32>>::new();
    let mut num_splits: u32 = 0;

    let Some(&last_splitter) = splitters.last() else { return num_splits; };

    while let Some(mut beam) = beams.pop_first() {
        while beam <= last_splitter {
            if !seen.insert(beam) { break; }

            if splitters.contains(&beam) {
                beams.extend(split_beam(beam));
                num_splits += 1;

                break;
            }

            beam.y += 1;
        }
    }

    num_splits
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let splitters = parse_splitters(input);
    let start = find_start(input)?;
    let splits = count_splits(&splitters, start);

    Ok(Box::new(splits))
}

fn count_timelines(splitters: &BTreeSet<Point<i32>>, start: Point<i32>) -> u64 {
    let mut beams: Vec<Point<i32>> = once(start).collect();
    let mut cache = HashMap::<Point<i32>, u64>::new();

    let Some(&last_splitter) = splitters.last() else { return 1; };

    while let Some(origin) = beams.pop() {
        let mut beam = origin;

        loop {
            if beam > last_splitter {
                cache.insert(origin, 1);
                break;
            } else if splitters.contains(&beam) {
                let splits = split_beam(beam);
                let timelines: Option<u64> = splits
                    .iter()
                    .map(|split| cache.get(split))
                    .sum();

                match timelines {
                    Some(timelines) => {
                        cache.insert(origin, timelines);
                    },
                    None => {
                        beams.extend(once(origin).chain(splits));
                    }
                };

                break;
            }

            beam.y += 1;
        }
    }

    cache[&start]
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let splitters = parse_splitters(input);
    let start = find_start(input)?;
    let timelines = count_timelines(&splitters, start);

    Ok(Box::new(timelines))
}