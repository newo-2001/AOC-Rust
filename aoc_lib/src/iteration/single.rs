use std::{fmt::{Formatter, Display}, error::Error};

#[derive(Debug)]
pub enum SingleError {
    None,
    More
}

impl Display for SingleError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Iterator yielded no elements"),
            Self::More => write!(f, "Iterator yielded more than one element")
        }
    }
}

impl Error for SingleError {}

pub fn single<T>(mut it: impl Iterator<Item=T>) -> Result<T, SingleError> {
    match it.next() {
        None => Err(SingleError::None),
        Some(v) => match it.next() {
            None => Ok(v),
            Some(_) => Err(SingleError::More)
        }
    }
}