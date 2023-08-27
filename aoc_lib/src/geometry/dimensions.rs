use std::{error::Error, fmt::{Display, Formatter, self, Debug}};

use num::Integer;

use super::Point2D;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Dimensions(pub usize, pub usize);

impl Dimensions {
    pub fn width(&self) -> usize { self.0 }
    pub fn height(&self) -> usize { self.1 }
    
    pub fn surface_area(&self) -> usize {
        self.width() * self.height()
    }
}

impl<T: Integer + From<usize>> Into<Point2D<T>> for Dimensions {
    fn into(self) -> Point2D<T> {
        let Self(x, y) = self;
        Point2D(x.into(), y.into())
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
        write!(f, "{}x{}", width, height)
    }
}

#[derive(Debug)]
pub struct WrongDimensionsError {
    pub expected: Dimensions
}

impl Display for WrongDimensionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Data did not have the expected dimensions, expected: {}", self.expected)
    }
}

impl Error for WrongDimensionsError {}