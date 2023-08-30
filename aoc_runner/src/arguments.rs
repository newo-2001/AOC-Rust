use std::{error::Error, num::ParseIntError};

use aoc_runner_api::Puzzle;

use crate::RunnerAction;

fn available_puzzles() -> impl Iterator<Item=Puzzle> {
    (2015..=2022).flat_map(move |year| {
        (1..=25).flat_map(move |day| {
            (1..=(if day == 25 { 1 } else { 2 })).map(move |part| {
                Puzzle { year, day, part }
            })
        })
    })
}

fn locate_puzzles(year: Option<u16>) -> impl Iterator<Item=Puzzle> {
    available_puzzles()
        .filter(move |puzzle| {
            aoc_solvers::get_solver(puzzle).is_some() &&
            year.map_or(true, |year| puzzle.year == year)
        })
}

pub fn parse_puzzles<'a>(mut arguments: impl Iterator<Item=String>) -> Result<impl Iterator<Item=Puzzle>, Box<dyn Error>> {
    let puzzles: Box<dyn Iterator<Item=Puzzle>> = if let Some(year) = arguments.next() {
        let year = year.parse::<u16>()?;
        if let Some(day) = arguments.next() {
            let day = day.parse::<u8>()?;
            let parts = arguments.next()
                .map(|part| vec![part.parse::<u8>()])
                .unwrap_or_else(|| if day == 25 { vec![Ok(1u8)] } else { vec![Ok(1u8), Ok(2u8)] })
                .into_iter().collect::<Result<Vec<u8>, ParseIntError>>()?;

            Box::new(parts.into_iter().map(move |part| Puzzle { year, day, part }))
        } else {
            Box::new(locate_puzzles(Some(year)))
        }
    } else {
        Box::new(locate_puzzles(None))
    };

    Ok(puzzles)
}

pub fn parse(arguments: Vec<String>) -> Result<RunnerAction, Box<dyn Error>> {
    let action = arguments.first().map(|arg| arg.as_str());
    let action = match action {
        Some("verify") => {
            let puzzles = parse_puzzles(arguments.into_iter().skip(1))?;
            RunnerAction::Verify(puzzles.collect())
        },
        _ => {
            let puzzles = parse_puzzles(arguments.into_iter())?;
            RunnerAction::Run(puzzles.collect())
        }
    };

    Ok(action)
}