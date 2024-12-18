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
    anonymous_lifetime_in_impl_trait,
    unsigned_is_multiple_of,
    let_chains,
    iter_map_windows
)]

use std::fs::File;

use ahash::HashMap;
use jikan::{DayManifest, ManifestProvider, Puzzle};

mod solvers;

type SolverResult = jikan::SolverResult<anyhow::Error>;
type Solver = jikan::Solver<anyhow::Error>;

macro_rules! solver {
    ($year: literal, $day: literal, $part: literal) => {
        vec![(
            Puzzle { year: $year, day: $day.parse().unwrap(), part: $part },
            paste::expr! { solvers::[<year_ $year>]::[<day_ $day>]::[<solve_part_ $part>] } as Solver
        )]
    };

    ($year: literal, $day: literal) => {
        [
            solver!($year, $day, 1),
            solver!($year, $day, 2)
        ].into_iter()
            .flatten()
            .collect::<Vec<_>>()
    };

    ($year: literal) => {
        [
            solver!($year, "01"),
            solver!($year, "02"),
            solver!($year, "03"),
            solver!($year, "04"),
            solver!($year, "05"),
            solver!($year, "06"),
            solver!($year, "07"),
            solver!($year, "08"),
            solver!($year, "09"),
            solver!($year, "10"),
            solver!($year, "11"),
            solver!($year, "12"),
            solver!($year, "13"),
            solver!($year, "14"),
            solver!($year, "15"),
            solver!($year, "16"),
            solver!($year, "17"),
            solver!($year, "18"),
            solver!($year, "19"),
            solver!($year, "20"),
            solver!($year, "21"),
            solver!($year, "22"),
            solver!($year, "23"),
            solver!($year, "24"),
        ].into_iter()
            .flatten()
            .chain(solver!($year, "25", 1))
            .collect::<Vec<_>>()
    }
}

struct Manifests;
impl ManifestProvider for Manifests {
    fn get_manifest(day: jikan::Day) -> Result<DayManifest, Box<dyn std::error::Error>> {
        let path = format!("data/{}/day_{:02}.yaml", day.year, day.day);
        let file = File::open(&path)?;
        let manifest = serde_yml::from_reader(file)?;
        Ok(manifest)
    }
}

fn main() {
    let options = jikan::ExecutionOptions::from_args();
    let solvers: HashMap<Puzzle, Solver> = [
        solver!(2015),
        solver!(2016),

        solver!(2017, "01"),
        solver!(2017, "02"),
        solver!(2017, "03"),
        solver!(2017, "04"),
        solver!(2017, "05"),
        solver!(2017, "06"),
        solver!(2017, "07"),
        solver!(2017, "08"),
        solver!(2017, "09"),
        solver!(2017, "10"),
        solver!(2017, "11"),
        solver!(2017, "12"),
        solver!(2017, "13"),
        solver!(2017, "14"),
        solver!(2017, "15"),
        solver!(2017, "16"),
        solver!(2017, "17"),
        solver!(2017, "18"),
        solver!(2017, "19"),
        solver!(2017, "20"),

        solver!(2023, "01"),
        solver!(2023, "02"),
        solver!(2023, "03"),
        solver!(2023, "04"),
        solver!(2023, "05"),
        solver!(2023, "06"),
        solver!(2023, "07"),
        solver!(2023, "08"),
        solver!(2023, "09"),
        solver!(2023, "10"),
        solver!(2023, "11"),
        solver!(2023, "12"),
        solver!(2023, "13"),
        solver!(2023, "14"),
        solver!(2023, "15"),
        solver!(2023, "16"),
        solver!(2023, "17"),
        solver!(2023, "18"),
        solver!(2023, "19"),
        solver!(2023, "20"),
        solver!(2023, "21", 1),
        solver!(2023, "22"),
        solver!(2023, "23"),
        solver!(2023, "24", 1),
        solver!(2023, "25", 1),

        solver!(2024, "01"),
        solver!(2024, "02"),
        solver!(2024, "03"),
        solver!(2024, "04"),
        solver!(2024, "05"),
        solver!(2024, "06"),
        solver!(2024, "07"),
        solver!(2024, "08"),
        solver!(2024, "09"),
        solver!(2024, "10"),
        solver!(2024, "11"),
        solver!(2024, "12"),
        solver!(2024, "13"),
        solver!(2024, "14"),
        solver!(2024, "15"),
        solver!(2024, "16"),
        solver!(2024, "17"),
        solver!(2024, "18")
    ].into_iter()
        .flatten()
        .collect();

    jikan::execute::<Manifests, _, _>(options, &solvers);
}