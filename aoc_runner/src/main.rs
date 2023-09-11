#![allow(
    clippy::module_name_repetitions,
)]

use std::{env, num::ParseIntError, time::Instant};

use aoc_runner_api::Puzzle;
use colored::{Colorize, Color};
use run::{RunnerAction, run_puzzle, verify_puzzle, RunPuzzleError, RunStats};

mod arguments;
mod run;
mod utils;

fn main() -> Result<(), ParseIntError> {
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

fn execute(runner: fn(Puzzle) -> Result<RunStats, RunPuzzleError>, puzzles: &[Puzzle]) {
    println!("Executing {} puzzle(s)...", puzzles.len());

    let start_time = Instant::now();
    let results = puzzles.iter().map(|&puzzle| {
        let result = runner(puzzle);
        let puzzle = format!("[{}]", puzzle).bright_yellow().bold();
        match &result {
            Err(err) => println!("{} {} {}", puzzle, "(Failure)".bright_red().bold(), err.to_string().bright_red()),
            Ok(RunStats { result, duration }) => {

                let status = "(Success)".bright_green().bold();
                println!("{} {} {} {}", puzzle, utils::format_duration(duration), status, result);
            }
        }

        result
    });

    let passed = results.filter(Result::is_ok).count();
    let end_time = Instant::now();
    let duration = end_time - start_time;

    let msg = format!("{} {} / {} puzzle(s) executed successfully", utils::format_duration(&duration), passed, puzzles.len())
        .color(if passed == puzzles.len() { Color::BrightGreen } else { Color::BrightRed })
        .bold();

    println!("{msg}");
}