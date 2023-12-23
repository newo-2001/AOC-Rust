#![feature(
    get_many_mut,
    extract_if,
    iter_collect_into,
    try_find,
    map_try_insert,
    trait_alias,
    iter_next_chunk,
    iter_array_chunks,
    never_type,
    cmp_minmax,
    iter_intersperse,
    anonymous_lifetime_in_impl_trait
)]

#![allow(
    clippy::similar_names,
    clippy::into_iter_without_iter
)]

use aoc_runner_api::{Solver, Puzzle, PuzzlePart};

mod aoc_2016;
mod aoc_2015;
mod aoc_2017;
mod aoc_2023;

// TODO: replace with procedural macro
macro_rules! solver {
    ($year:literal, $day: literal, $part: literal) => {
        paste::expr! {
            Some(Box::new([<aoc_ $year>]::[<day_ $day>]::[<solve_part_ $part>]))
        }
    };

    ($year:literal, $day: literal, $part: expr) => {
        match ($part) {
            PuzzlePart::First => solver!($year, $day, 1),
            PuzzlePart::Second => solver!($year, $day, 2),
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

pub fn get_solver(puzzle: Puzzle) -> Option<Box<Solver>> {
    match puzzle.year {
        2015 => solver!(2015, puzzle.day, puzzle.part),
        2016 => solver!(2016, puzzle.day, puzzle.part),
        2017 => match puzzle.day {
            1  => solver!(2017, "01", puzzle.part),
            2  => solver!(2017, "02", puzzle.part),
            3  => solver!(2017, "03", puzzle.part),
            4  => solver!(2017, "04", puzzle.part),
            5  => solver!(2017, "05", puzzle.part),
            6  => solver!(2017, "06", puzzle.part),
            7  => solver!(2017, "07", puzzle.part),
            8  => solver!(2017, "08", puzzle.part),
            9  => solver!(2017, "09", puzzle.part),
            10 => solver!(2017, "10", puzzle.part),
            11 => solver!(2017, "11", puzzle.part),
            12 => solver!(2017, "12", puzzle.part),
            13 => solver!(2017, "13", puzzle.part),
            14 => solver!(2017, "14", puzzle.part),
            15 => solver!(2017, "15", puzzle.part),
            16 => solver!(2017, "16", puzzle.part),
            17 => solver!(2017, "17", puzzle.part),
            18 => solver!(2017, "18", puzzle.part),
            19 => solver!(2017, "19", puzzle.part),
            20 => solver!(2017, "20", puzzle.part),
            _ => None
        },
        2023 => match puzzle.day {
            1  => solver!(2023, "01", puzzle.part),
            2  => solver!(2023, "02", puzzle.part),
            3  => solver!(2023, "03", puzzle.part),
            4  => solver!(2023, "04", puzzle.part),
            5  => solver!(2023, "05", puzzle.part),
            6  => solver!(2023, "06", puzzle.part),
            7  => solver!(2023, "07", puzzle.part),
            8  => solver!(2023, "08", puzzle.part),
            9  => solver!(2023, "09", puzzle.part),
            10 => solver!(2023, "10", puzzle.part),
            11 => solver!(2023, "11", puzzle.part),
            12 => solver!(2023, "12", puzzle.part),
            13 => solver!(2023, "13", puzzle.part),
            14 => solver!(2023, "14", puzzle.part),
            15 => solver!(2023, "15", puzzle.part),
            16 => solver!(2023, "16", puzzle.part),
            17 => solver!(2023, "17", puzzle.part),
            18 => solver!(2023, "18", puzzle.part),
            19 => solver!(2023, "19", puzzle.part),
            20 => solver!(2023, "20", puzzle.part),
            21 if puzzle.part == PuzzlePart::First => solver!(2023, "21", 1),
            22 => solver!(2023, "22", puzzle.part),
            23 => solver!(2023, "23", puzzle.part),
            _  => None
        }
        _ => None
    }
}