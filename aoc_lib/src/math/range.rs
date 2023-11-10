// We have `std::iter::range`, What is the purpose of this?
// https://kaylynn.gay/blog/post/rust_ranges_and_suffering

use std::{iter::Step, fmt::Display, cmp::{max, minmax}};

use num::{FromPrimitive, Integer, One};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Range<T> {
    pub start: T,
    pub end: T
}

pub enum RangeDirection {
    Increasing,
    Decreasing
}

impl<T: Step> Range<T> {
    pub fn exclusive(start: T, end: T) -> Range<T> {
        Range { start, end }
    }

    // TODO: inclusive ranges should be represented using a tag in the type system
    // This method is flawed because not the entire domain of `T` fits in the range
    pub fn inclusive(start: T, end: T) -> Range<T> {
        let end = if start <= end { T::forward(end, 1) }
            else { T::backward(end, 1) };

        Range { start, end }
    }

    pub fn direction(&self) -> RangeDirection {
        if self.end >= self.start { RangeDirection::Increasing }
        else { RangeDirection::Decreasing }
    }

    pub fn contains(&self, value: &T) -> bool {
        let (small, big) = match self.direction() {
            RangeDirection::Decreasing => (&self.end, &self.start),
            RangeDirection::Increasing => (&self.start, &self.end)
        };
        
        small <= value && value < big
    }

    pub fn contains_range(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    pub fn interval(self) -> T where T: Integer {
        let [small, large] = minmax(self.start, self.end);
        large - small
    }

    pub fn merge(self, other: Self) -> Result<Self, (Self, Self)> where T: Ord {
        if other.start <= self.end { Ok(Range::exclusive(self.start, max(self.end, other.end))) }
        else { Err((self, other)) }
    }

    pub fn sum(self) -> T where T: Integer + FromPrimitive {
        let amount = self.clone().interval();
        let end = match self.direction() {
            RangeDirection::Increasing => self.end - One::one(),
            RangeDirection::Decreasing => self.end + One::one()
        };

        (self.start + end) * amount / T::from_u8(2).unwrap()
    }
}

impl<T: Copy> Copy for Range<T> {}

impl<T: Display> Display for Range<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}..{})", self.start, self.end)
    }
}

impl<T: Step> From<(T, T)> for Range<T> {
    fn from((start, end): (T, T)) -> Self {
        Range::<T>::exclusive(start, end)
    }
}

impl<T: Step> IntoIterator for Range<T> {
    type Item = T;
    type IntoIter = RangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        RangeIterator {
            next: self.start.clone(),
            range: self
        }
    }
}

pub struct RangeIterator<T: Step> {
    range: Range<T>,
    next: T
}

impl<T: Step> Iterator for RangeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.clone();
        self.next = (self.next != self.range.end).then_some(match self.range.direction() {
            RangeDirection::Increasing => T::forward(self.next.clone(), 1),
            RangeDirection::Decreasing => T::backward(self.next.clone(), 1)
        })?;

        Some(current)
    }
}