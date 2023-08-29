use num::{Integer, Signed, Zero, One};

use super::Point2D;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RotationDirection {
    Left,
    Right
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]

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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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