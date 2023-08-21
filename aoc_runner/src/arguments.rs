use std::{error::Error, num::ParseIntError};

use aoc_runner_api::Puzzle;

use crate::RunnerAction;

pub fn parse_puzzles<'a>(mut arguments: impl Iterator<Item=String>) -> Result<impl Iterator<Item=Puzzle>, Box<dyn Error>> {
    let year = arguments.next().ok_or("Missing 'year' argument ")?.parse::<u16>()?;
    let day = arguments.next().ok_or("Missing 'day' argument")?.parse::<u8>()?;
    let parts = arguments.next()
        .map(|part| vec![part.parse::<u8>()])
        .unwrap_or_else(|| vec![Ok(1u8), Ok(2u8)])
        .into_iter().collect::<Result<Vec<u8>, ParseIntError>>()?;

    let puzzles = parts.into_iter()
        .map(move |part| Puzzle { year, day, part });

    Ok(puzzles)
}

pub fn parse(arguments: Vec<String>) -> Result<RunnerAction, Box<dyn Error>> {
    let action = arguments.first().ok_or("Missing arguments")?;
    let action = match action {
        _ => {
            let puzzles = parse_puzzles(arguments.into_iter())?;
            RunnerAction::Run(puzzles.collect())
        }
    };

    Ok(action)
}