use std::{error::Error, env};

use aoc_runner_api::{Puzzle, SolverResult};
use run::{RunnerAction, run_puzzle, verify_puzzle};

mod arguments;
mod run;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    let action = arguments::parse(arguments)?;

    match action {
        RunnerAction::Run(puzzles) => execute(run_puzzle, &puzzles),
        RunnerAction::Verify(puzzles) => execute(verify_puzzle, &puzzles)
    }

    Ok(())
}

fn execute<'a>(runner: fn(&Puzzle) -> SolverResult, puzzles: &Vec<Puzzle>) {
    println!("Executing {} puzzle(s)...", puzzles.len());

    let results = puzzles.into_iter().map(|puzzle| {
        let (year, day, part) = (puzzle.year, puzzle.day, puzzle.part);
        let result = runner(puzzle);
        match &result {
            Ok(answer) => println!("[{} day {:02} part {}] (Success) {}", year, day, part, answer),
            Err(err) => println!("[{} day {:02} part {}] (Failed) {}", year, day, part, err.to_string())
        }

        result
    });

    let passed = results.filter(Result::is_ok).count();
    println!("{} / {} puzzles executed successfully", passed, puzzles.len());
}