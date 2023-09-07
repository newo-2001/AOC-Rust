#![feature(get_many_mut, extract_if, result_option_inspect, iter_collect_into)]

use aoc_2015::*;
use aoc_2016::*;
use aoc_runner_api::{Solver, Puzzle};

mod aoc_2016;
mod aoc_2015;

// TODO: replace with procedural macro
macro_rules! solver {
    ($year:literal, $day: literal, $part: literal) => {
        paste::expr! {
            Some(Box::new([<aoc_ $year _day_ $day>]::[<solve_part_ $part>]))
        }
    };

    ($year:literal, $day: literal, $part: expr) => {
        match ($part) {
            1 => solver!($year, $day, 1),
            2 => solver!($year, $day, 2),
            _ => None
        }
    };

    ($year:literal, $day: expr, $part: expr) => {
        match ($day) {
            1  => solver!($year, "01", $part),
            2  => solver!($year, "02", $part),
            3  => solver!($year, "03", $part),
            4  => solver!($year, "04", $part),
            5  => solver!($year, "05", $part),
            6  => solver!($year, "06", $part),
            7  => solver!($year, "07", $part),
            8  => solver!($year, "08", $part),
            9  => solver!($year, "09", $part),
            10 => solver!($year, "10", $part),
            11 => solver!($year, "11", $part),
            12 => solver!($year, "12", $part),
            13 => solver!($year, "13", $part),
            14 => solver!($year, "14", $part),
            15 => solver!($year, "15", $part),
            16 => solver!($year, "16", $part),
            17 => solver!($year, "17", $part),
            18 => solver!($year, "18", $part),
            19 => solver!($year, "19", $part),
            20 => solver!($year, "20", $part),
            21 => solver!($year, "21", $part),
            22 => solver!($year, "22", $part),
            23 => solver!($year, "23", $part),
            24 => solver!($year, "24", $part),
            25 => solver!($year, "25", 1),
            _ => None
        }
    }
}

pub fn get_solver(puzzle: &Puzzle) -> Option<Box<Solver>> {
    match puzzle.year {
        2015 => solver!(2015, puzzle.day, puzzle.part),
        2016 => match puzzle.day {
            1  => solver!(2016, "01", puzzle.part),
            2  => solver!(2016, "02", puzzle.part),
            3  => solver!(2016, "03", puzzle.part),
            4  => solver!(2016, "04", puzzle.part),
            5  => solver!(2016, "05", puzzle.part),
            6  => solver!(2016, "06", puzzle.part),
            7  => solver!(2016, "07", puzzle.part),
            8  => solver!(2016, "08", puzzle.part),
            9  => solver!(2016, "09", puzzle.part),
            10 => solver!(2016, "10", puzzle.part),
            11 => solver!(2016, "11", puzzle.part),
            12 => solver!(2016, "12", puzzle.part),
            13 => solver!(2016, "13", puzzle.part),
            14 => solver!(2016, "14", puzzle.part),
            15 => solver!(2016, "15", puzzle.part),
            16 => solver!(2016, "16", puzzle.part),
            _ => None
        },
        _ => None
    }
}