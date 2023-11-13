#![allow(
    clippy::module_name_repetitions,
    clippy::similar_names
)]

use std::{env, time::{Instant, Duration}, collections::HashMap};

use aoc_runner_api::Puzzle;
use colored::{Colorize, ColoredString};
use itertools::Itertools;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use run::{RunnerAction, run_puzzle, verify_puzzle, RunStats};

mod arguments;
mod run;
mod utils;

fn main() -> Result<(), arguments::Error> {
    let arguments = env::args()
        .skip(1)
        .collect_vec();

    let arguments = arguments.iter()
        .map(String::as_str)
        .collect_vec();

    let action = RunnerAction::parse(&arguments)?;
    match action {
        RunnerAction::Run(puzzles) => execute(run_puzzle, &puzzles),
        RunnerAction::Verify(puzzles) => execute(verify_puzzle, &puzzles)
    }

    Ok(())
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Status {
    Success,
    Failure,
    NotRun
}

fn execute(runner: fn(Puzzle) -> Result<RunStats, run::Error>, puzzles: &[Puzzle]) {
    println!("Executing {} puzzle(s)...", puzzles.len());

    let start_time = Instant::now();

    let results: Vec<_> = puzzles.par_iter().map(|&puzzle| {
        let result = runner(puzzle);
        print_runner_result(puzzle, &result);
        result
    }).collect();

    let duration = start_time.elapsed();

    let stats = results.into_iter().map(|result| match result {
        Err(run::Error::RunnerError { .. }) => Status::Failure,
        Err(run::Error::ResolutionError(_)) => Status::NotRun,
        Ok(_) => Status::Success
    }).counts();

    print_summary(&stats, duration);
}

fn print_runner_result(puzzle: Puzzle, result: &Result<RunStats, run::Error>) {
    struct DisplayInfo<'a> {
        status: ColoredString,
        output: ColoredString,
        duration: Option<&'a Duration>
    }

    let puzzle = format!("[{puzzle}]").bold().bright_blue();
    let info = match result {
        Err(run::Error::RunnerError { duration, error }) => DisplayInfo {
            status: "(Failure)".bold().bright_red(),
            output: error.to_string().bright_red(),
            duration: Some(duration)
        },
        Err(run::Error::ResolutionError(error)) => DisplayInfo {
            status: "(Not Run)".bold().bright_yellow(),
            output: error.to_string().bright_yellow(),
            duration: None
        },
        Ok(RunStats { duration, result }) => DisplayInfo {
            status: "(Success)".bold().bright_green(),
            output: result.to_string().bright_green(),
            duration: Some(duration)
        }
    };

    match info {
        DisplayInfo { duration: Some(duration), status, output } => {
            println!("{} {} {} {}", puzzle, utils::format_duration(duration), status, output);
        },
        DisplayInfo { duration: None, status, output } => {
            println!("{puzzle} {status} {output}");
        }
    }
}

// TODO: Clean this up
fn print_summary(stats: &HashMap<Status, usize>, duration: Duration) {
    let [succeeded, not_ran, failed] = [Status::Success, Status::NotRun, Status::Failure]
        .map(|status| *stats.get(&status).unwrap_or(&0));

    let total_puzzles: usize = stats.values().sum();
    let puzzles = if total_puzzles == 1 { "puzzle" } else { "puzzles" };
    
    let msg = format!("Execution took {}", utils::format_duration(&duration)).bold().bright_blue();
    println!("\n{msg}");

    if succeeded > 0 {
        let msg = format!("{succeeded} / {total_puzzles} {puzzles} executed successfully").bold().bright_green();
        println!("{msg}");
    }

    if failed > 0 {
        let msg = format!("{failed} / {total_puzzles} {puzzles} failed to execute").bold().bright_red();
        println!("{msg}");
    }

    if not_ran > 0 {
        let msg = format!("{not_ran} / {total_puzzles} {puzzles} were not executed").bold().bright_yellow();
        println!("{msg}");
    }
}