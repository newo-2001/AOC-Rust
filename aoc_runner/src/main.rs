use std::{error::Error, env};

use aoc_runner_api::{Puzzle, SolverResult};
use colored::{Colorize, Color};
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
        let puzzle = format!("[{} day {:02} part {}]", year, day, part).bright_yellow().bold();
        match &result {
            Ok(answer) => println!("{} {} {}", puzzle, "(Success)".bright_green().bold(), answer),
            Err(err) => println!("{} {} {}", puzzle, "(Failure)".bright_red().bold(), err.to_string().bright_red())
        }

        result
    });

    let passed = results.filter(Result::is_ok).count();
    let msg = format!("{} / {} puzzles executed successfully", passed, puzzles.len())
        .color(if passed == puzzles.len() { Color::BrightGreen } else { Color::BrightRed })
        .bold();

    println!("{}", msg);
}