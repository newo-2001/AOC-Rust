use nom::{branch::alt, combinator::value, character::complete::one_of};
use num::{Integer, Signed, Zero, One};

use crate::parsing::TextParserResult;

use super::Point2D;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RotationDirection {
    Left,
    Right
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West
}

impl CardinalDirection {
    pub fn direction_vector<T>(self) -> Point2D<T>
        where T: Signed + Integer
    {
        let (zero, one): (T, T) = (Zero::zero(), One::one());

        match self {
            Self::North => Point2D(zero, -one),
            Self::East => Point2D(one, zero),
            Self::South => Point2D(zero, one),
            Self::West => Point2D(-one, zero)
        }
    }

    pub fn rotate(self, rotation_direction: RotationDirection) -> CardinalDirection {
        match (self, rotation_direction) {
            (Self::North, RotationDirection::Left) => Self::West,
            (Self::North, RotationDirection::Right) => Self::East,
            (Self::East, RotationDirection::Left) => Self::North,
            (Self::East, RotationDirection::Right) => Self::South,
            (Self::South, RotationDirection::Left) => Self::East,
            (Self::South, RotationDirection::Right) => Self::West,
            (Self::West, RotationDirection::Left) => Self::South,
            (Self::West, RotationDirection::Right) => Self::North
        }
    }

    pub fn parse(input: &str) -> TextParserResult<CardinalDirection> {
        alt((
            value(Self::North, one_of("UuNn^")),
            value(Self::East, one_of("RrEe>")),
            value(Self::South, one_of("DdSsVv")),
            value(Self::West, one_of("LlWw<"))
        ))(input)
    }

    pub fn relative_char(self) -> char {
        match self {
            Self::North => 'U',
            Self::East => 'R',
            Self::South => 'D',
            Self::West => 'L'
        }
    }

    pub fn absolute_char(self) -> char {
        match self {
            Self::North => 'N',
            Self::East => 'E',
            Self::South => 'S',
            Self::West => 'W'
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]

pub enum OrdinalDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest
}

impl OrdinalDirection {
    pub fn direction_vector<T>(self) -> Point2D<T>
        where T: Integer + Signed
    {
        use CardinalDirection::*;

        match self {
            Self::NorthEast => North.direction_vector() + East.direction_vector(),
            Self::NorthWest => North.direction_vector() + West.direction_vector(),
            Self::SouthEast => South.direction_vector() + East.direction_vector(),
            Self::SouthWest => South.direction_vector() + West.direction_vector()
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Cardinal(CardinalDirection),
    Ordinal(OrdinalDirection)
}

impl Direction {
    pub fn direction_vector<T>(self) -> Point2D<T>
        where T: Integer + Signed
    {
        match self {
            Self::Cardinal(direction) => direction.direction_vector(),
            Self::Ordinal(direction) => direction.direction_vector()
        }
    }
}