use std::{fmt::{Display, Formatter, self}, ops::{Add, Sub, Mul, Div}, cmp::minmax};
use derive_more;

use nom::{sequence::terminated, character::complete::{char, space0}, Parser};
use num::{Zero, One};

use crate::parsing::{Parsable, Map3, parens};

use super::Directional;

#[derive(
    Debug, Clone, Default, Eq, PartialEq, Hash, PartialOrd, Ord,
    derive_more::Neg,
    derive_more::Add, derive_more::AddAssign,
    derive_more::Sub, derive_more::SubAssign,
    derive_more::Mul, derive_more::MulAssign,
    derive_more::Div, derive_more::DivAssign
)]
pub struct Point3D<T = i32>(pub T, pub T, pub T);

impl<T> Point3D<T> {
    pub fn x(self) -> T { self.0 }
    pub fn y(self) -> T { self.1 }
    pub fn z(self) -> T { self.2 }

    /// The point located at ``(0, 0, 0)`` (The origin)
    #[must_use]
    pub fn zero() -> Self where T: Zero { Self(Zero::zero(), Zero::zero(), Zero::zero()) }

    /// The point located at ``(1, 1, 1)``
    #[must_use]
    pub fn one() -> Self where T: One { Self(One::one(), One::one(), One::one()) }

    /// The distance between two points when using the
    /// [manhattan (or taxicab) distance function](https://en.wikipedia.org/wiki/Taxicab_geometry).
    pub fn manhattan_distance(self, Self(x2, y2, z2): Self) -> T where
        T: Ord + Sub<Output=T> + Add<Output=T>
    {
        let Self(x, y, z) = self;
        let [min_x, max_x] = minmax(x, x2);
        let [min_y, max_y] = minmax(y, y2);
        let [min_z, max_z] = minmax(z, z2);

        max_x - min_x + max_y - min_y + max_z - min_z
    }

    /// The distance of this point to another
    /// if these points were to represent coordinates on a 2D hex grid.
    /// For more information on these coordinates,
    /// check out [this](https://www.redblobgames.com/grids/hexagons/#coordinates-cube) article on hexagonal cube grids.
    pub fn hex_distance(self, other: Self) -> T where
        T: One + Ord + Sub<Output=T> + Add<Output=T> + Div<Output=T>
    {
        let two = T::one() + T::one();
        self.manhattan_distance(other) / two
    }

    /// Computes the direct neigbours of this point when taking a step in every [`Direction3D`].
    /// If this calculation would overflow `T`, the neighbours are not included in the list.
    pub fn neighbours<U, D>(self, directions: impl IntoIterator<Item=D>) -> impl Iterator<Item=Self> where
        T: Clone + TryFrom<U>,
        U: Add<Output = U> + TryFrom<T>,
        D: Directional<Point3D<U>>
    {
        directions.into_iter()
            .map(Directional::direction_vector)
            .filter_map(move |direction| self.clone().checked_add(direction))
    }

    /// Perform a termwise checked addition with another point.
    /// Returns `None` if any component overflows.
    pub fn checked_add<U>(self, Point3D(x2, y2, z2): Point3D<U>) -> Option<Self> where
        U: TryFrom<T> + Add<Output=U>,
        T: TryFrom<U>
    {
        let add = |a: T, b: U| T::try_from(U::try_from(a).ok()? + b).ok();
        let Self(x, y, z) = self;

        let x = add(x, x2)?;
        let y = add(y, y2)?;
        let z = add(z, z2)?;
        Some(Self(x, y, z))
    }

    pub fn dot(self, other: Self) -> T where
        T: Mul<Output=T> + Add<Output=T>
    {
        let Self(x1, y1, z1) = self;
        let Self(x2, y2, z2) = other;

        x1 * x2 + y1 * y2 + z1 * z2
    }

    pub fn magnitude(self) -> T where
        T: Zero + Ord + Sub<Output=T>
    {
        self.manhattan_distance(Self::zero())
    }

    #[must_use]
    pub fn normalized(self) -> Self where
        T: Zero + Ord + Copy + Sub<Output=T>,
        Self: Div<T, Output=Self>
    {
        self / self.magnitude()
    }
}

impl<T: Copy> Copy for Point3D<T> {}

impl<T: Display> Display for Point3D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(x, y, z) = self;
        write!(f, "({x}, {y}, {z})")
    }
}

impl<T> From<(T, T, T)> for Point3D<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Self(x, y, z)
    }
}

impl<'a, T: Parsable<'a>> Parsable<'a> for Point3D<T> {
    fn parse(input: &'a str) -> crate::parsing::TextParserResult<'a, Self> {
        let seperator = || terminated(char(','), space0);
        let point = || (
            terminated(T::parse, seperator()),
            terminated(T::parse ,seperator()),
            T::parse
        ).map3(Point3D);

        parens(point()).or(point()).parse(input)
    }
}