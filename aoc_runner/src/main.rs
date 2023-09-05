use std::{env, num::ParseIntError};

use aoc_runner_api::Puzzle;
use colored::{Colorize, Color};
use run::{RunnerAction, run_puzzle, verify_puzzle, RunPuzzleError, RunStats};

mod arguments;
mod run;

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

fn execute<'a>(runner: fn(&Puzzle) -> Result<RunStats, RunPuzzleError>, puzzles: &Vec<Puzzle>) {
    println!("Executing {} puzzle(s)...", puzzles.len());

    let results = puzzles.into_iter().map(|puzzle| {
        let result = runner(puzzle);
        let puzzle = format!("[{}]", puzzle).bright_yellow().bold();
        match &result {
            Err(err) => println!("{} {} {}", puzzle, "(Failure)".bright_red().bold(), err.to_string().bright_red()),
            Ok(RunStats { result, duration }) => {
                let mins = (duration.as_secs_f32() / 60f32) as u32;
                let secs = duration.as_secs() % 60;
                let millis = duration.as_millis() % 1000;
                let duration = format!("[{:02}:{:02}.{:03}]", mins, secs, millis)
                    .bright_blue().bold();

                let status = "(Success)".bright_green().bold();
                println!("{} {} {} {}", puzzle, duration, status, result)
            }
        }

        result
    });

    let passed = results.filter(Result::is_ok).count();
    let msg = format!("{} / {} puzzles executed successfully", passed, puzzles.len())
        .color(if passed == puzzles.len() { Color::BrightGreen } else { Color::BrightRed })
        .bold();

    println!("{}", msg);
}