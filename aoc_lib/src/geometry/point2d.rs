use derive_more;
use std::{ops::{Add, Sub}, fmt::{Display, Formatter, self}, cmp::{minmax, Ordering}};

use nom::{character::complete::{char, i64}, combinator::{opt, map_res}, sequence::separated_pair, Parser};
use num::{clamp, Zero, One};

use crate::parsing::{TextParserResult, Parsable, parens};

use super::{Dimensions, Directional, CardinalDirection};

#[derive(
    Clone, Debug, Default, PartialEq, Eq, Hash,
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
    pub fn zero() -> Self where T: Zero { Point2D(Zero::zero(), Zero::zero()) }

    /// The point located at ``(1, 1)``
    #[must_use]
    pub fn one() -> Self where T: One { Point2D(One::one(), One::one()) }

    pub fn x(self) -> T { self.0 }
    pub fn y(self) -> T { self.1 }

    /// Perform a termwise checked addition with another point.
    /// Returns `None` if any component overflows.
    pub fn checked_add<U>(self, rhs: Point2D<U>) -> Option<Point2D<T>>
        where U: TryInto<T> + Add<Output=U>,
              T: TryInto<U>
    {
        let add = |a: T, b: U| (a.try_into().ok()? + b).try_into().ok();

        let x = add(self.0, rhs.0)?;
        let y = add(self.1, rhs.1)?;
        Some(Point2D(x, y))
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
        Point2D(x, y)
    }

    /// Computes the neigbours of this point in the given directions
    /// If this calculation would overflow `T`, the neighbours are not included in the list.
    pub fn neighbours<U, D>(self, directions: impl IntoIterator<Item=D>) -> impl Iterator<Item=Point2D<T>>
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
}

impl<T: TryFrom<i64>> Parsable<'_> for Point2D<T> {
    /// Parses a [`Point2D`] from a string in the form ``x, y`` or ``(x, y)``, the space is optional.
    fn parse(input: &str) -> TextParserResult<Point2D<T>>
    {
        let point = || separated_pair(
            i64,
            Parser::and(
                char(','),
                opt(char(' '))
            ),
            i64
        );
        
        map_res(
            parens(point()).or(point()),
            |(x, y)| Ok::<_, T::Error>(Point2D(T::try_from(x)?, T::try_from(y)?))
        ).parse(input)
    }
}

impl<T> From<(T, T)> for Point2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Point2D(x, y)
    }
}

impl<T: From<usize>> From<Dimensions> for Point2D<T> {
    fn from(Dimensions(width, height): Dimensions) -> Self {
        Point2D(width.into(), height.into())
    }
}

impl<T: PartialOrd> PartialOrd for Point2D<T> {
    fn partial_cmp(&self, Self(x2, y2): &Self) -> Option<Ordering> {
        let Self(x, y) = self;
        let x = x.partial_cmp(x2);
        let y = y.partial_cmp(y2);

        match (x, y) {
            (Some(x), Some(y)) if x == y => Some(x),
            _ => None
        }
    }
}

impl<T: Display> Display for Point2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(x, y) = self;
        write!(f, "({x}, {y})")
    }
}