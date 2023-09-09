use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul}, cmp::{max, min}, fmt::{Display, Formatter, self}};

use nom::{character::complete, combinator::opt, sequence::{terminated, delimited}, Parser};
use num::{Integer, clamp, Zero, One, CheckedAdd, NumCast, Signed, FromPrimitive};

use crate::parsing::TextParserResult;

use super::{Direction, CardinalDirection, OrdinalDirection, Dimensions};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point2D<T = i32>(pub T, pub T) where T: Integer;

impl<T: Integer + Copy> Copy for Point2D<T> {}

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
    pub fn checked_add<U>(self, rhs: Point2D<U>) -> Option<Point2D<T>>
        where U: Integer + NumCast + Signed,
              T: NumCast
    {
        let add = |a: T, b: U| T::from(U::from(a).unwrap() + b);

        let x = add(self.0, rhs.0)?;
        let y = add(self.1, rhs.1)?;
        Some(Point2D(x, y))
    }

    pub fn manhattan_distance(&self, other: &Self) -> T {
        let x = max(self.0, other.0) - min(self.0, other.0);
        let y = max(self.1, other.1) - min(self.1, other.1);
        x + y
    }

    pub fn clamp(&self, min: T, max: T) -> Point2D<T> {
        let x = clamp(self.0, min, max);
        let y = clamp(self.1, min, max);
        Point2D(x, y)
    }

    pub fn direct_neighbours(self) -> impl Iterator<Item=Point2D<T>>
        where T: CheckedAdd + NumCast
    {
        [
            Direction::Cardinal(CardinalDirection::North),
            Direction::Cardinal(CardinalDirection::East),
            Direction::Cardinal(CardinalDirection::South),
            Direction::Cardinal(CardinalDirection::West),
        ].into_iter()
            .filter_map(move |direction| self.checked_add(direction.direction_vector::<i64>()))
    }

    pub fn neighbours(self) -> impl Iterator<Item=Point2D<T>>
        where T: CheckedAdd + NumCast
    {
        [
            Direction::Ordinal(OrdinalDirection::NorthEast),
            Direction::Ordinal(OrdinalDirection::SouthEast),
            Direction::Ordinal(OrdinalDirection::SouthWest),
            Direction::Ordinal(OrdinalDirection::NorthWest)
        ].into_iter()
            .filter_map(move |direction| self.checked_add(direction.direction_vector::<i64>()))
            .chain(self.direct_neighbours())
    }
}

impl<T: Integer> Point2D<T> {
    pub fn zero() -> Point2D<T> {
        Point2D(Zero::zero(), Zero::zero())
    }

    pub fn one() -> Point2D<T> {
        Point2D(One::one(), One::one())
    }

    pub fn x(self) -> T {
        self.0
    }

    pub fn y(self) -> T {
        self.1
    }

    pub fn parse(input: &str) -> TextParserResult<Point2D<T>>
        where T: FromPrimitive
    {
        let sep = complete::char(',').and(opt(complete::char(' ')));
        let point = terminated(complete::i64, sep).and(complete::i64);
        
        delimited(opt(complete::char('(')), point, opt(complete::char(')')))
            .map(|(x, y)| Point2D(T::from_i64(x).unwrap(), T::from_i64(y).unwrap()))
            .parse(input)
    }
}

impl<T: Integer> From<(T, T)> for Point2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Point2D(x, y)
    }
}

impl<T: Integer + From<usize>> From<Dimensions> for Point2D<T> {
    fn from(Dimensions(width, height): Dimensions) -> Self {
        Point2D(width.into(), height.into())
    }
}

impl<T: Integer + Copy> PartialOrd for Point2D<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let x = self.x().partial_cmp(&other.x());
        let y = self.y().partial_cmp(&other.y());

        match (x, y) {
            (Some(x), Some(y)) if x == y => Some(x),
            _ => None
        }
    }
}

impl<T: Integer + Display + Copy> Display for Point2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x(), self.y())
    }
}