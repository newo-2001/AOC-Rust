use std::{fmt::{Debug, Display}, error::Error};

#[derive(Debug)]
pub struct InvalidTokenError<T>(pub T) where T: Display + Debug;

impl<T: Display + Debug> Display for InvalidTokenError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Encountered invalid token: {}", self.0)
    }
}

impl<T: Display + Debug> Error for InvalidTokenError<T> {}