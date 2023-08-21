use std::{error::Error, env};

use aoc_runner_api::Puzzle;
use run::{RunnerAction, run_puzzle};

mod arguments;
mod run;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    let action = arguments::parse(arguments)?;

    match action {
        RunnerAction::Run(puzzles) => run_puzzles(&puzzles)
    }

    Ok(())
}

fn run_puzzles<'a>(puzzles: impl IntoIterator<Item=&'a Puzzle>) {
    for &puzzle in puzzles {
        let (year, day, part) = (puzzle.year, puzzle.day, puzzle.part);
        match run_puzzle(&puzzle) {
            Ok(answer) => println!("[{} day {} part {}] (Success) {}", year, day, part, answer),
            Err(err) => println!("[{} day {} part {}] (Failed) {}", year, day, part, err.to_string())
        }
    }
}