use indexmap::IndexSet;
use itertools::Itertools;
use num::Signed;
use std::{hash::Hash, ops::{Add, Sub, Div, Mul}, fmt::Debug};

use crate::iteration::ExtraIter;

use super::{Point2D, Area, Orientation};

#[derive(Debug, Clone)]
pub struct Polygon<T> {
    points: IndexSet<Point2D<T>, ahash::RandomState>
}

impl<T> Polygon<T> {
    #[must_use]
    pub fn new() -> Self { Self { points: IndexSet::default() }}

    pub fn push_point(&mut self, point: Point2D<T>) where
        T: Hash + Eq
    {
        self.points.insert(point);
    }

    #[must_use]
    pub fn iter(&self) -> indexmap::set::Iter<'_, Point2D<T>> { self.into_iter() }

    #[must_use]
    pub fn len(&self) -> usize { self.points.len() }

    #[must_use]
    pub fn is_empty(&self) -> bool { self.points.is_empty() }

    #[must_use]
    /// Constructs the bounding rectangle of the polygon.
    /// This operation is O(N).
    pub fn bounding_rect(&self) -> Option<Area<T>> where
        T: Ord + Clone
    {
        let (min_x, max_x) = self.iter().map(|point| point.clone().x()).minmax().into_option()?;
        let (min_y, max_y) = self.iter().map(|point| point.clone().y()).minmax().into_option()?;
        Some(Area::from_corners(Point2D(min_x, min_y), Point2D(max_x, max_y)))
    }

    #[must_use]
    /// Computes the circumference of the shape using its boundry points.
    /// This operation is O(N)
    pub fn circumference(&self) -> T where
        T: Clone + Default + Add<Output=T> + Sub<Output=T> + Ord
    {
        self.points.iter()
            .cloned()
            .circular_tuple_windows()
            .sum_by(|(a, b)| a.manhattan_distance(b))
    }

    #[must_use]
    /// Computes the area of the polygon including the boundry using the [shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula).
    /// This operation is O(N).
    pub fn shoelace(&self) -> T where
        T: Add<Output=T> + Sub<Output=T> + Div<Output=T> + Mul<Output=T> +
           Clone + Default + From<u8>
    {
        self.points.iter()
            .cloned()
            .circular_tuple_windows()
            .sum_by(|(Point2D(x, y), Point2D(x2, y2))| (x * y2) - (x2 * y)) / T::from(2u8)
    }

    /// Computes the area of the polygon including the boundry using [pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem).
    /// This operation is O(N).
    #[must_use]
    pub fn pick(&self) -> T where
        T: Add<Output=T> + Sub<Output=T> + Div<Output=T> + Mul<Output=T> +
           Clone + Default + Ord + From<u8>
    {
        self.shoelace() + self.circumference() / T::from(2u8) + T::from(1u8)
    }

    /// Determines whether the polygon is oriented clockwise or counter clockwise.
    /// This operation has to compute the area so the operation is O(N).
    #[must_use]
    pub fn orientation(&self) -> Orientation where
        T: Add<Output=T> + Sub<Output=T> + Div<Output=T> + Mul<Output=T> +
           Clone + Default + Ord + From<u8> + Signed
    {
        if self.pick().is_negative() { Orientation::CounterClockwise }
        else { Orientation::Clockwise }
    }

    /// Flip the orientation of the polygon.
    /// If the points were in clockwise order, they will now be counter-clockwise.
    /// This operation is O(N).
    pub fn flip(&mut self)
        where T: Hash + Eq
    {
        self.points.reverse();
    }
}

impl<T> Default for Polygon<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hash + Eq> FromIterator<Point2D<T>> for Polygon<T> {
    fn from_iter<I: IntoIterator<Item = Point2D<T>>>(iter: I) -> Self {
        Self { points: iter.into_iter().collect() }
    }
}

impl<T: Hash + Eq> Extend<Point2D<T>> for Polygon<T> {
    fn extend<I: IntoIterator<Item=Point2D<T>>>(&mut self, iter: I) {
        self.points.extend(iter);
    }
}

impl<T: Hash + Eq> IntoIterator for Polygon<T> {
    type Item = Point2D<T>;
    type IntoIter = indexmap::set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Polygon<T> {
    type Item = &'a Point2D<T>;
    type IntoIter = indexmap::set::Iter<'a, Point2D<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.points.iter()
    }
}