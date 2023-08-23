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

pub trait Single<T> {
    fn single(self) -> Result<T, SingleError>;
}

impl<T, I: Iterator<Item=T>> Single<T> for I {
    fn single(mut self) -> Result<T, SingleError> {
        match self.next() {
            None => Err(SingleError::None),
            Some(v) => match self.next() {
                None => Ok(v),
                Some(_) => Err(SingleError::More)
            }
        }
    }
}