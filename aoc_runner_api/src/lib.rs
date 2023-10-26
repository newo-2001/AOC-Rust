use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PuzzlePart {
    First,
    Second
}

impl Display for PuzzlePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number())
    }
}

impl PuzzlePart {
    #[must_use]
    pub fn number(self) -> u8 {
        match self {
            Self::First => 1,
            Self::Second => 2
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Puzzle {
    pub year: u16,
    pub day: u8,
    pub part: PuzzlePart
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} day {:02} part {}", self.year, self.day, self.part)
    }
}

pub type SolverResult<'a> = Result<Box<dyn Display + Send + Sync + 'a>, Box<dyn Error + 'a + Send + Sync>>;
pub type Solver = dyn Fn(&str) -> SolverResult;
pub type SolverProvider = dyn Fn(&Puzzle) -> Option<Box<Solver>>;