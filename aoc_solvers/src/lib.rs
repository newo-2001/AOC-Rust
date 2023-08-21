use aoc_2016::aoc_2016_day_01;
use aoc_runner_api::{Solver, Puzzle};

mod aoc_2016;

pub fn get_solver(puzzle: &Puzzle) -> Option<Box<Solver>> {
    let solver: Box<Solver> = match (puzzle.year, puzzle.day, puzzle.part) {
        (2016, 1, 1) => Box::from(aoc_2016_day_01::solve_part_1),
        (2016, 1, 2) => Box::from(aoc_2016_day_01::solve_part_2),
        _ => return None
    };

    Some(solver)
}