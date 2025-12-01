use std::fmt::{Display, Formatter, self, Debug};

use itertools::Itertools;
use nom::{sequence::preceded, character::complete::char, Parser};
use thiserror::Error;

use crate::parsing::{TextParserResult, usize, Parsable};

use super::Point2D;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Dimensions(pub usize, pub usize);

impl Dimensions {
    #[must_use]
    pub const fn width(&self) -> usize { self.0 }

    #[must_use]
    pub const fn height(&self) -> usize { self.1 }
    
    #[must_use]
    pub const fn surface_area(&self) -> usize {
        self.width() * self.height()
    }   
}

impl Parsable<'_> for Dimensions {
    fn parse(input: &str) -> TextParserResult<'_, Self> {
        usize.and(preceded(char('x'), usize))
            .map(|(width, height)| Self(width, height))
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

impl From<Point2D<usize>> for Dimensions {
    fn from(value: Point2D<usize>) -> Self {
        Self(value.x() + 1, value.y() + 1)
    }
}

impl From<usize> for Dimensions {
    fn from(value: usize) -> Self {
        Self(value, value)
    }
}

#[derive(Debug, Error)]
#[error("Can't compute dimensions for non-rectangular data")]
pub struct NotRectangularError;

impl<T> TryInto<Dimensions> for &Vec<Vec<T>> {
    type Error = NotRectangularError;

    fn try_into(self) -> Result<Dimensions, Self::Error> {
        if !self.iter().map(Vec::len).all_equal() {
            return Err(NotRectangularError)
        }
        
        let columns = self.first().map(Vec::len).unwrap_or_default();
        Ok(Dimensions(columns, self.len()))
    }
}

#[derive(Debug, Error)]
#[error("Data did not have the expected dimensions, expected: {expected}")]
pub struct WrongDimensionsError {
    pub expected: Dimensions
}