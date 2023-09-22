use derive_more::{Add, AddAssign, Sub, SubAssign};
use std::{ops::Mul, cmp::{max, min}, fmt::{Display, Formatter, self}};

use nom::{character::complete::{char, i64}, combinator::opt, sequence::{terminated, delimited}, Parser};
use num::{Integer, clamp, Zero, One, NumCast, Signed, FromPrimitive};

use crate::parsing::TextParserResult;

use super::{Direction2D, CardinalDirection, OrdinalDirection, Dimensions};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Add, AddAssign, Sub, SubAssign)]
pub struct Point2D<T = i32>(pub T, pub T) where T: Integer;

impl<T: Integer + Copy> Copy for Point2D<T> {}

impl<T: Integer + Copy> Mul<T> for Point2D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Point2D(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Integer + Copy> Point2D<T> {
    /// Perform a termwise checked addition with another point.
    /// Returns `None` if any component overflows.
    pub fn checked_add<U>(self, rhs: Point2D<U>) -> Option<Point2D<T>>
        where U: Integer + NumCast + Signed,
              T: NumCast
    {
        let add = |a: T, b: U| T::from(U::from(a).unwrap() + b);

        let x = add(self.0, rhs.0)?;
        let y = add(self.1, rhs.1)?;
        Some(Point2D(x, y))
    }

    /// The distance between two points when using the
    /// [manhattan (or taxicab) distance function](https://en.wikipedia.org/wiki/Taxicab_geometry).
    pub fn manhattan_distance(&self, other: &Self) -> T {
        let x = max(self.0, other.0) - min(self.0, other.0);
        let y = max(self.1, other.1) - min(self.1, other.1);
        x + y
    }

    // Clamps the individual components of the point
    #[must_use]
    pub fn clamp(&self, min: T, max: T) -> Self {
        let x = clamp(self.0, min, max);
        let y = clamp(self.1, min, max);
        Point2D(x, y)
    }

    /// Computes the direct neigbours of this point when taking a step in every [`CardinalDirection`].
    /// If this calculation would overflow `T`, the neighbours are not included in the list.
    pub fn direct_neighbours(self) -> impl Iterator<Item=Point2D<T>>
        where T: NumCast
    {
        [
            Direction2D::Cardinal(CardinalDirection::North),
            Direction2D::Cardinal(CardinalDirection::East),
            Direction2D::Cardinal(CardinalDirection::South),
            Direction2D::Cardinal(CardinalDirection::West),
        ].into_iter()
            .filter_map(move |direction| self.checked_add(direction.direction_vector::<i64>()))
    }

    /// Computes the neigbours of this point when taking a step in every [`Direction2D`].
    /// If this calculation would overflow `T`, the neighbours are not included in the list.
    pub fn neighbours(self) -> impl Iterator<Item=Point2D<T>>
        where T: NumCast
    {
        [
            Direction2D::Ordinal(OrdinalDirection::NorthEast),
            Direction2D::Ordinal(OrdinalDirection::SouthEast),
            Direction2D::Ordinal(OrdinalDirection::SouthWest),
            Direction2D::Ordinal(OrdinalDirection::NorthWest)
        ].into_iter()
            .filter_map(move |direction| self.checked_add(direction.direction_vector::<i64>()))
            .chain(self.direct_neighbours())
    }
}

impl<T: Integer> Point2D<T> {
    /// The point located at ``(0, 0)`` (The origin)
    #[must_use]
    pub fn zero() -> Self {
        Point2D(Zero::zero(), Zero::zero())
    }

    /// The point located at ``(1, 1)``
    #[must_use]
    pub fn one() -> Self {
        Point2D(One::one(), One::one())
    }

    pub fn x(self) -> T { self.0 }

    pub fn y(self) -> T { self.1 }

    /// Parses a [`Point2D`] from a string in the form ``x, y`` or ``(x, y)``, the space is optional.
    pub fn parse(input: &str) -> TextParserResult<Point2D<T>>
        where T: FromPrimitive
    {
        let sep = char(',').and(opt(char(' ')));
        let point = terminated(i64, sep).and(i64);
        
        delimited(opt(char('(')), point, opt(char(')')))
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