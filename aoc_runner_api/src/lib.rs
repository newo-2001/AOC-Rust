use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Puzzle {
    pub year: u16,
    pub day: u8,
    pub part: u8
}

pub type SolverResult = Result<Box<dyn Display>, Box<dyn Error>>;
pub type Solver = dyn Fn(&str) -> SolverResult;
pub type SolverProvider = dyn Fn(&Puzzle) -> Option<Box<Solver>>;