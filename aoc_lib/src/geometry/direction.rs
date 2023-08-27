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
            CardinalDirection::North => Point2D(zero, -one),
            CardinalDirection::East => Point2D(one, zero),
            CardinalDirection::South => Point2D(zero, one),
            CardinalDirection::West => Point2D(-one, zero)
        }
    }

    pub fn rotate(self, rotation_direction: RotationDirection) -> CardinalDirection {
        match (self, rotation_direction) {
            (CardinalDirection::North, RotationDirection::Left) => CardinalDirection::West,
            (CardinalDirection::North, RotationDirection::Right) => CardinalDirection::East,
            (CardinalDirection::East, RotationDirection::Left) => CardinalDirection::North,
            (CardinalDirection::East, RotationDirection::Right) => CardinalDirection::South,
            (CardinalDirection::South, RotationDirection::Left) => CardinalDirection::East,
            (CardinalDirection::South, RotationDirection::Right) => CardinalDirection::West,
            (CardinalDirection::West, RotationDirection::Left) => CardinalDirection::South,
            (CardinalDirection::West, RotationDirection::Right) => CardinalDirection::North
        }
    }
}