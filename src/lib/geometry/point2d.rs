use derive_more;
use std::{ops::{Add, Sub}, fmt::{Display, Formatter, self}, cmp::minmax};

use nom::{character::complete::{char, space0}, sequence::separated_pair, Parser};
use num::{clamp, Zero, One};

use crate::parsing::{TextParserResult, Parsable, parens, Map2};

use super::{Dimensions, Directional, CardinalDirection};

#[derive(
    Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord,
    derive_more::Neg,
    derive_more::Add, derive_more::AddAssign,
    derive_more::Sub, derive_more::SubAssign,
    derive_more::Mul, derive_more::MulAssign,
    derive_more::Div, derive_more::DivAssign
)]
pub struct Point2D<T = i32>(pub T, pub T);

impl<T: Copy> Copy for Point2D<T> {}

impl<T> Point2D<T> {
    /// The point located at ``(0, 0)`` (The origin)
    #[must_use]
    pub fn zero() -> Self where T: Zero { Self(Zero::zero(), Zero::zero()) }

    /// The point located at ``(1, 1)``
    #[must_use]
    pub fn one() -> Self where T: One { Self(One::one(), One::one()) }

    pub fn x(self) -> T { self.0 }
    pub fn y(self) -> T { self.1 }

    /// Perform a termwise checked addition with another point.
    /// Returns `None` if any component overflows.
    pub fn checked_add<U>(self, rhs: Point2D<U>) -> Option<Self>
        where U: TryInto<T> + Add<Output=U>,
              T: TryInto<U>
    {
        let add = |a: T, b: U| (a.try_into().ok()? + b).try_into().ok();

        let x = add(self.0, rhs.0)?;
        let y = add(self.1, rhs.1)?;
        Some(Self(x, y))
    }

    /// The distance between two points when using the
    /// [manhattan (or taxicab) distance function](https://en.wikipedia.org/wiki/Taxicab_geometry).
    pub fn manhattan_distance(self, Self(x2, y2): Self) -> T
        where T: Ord + Sub<Output=T> + Add<Output=T>
    {
        let Self(x, y) = self;
        let [min_x, max_x] = minmax(x, x2);
        let [min_y, max_y] = minmax(y, y2);

        max_x - min_x + max_y - min_y
    }

    // Clamps the individual components of the point
    #[must_use]
    pub fn clamp(self, min: T, max: T) -> Self
        where T: PartialOrd + Clone
    {
        let Self(x, y) = self;
        let x = clamp(x, min.clone(), max.clone());
        let y = clamp(y, min, max);
        Self(x, y)
    }

    /// Computes the neigbours of this point in the given directions
    /// If this calculation would overflow `T`, the neighbours are not included in the list.
    pub fn neighbours<U, D>(self, directions: impl IntoIterator<Item=D>) -> impl Iterator<Item=Self>
        where T: Clone + TryFrom<U>,
              U: TryFrom<T> + Add<Output=U>,
              D: Directional<Point2D<U>>
    {
        directions.into_iter()
            .map(Directional::direction_vector)
            .filter_map(move |direction| self.clone().checked_add(direction))
    }

    /// Calculate the relative [`CardinalDirection`] from one point to another
    /// Returns `None` if the points are the same or do not lie on the same line
    pub fn direction_to(self, Self(x2, y2): Self) -> Option<CardinalDirection>
        where T: Ord
    {
        let Self(x, y) = self;
        if x == x2 && y < y2 { Some(CardinalDirection::South) }
        else if x == x2 && y > y2 { Some(CardinalDirection::North) }
        else if x < x2 && y == y2 { Some(CardinalDirection::East) }
        else if x > x2 && y == y2 { Some(CardinalDirection::West) }
        else { None }
    }

    /// Attempts to map a `Point2D<T>` to a `Point2D<U>`.
    pub fn try_map<U>(self) -> Result<Point2D<U>, T::Error> where
        T: TryInto<U>
    {
        Ok(Point2D(self.0.try_into()?, self.1.try_into()?))
    }
}

impl<'a, T: Parsable<'a>> Parsable<'a> for Point2D<T> {
    /// Parses a [`Point2D`] from a string in the form ``x, y`` or ``(x, y)``, the space is optional.
    fn parse(input: &'a str) -> TextParserResult<'a, Self>
    {
        let point = || separated_pair(
            T::parse,
            Parser::and(
                char(','),
                space0
            ),
            T::parse
        );
        
        parens(point()).or(point())
            .map2(Point2D)
            .parse(input)
    }
}

impl<T> From<(T, T)> for Point2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Self(x, y)
    }
}

impl<T: From<usize>> From<Dimensions> for Point2D<T> {
    fn from(Dimensions(width, height): Dimensions) -> Self {
        Self(width.into(), height.into())
    }
}

impl<T: Display> Display for Point2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(x, y) = self;
        write!(f, "({x}, {y})")
    }
}