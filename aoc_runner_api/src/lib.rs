use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Puzzle {
    pub year: u16,
    pub day: u8,
    pub part: u8
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} day {:02} part {}", self.year, self.day, self.part)
    }
}

pub type SolverResult<'a> = Result<Box<dyn Display>, Box<dyn Error + 'a>>;
pub type Solver = dyn Fn(&str) -> SolverResult;
pub type SolverProvider = dyn Fn(&Puzzle) -> Option<Box<Solver>>;