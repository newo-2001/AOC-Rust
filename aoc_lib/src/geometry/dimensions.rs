use std::fmt::{Display, Formatter, self, Debug};

use nom::{sequence::preceded, character::complete, Parser};
use thiserror::Error;

use crate::parsing::{TextParserResult, self};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Dimensions(pub usize, pub usize);

impl Dimensions {
    #[must_use]
    pub fn width(&self) -> usize { self.0 }

    #[must_use]
    pub fn height(&self) -> usize { self.1 }
    
    #[must_use]
    pub fn surface_area(&self) -> usize {
        self.width() * self.height()
    }
    
    pub fn parse(input: &str) -> TextParserResult<Dimensions> {
        let size = parsing::usize;
        size.and(preceded(complete::char('x'), size))
            .map(|(width, height)| Dimensions(width, height))
            .parse(input)
    }
}

impl PartialOrd for Dimensions {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let width = self.width().partial_cmp(&other.width());
        let height = self.height().partial_cmp(&other.height());
        match (width, height) {
            (Some(width), Some(height)) if width == height => Some(width),
            _ => None
        }
    }
}

impl Display for Dimensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(width, height) = self;
        write!(f, "{width}x{height}")
    }
}

#[derive(Debug, Error)]
#[error("Data did not have the expected dimensions, expected: {expected}")]
pub struct WrongDimensionsError {
    pub expected: Dimensions
}