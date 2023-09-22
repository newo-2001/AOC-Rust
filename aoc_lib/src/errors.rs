use thiserror::Error;

#[derive(Debug, Error)]
#[error("The puzzle has no valid solution for the given input")]
pub struct NoSolutionError;

#[derive(Debug, Error)]
#[error("The puzzle input is empty")]
pub struct NoInputError;
