use std::{ops::{Add, AddAssign, Sub, SubAssign, Mul}, cmp::{max, min}, fmt::{Display, Formatter, self}};

use num::{Integer, clamp, Zero, One};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point2D<T>(pub T, pub T) where T: Integer;

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
}

impl<T: Integer> From<(T, T)> for Point2D<T> {
    fn from((x, y): (T, T)) -> Self {
        Point2D(x, y)
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