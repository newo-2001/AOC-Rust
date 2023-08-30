use std::{error::Error, fmt::{Display, Formatter, self}};

#[derive(Debug)]
pub struct NoSolutionError;

impl Display for NoSolutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "The puzzle has no valid solution for the given input")
    }
}

impl Error for NoSolutionError {}