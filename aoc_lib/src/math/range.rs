// We have `std::iter::range`, What is the purpose of this?
// https://kaylynn.gay/blog/post/rust_ranges_and_suffering

use std::{iter::Step, fmt::Display, cmp::{max, minmax}, ops::{Sub, Add, Div}};

use num::{FromPrimitive, One, Zero};

/// A contiguous range with a lower and upper bound.
/// Prefer this over [`std::iter::range`] for computations where iteration is not required
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Range<T = i32> {
    pub start: T,
    pub end: T
}

pub enum RangeDirection {
    Increasing,
    Decreasing
}

impl<T> Range<T> {
    pub fn exclusive(start: T, end: T) -> Range<T> {
        Self { start, end }
    }

    // TODO: inclusive ranges should be represented using a tag in the type system
    // This method is flawed because not the entire domain of `T` fits in the range
    pub fn inclusive(start: T, end: T) -> Range<T>
        where T: Step
    {
        let end = if start <= end { T::forward(end, 1) }
            else { T::backward(end, 1) };

        Self { start, end }
    }

    /// The [`RangeDirection`] of the range, `Increasing` or `Decreasing`.
    /// This is dependent on the order of the arguments and affects iteration order.
    pub fn direction(&self) -> RangeDirection
        where T: PartialOrd
    {
        if self.end >= self.start { RangeDirection::Increasing }
        else { RangeDirection::Decreasing }
    }

    /// Returns whether the item is contained in the range or not
    pub fn contains(&self, value: &T) -> bool
        where T: PartialOrd
    {
        let (small, big) = match self.direction() {
            RangeDirection::Decreasing => (&self.end, &self.start),
            RangeDirection::Increasing => (&self.start, &self.end)
        };
        
        small <= value && value < big
    }

    /// Returns whether another range is a subset of this range
    pub fn contains_range(&self, other: &Self) -> bool
        where T: PartialOrd
    {
        other.start >= self.start && other.end <= self.end
    }

    /// The amount of items in the range.
    /// Note that this will panic on an inclusive range of `0..=MAX`
    /// as the amount of items in the range is `MAX + 1`
    pub fn interval(self) -> T
        where T: Ord + One + Sub<Output=T> + Add<Output=T>
    {
        let [small, large] = minmax(self.start, self.end);
        large - small
    }

    /// Attempts to merge the two ranges.
    /// If they partially overlap, an Ok result with all items from both ranges is returned.
    /// If they are disjoint, Err is returned with a tuple containing the two original ranges.
    pub fn merge(self, other: Self) -> Result<Self, (Self, Self)>
        where T: Ord
    {
        if other.start <= self.end { Ok(Range::exclusive(self.start, max(self.end, other.end))) }
        else { Err((self, other)) }
    }

    /// Sums the entire range of numbers.
    /// Since the range is contiguous, an O(1) algorithm is used.
    pub fn sum(self) -> T
        where T: Clone + Ord + Zero + One + FromPrimitive +
                 Sub<Output=T> + Add<Output=T> + Div<Output=T>
    {
        let amount = self.clone().interval();
        let end = match self.direction() {
            RangeDirection::Increasing => self.end - T::one(),
            RangeDirection::Decreasing => self.end + T::one()
        };

        (self.start + end) * amount / T::from_u8(2).unwrap()
    }
}

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

#[cfg(test)]
mod tests {
    use crate::math::Range;

    #[test]
    fn contains() {
        assert!(!Range::exclusive(5, 10).contains(&10));
        assert!(!Range::inclusive(3, 5).contains(&6));
        assert!(Range::exclusive(3, 5).contains(&4));
        assert!(Range::inclusive(1, 1).contains(&1));
    }

    #[test]
    fn interval() {
        assert_eq!(7, Range::exclusive(3, 10).interval());
        assert_eq!(7, Range::exclusive(10, 3).interval());
        assert_eq!(1, Range::inclusive(1, 1).interval());
        assert_eq!(3, Range::inclusive(4, 6).interval());
        assert_eq!(3, Range::inclusive(6, 4).interval());
    }

    #[test]
    fn sum() {
        assert_eq!(7, Range::exclusive(3, 5).sum());
        assert_eq!(20, Range::inclusive(2, 6).sum());
        assert_eq!(9, Range::exclusive(5, 3).sum());
        assert_eq!(20, Range::inclusive(6, 2).sum());
    }

    #[test]
    fn merge() {
        let actual = Range::exclusive(5, 7).merge(Range::inclusive(7, 9));
        assert_eq!(Ok(Range::exclusive(5, 10)), actual);

        let first = Range::inclusive(5, 6);
        let second = Range::exclusive(8, 10);
        assert_eq!(Err((first, second)), first.merge(second));
    }
}