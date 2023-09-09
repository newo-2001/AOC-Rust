use std::{cmp::{min, max}, iter::Step, fmt::{Display, Formatter, self}};

use num::{Integer, Zero, ToPrimitive};

use super::{Point2D, Dimensions};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Area<T> where T: Integer {
    top_left: Point2D<T>,
    bottom_right: Point2D<T>
}

impl<T: Integer + Copy> Copy for Area<T> {}

impl<T> Area<T> where T: Integer + Copy {
    pub fn from_corners(first: Point2D<T>, second: Point2D<T>) -> Area<T> {
        let top_left = Point2D(min(first.x(), second.x()), min(first.y(), second.y()));
        let bottom_right = Point2D(max(first.x(), second.x()), max(second.x(), second.y()));
        Area { top_left, bottom_right }
    }
    
    pub fn from_dimensions_at(top_left: Point2D<T>, dimensions: Dimensions) -> Area<T> where T: From<usize> {
        Area {
            top_left,
            bottom_right: top_left + dimensions.into()
        }
    }

    pub fn dimensions(&self) -> Dimensions where T: ToPrimitive {
        let Point2D(width, height) = self.bottom_right() - self.top_left();
        Dimensions(T::to_usize(&width).unwrap() + 1, T::to_usize(&height).unwrap() + 1)
    }

    pub fn top(&self) -> T { self.top_left.y() }
    pub fn left(&self) -> T { self.top_left.x() }
    pub fn bottom(&self) -> T { self.bottom_right.y() }
    pub fn right(&self) -> T { self.bottom_right.x() }

    pub fn top_left(&self) -> Point2D<T> { self.top_left }
    pub fn bottom_right(&self) -> Point2D<T> { self.bottom_right }

    pub fn top_right(&self) -> Point2D<T> {
        self.top_left + Point2D(self.bottom_right.x(), Zero::zero())
    }

    pub fn bottom_left(&self) -> Point2D<T> {
        self.top_left + Point2D(Zero::zero(), self.bottom_right.y())
    }

    pub fn corners(&self) -> [Point2D<T>; 4] {
        [self.top_left(), self.top_right(), self.bottom_left(), self.bottom_right()]
    }

    pub fn iter(self) -> impl Iterator<Item=Point2D<T>> where T: Step {
        (self.top()..=self.bottom()).flat_map(move |y| {
            (self.left()..=self.right()).map(move |x| Point2D(x, y))
        })
    }

    pub fn contains(&self, Point2D(x, y): Point2D<T>) -> bool {
        x >= self.left() && x <= self.right() &&
        y >= self.top() && y <= self.bottom()
    }
}

impl<T> From<Dimensions> for Area<T> where T: Integer + Copy + From<usize> {
    fn from(Dimensions(width, height): Dimensions) -> Self {
        let bottom_right: Point2D<T> = Point2D(width.into(), height.into()) - Point2D::one();
        Area { top_left: Point2D::zero(), bottom_right }
    }
}

impl<T> Display for Area<T> where T: Integer + Display + Copy {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}", self.top_left, self.bottom_right)
    }
}