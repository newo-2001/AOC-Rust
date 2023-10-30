use std::fmt::{Debug, Display};

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Encountered invalid token: `{0}`")]
pub struct InvalidTokenError<T>(pub T) where T: Display + Debug;

#[derive(Error, Debug)]
#[error("Parsing failed: {0}")]
pub struct ParseError(String);

impl ParseError {
    #[must_use]
    pub fn new(error: String) -> Self {
        Self(error)
    }
}