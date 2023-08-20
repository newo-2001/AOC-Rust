use std::{ops::{Add, Sub, Mul, AddAssign, SubAssign}, cmp::{max, min}, hash::Hash};
use num::{Integer, Signed, FromPrimitive, Zero, One};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point2D<T>(T, T)
    where T: Integer;

impl<T: Integer> Add for Point2D<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Integer + Copy> AddAssign for Point2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0 + rhs.0;
        self.1 = self.1 + rhs.1;
    }
}

impl<T: Integer> Sub for Point2D<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Integer + Copy> SubAssign for Point2D<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = self.0 - rhs.0;
        self.1 = self.1 - rhs.1;
    }
}

impl<T: Integer + Copy> Mul<T> for Point2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point2D(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Integer + Copy> Point2D<T> {
    pub fn manhattan_distance(&self, other: &Self) -> T {
        let x = max(self.0, other.0) - min(self.0, other.0);
        let y = max(self.1, other.1) - min(self.1, other.1);
        x + y
    }
}

impl<T: Integer> Point2D<T> {
    pub fn zero() -> Point2D<T> {
        Point2D(Zero::zero(), Zero::zero())
    }

    pub fn one() -> Point2D<T> {
        Point2D(One::one(), One::one())
    }
}

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
        where T: Signed + Integer + FromPrimitive
    {
        let (zero, one) = (Zero::zero(), T::from_i32(1).unwrap());

        match self {
            CardinalDirection::North => Point2D(zero, one),
            CardinalDirection::East => Point2D(one, zero),
            CardinalDirection::South => Point2D(zero, -one),
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