use thiserror::Error;

#[derive(Debug, Error)]
#[error("The puzzle has no valid solution for the given input")]
pub struct NoSolution;

#[derive(Debug, Error)]
#[error("Multiple valid solutions possible, the answer is ambiguous")]
pub struct MultipleSolutions;

#[derive(Debug, Error)]
#[error("The puzzle input is empty")]
pub struct NoInput;
