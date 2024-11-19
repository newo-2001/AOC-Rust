use std::{cmp::minmax, iter::Step, fmt::{Display, Formatter, self}, ops::{Sub, Add}, hash::Hash};

use ahash::HashSet;
use num::{Zero, One};

use super::{Point2D, Dimensions};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Area<T> {
    pub(crate) top_left: Point2D<T>,
    pub(crate) bottom_right: Point2D<T>
}

impl<T: Copy> Copy for Area<T> {}

impl<T> Area<T> {
    pub fn from_corners(Point2D(x, y): Point2D<T>, Point2D(x2, y2): Point2D<T>) -> Area<T>
        where T: Ord
    {
        let [min_x, max_x] = minmax(x, x2);
        let [min_y, max_y] = minmax(y, y2);
        let top_left = Point2D(min_x, min_y);
        let bottom_right = Point2D(max_x, max_y);
        Area { top_left, bottom_right }
    }
    
    pub fn from_dimensions_at(top_left: Point2D<T>, dimensions: Dimensions) -> Area<T>
        where T: From<usize> + Add<Output=T> + Clone
    {
        Area {
            top_left: top_left.clone(),
            bottom_right: top_left + dimensions.into()
        }
    }

    pub fn dimensions(self) -> Dimensions
        where T: Into<usize> + Sub<Output=T> + Add<Output = T> + One
    {
        let Point2D(width, height) = self.bottom_right - self.top_left + Point2D::one();
        Dimensions(width.into(), height.into())
    }

    pub fn top(self) -> T { self.top_left.y() }
    pub fn left(self) -> T { self.top_left.x() }
    pub fn bottom(self) -> T { self.bottom_right.y() }
    pub fn right(self) -> T { self.bottom_right.x() }

    pub fn top_left(self) -> Point2D<T> { self.top_left }
    pub fn bottom_right(self) -> Point2D<T> { self.bottom_right }

    pub fn top_right(self) -> Point2D<T>
        where T: Zero
    {
        self.top_left + Point2D(self.bottom_right.x(), Zero::zero())
    }

    pub fn bottom_left(self) -> Point2D<T>
        where T: Add<Output=T> + Zero
    {
        self.top_left + Point2D(Zero::zero(), self.bottom_right.y())
    }

    pub fn corners(self) -> [Point2D<T>; 4]
        where T: Zero + Add<Output = T> + Clone
    {
        let top_left = self.top_left.clone();
        let bottom_right = self.bottom_right.clone();
        let bottom_left = self.clone().bottom_left();
        let top_right = self.top_right();

        [top_left, top_right, bottom_left, bottom_right]
    }

    /// Produces an iterator containing all the [`Point2D`]s on the edges of the area
    pub fn edges(self) -> impl Iterator<Item=Point2D<T>>
        where T: Step + Copy,
              Point2D<T>: Hash + Eq
    {
        (self.left()..=self.right()).map(move |x| {
            [Point2D(x, self.top()), Point2D(x, self.bottom())]
        }).chain((self.top()..=self.bottom()).map(move |y| {
            [Point2D(self.left(), y), Point2D(self.right(), y)]
        })).flatten()
            .collect::<HashSet<_>>()
            .into_iter()
    }

    pub fn contains(&self, Point2D(x, y): &Point2D<T>) -> bool
        where T: Ord
    {
        let Point2D(left, top) = &self.top_left;
        let Point2D(right, bottom) = &self.bottom_right;
        x >= left && x <= right &&
        y >= top && y <= bottom
    }
}

impl<T: Step + Clone> IntoIterator for Area<T> {
    type IntoIter = impl Iterator<Item=Point2D<T>>;
    type Item = Point2D<T>;

    fn into_iter(self) -> Self::IntoIter {
        let Point2D(left, top) = self.top_left;
        let Point2D(right, bottom) = self.bottom_right;

        (top..=bottom).flat_map(move |y| {
            (left.clone()..=right.clone())
                .map(move |x| Point2D(x, y.clone()))
        })
    }
}

impl<T> From<Dimensions> for Area<T>
    where T: From<usize> + Zero + One + Sub<Output=T>
{
    fn from(Dimensions(width, height): Dimensions) -> Self {
        let bottom_right: Point2D<T> = Point2D(width.into(), height.into()) - Point2D::one();
        Area { top_left: Point2D::zero(), bottom_right }
    }
}

impl<T: Display> Display for Area<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} to {}", self.top_left, self.bottom_right)
    }
}