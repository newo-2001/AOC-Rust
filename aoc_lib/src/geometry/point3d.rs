use std::{fmt::{Display, Formatter, self}, ops::Mul};
use derive_more::{Add, AddAssign, Sub, SubAssign};

use num::{Integer, Zero, One, NumCast, Signed};

use crate::math::min_max;

use super::Direction3D;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, Add, AddAssign, Sub, SubAssign)]
pub struct Point3D<T: Integer = i32>(pub T, pub T, pub T);

impl<T: Integer> Point3D<T> {
    pub fn x(&self) -> &T { &self.0 }
    pub fn y(&self) -> &T { &self.1 }
    pub fn z(&self) -> &T { &self.2 }

    /// The point located at ``(0, 0, 0)`` (The origin)
    #[must_use]
    pub fn zero() -> Self { Point3D(Zero::zero(), Zero::zero(), Zero::zero()) }

    /// The point located at ``(1, 1, 1)``
    #[must_use]
    pub fn one() -> Self { Point3D(One::one(), One::one(), One::one()) }

    /// The distance between two points when using the
    /// [manhattan (or taxicab) distance function](https://en.wikipedia.org/wiki/Taxicab_geometry).
    pub fn manhattan_distance(self, Self(x2, y2, z2): Self) -> T {
        let Self(x, y, z) = self;
        let (min_x, max_x) = min_max(x, x2);
        let (min_y, max_y) = min_max(y, y2);
        let (min_z, max_z) = min_max(z, z2);

        max_x - min_x + max_y - min_y + max_z - min_z
    }

    /// The distance of this point to another
    /// if these points were to represent coordinates on a 2D hex grid.
    /// For more information on these coordinates,
    /// check out [this](https://www.redblobgames.com/grids/hexagons/#coordinates-cube) article on hexagonal cube grids.
    pub fn hex_distance(self, other: Self) -> T {
        let two = T::one() + T::one();
        self.manhattan_distance(other) / two
    }

    /// Computes the direct neigbours of this point when taking a step in every [`Direction3D`].
    /// If this calculation would overflow `T`, the neighbours are not included in the list.
    pub fn direct_neighbours(&self) -> impl Iterator<Item=Point3D<T>> + '_
        where T: NumCast + Clone
    {
        [
            Direction3D::North,
            Direction3D::East,
            Direction3D::South,
            Direction3D::West,
            Direction3D::Up,
            Direction3D::Down
        ].into_iter().map(Direction3D::direction_vector::<i64>)
            .filter_map(|direction| self.clone().checked_add(direction))
    }

    /// Perform a termwise checked addition with another point.
    /// Returns `None` if any component overflows.
    pub fn checked_add<U>(self, Point3D(x2, y2, z2): Point3D<U>) -> Option<Point3D<T>>
        where U: Integer + NumCast + Signed,
              T: NumCast
    {
        let add = |a: T, b: U| T::from(U::from(a).unwrap() + b);
        let Point3D(x, y, z) = self;

        let x = add(x, x2)?;
        let y = add(y, y2)?;
        let z = add(z, z2)?;
        Some(Point3D(x, y, z))
    }

    pub fn dot(self: Point3D<T>, other: Point3D<T>) -> T {
        let Point3D(x1, y1, z1) = self;
        let Point3D(x2, y2, z2) = other;

        x1 * x2 + y1 * y2 + z1 * z2
    }
}

impl<T: Integer + Copy> Copy for Point3D<T> {}

impl<T: Integer + Display> Display for Point3D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl<T: Integer> From<(T, T, T)> for Point3D<T> {
    fn from((x, y, z): (T, T, T)) -> Self {
        Point3D(x, y, z)
    }
}

impl<T: Integer + Copy> Mul<T> for Point3D<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let Point3D(x, y, z) = self;
        Point3D(x * rhs, y * rhs, z * rhs)
    }
}