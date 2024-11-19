// We have `std::iter::range`, What is the purpose of this?
// https://kaylynn.gay/blog/post/rust_ranges_and_suffering

use std::{iter::Step, fmt::Display, cmp::{max, min}, ops::{Sub, Add, Div}};

use num::{FromPrimitive, One, Zero};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Hash)]
#[error("Invalid range, start can't be greater than end: {0}")]
pub struct InvalidRangeError<T>(pub Range<T>);

/// A contiguous range with a lower and upper bound.
/// Prefer this over [`std::iter::range`] for computations where iteration is not required
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Range<T = i32> {
    pub start: T,
    pub end: T
}

impl<T> Range<T> {
    /// Produces the empty range: `(0..0)`.
    #[must_use]
    pub fn empty() -> Range<T> where
        T: Zero
    {
        Self { start: T::zero(), end: T::zero() }
    }

    /// Creates a range with an exclusive upperbound.
    /// If `start > end` this returns an error.
    pub fn exclusive(start: T, end: T) -> Result<Range<T>, InvalidRangeError<T>> where
        T: PartialOrd
    {
        let range = Self { start, end };
        if range.start <= range.end { Ok(range) } else { Err(InvalidRangeError(range)) }
    }

    // TODO: inclusive ranges should be represented using a tag in the type system
    // This method is flawed because not the entire domain of `T` fits in the range
    /// Creates a range with an inclusive upperbound.
    /// If `start > end` this returns an error.
    /// Additionally, this function panics if `end == T::MAX`.
    pub fn inclusive(start: T, end: T) -> Result<Range<T>, InvalidRangeError<T>> where
        T: Step + PartialOrd
    {
        let range = Self { start, end: T::forward(end, 1) };
        if range.start <= range.end { Ok(range) } else { Err(InvalidRangeError(range)) }
    }

    /// Returns whether the item is contained in the range or not
    pub fn contains(&self, value: &T) -> bool
        where T: PartialOrd
    {
        &self.start <= value && value < &self.end
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
        self.end - self.start
    }

    /// Attempts to merge the two ranges.
    /// If they partially overlap, an Ok result with all items from both ranges is returned.
    /// If they are disjoint, Err is returned with a tuple containing the two original ranges.
    pub fn merge(self, other: Self) -> Result<Self, (Self, Self)>
        where T: Ord
    {
        if other.start <= self.end {
            Ok(Range { start: self.start,end: max(self.end, other.end) })
        } else { Err((self, other)) }
    }

    /// Sums the entire range of numbers.
    /// Since the range is contiguous, an O(1) algorithm is used.
    pub fn sum(self) -> T where
        T: Clone + Ord + Zero + One + FromPrimitive +
           Sub<Output=T> + Add<Output=T> + Div<Output=T>
    {
        let amount = self.clone().interval();
        let end = self.end - T::one();

        (self.start + end) * amount / T::from_u8(2).unwrap()
    }

    /// Remove a given range from this one, returning the part before and after it.
    /// If the other range is not entirely contained in this one, a single range is returned
    /// that contains just the part before or after it, whichever is applicable.
    pub fn split_range(self, other: Self) -> (Self, Option<Self>) where
        T: Ord + Copy
    {
        if self.contains_range(&other) {
            (
                Range { start: self.start, end: other.start },
                Some(Range { start: other.end, end: self.end })
            )
        } else if other.end < self.end {
            (Range { start: max(self.start, other.end), end: self.end }, None)
        } else {
            (Range { start: self.start, end: min(self.end, other.start) }, None)
        }
    }
}

impl<T: Display> Display for Range<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}..{})", self.start, self.end)
    }
}

impl<T: Default> Default for Range<T> where {
    fn default() -> Self {
        Range { start: T::default(), end: T::default() }
    }
}

impl<T: Step> IntoIterator for Range<T> {
    type Item = T;
    type IntoIter = core::ops::Range<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..self.end
    }
}

#[macro_export]
macro_rules! range {
    ($start: literal..$end: literal) => {
        {
            static_assertions::const_assert!($start <= $end);
            $crate::math::Range { start: $start, end: $end }
        }
    };

    ($start: literal..=$end: literal) => {
        {
            static_assertions::const_assert!($start <= $end);
            $crate::math::Range { start: $start, end: $end + 1 }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::{Range, InvalidRangeError};

    #[test]
    fn constructors() {
        assert_eq!(Ok(range!(5..6)), Range::exclusive(5, 6));
        assert_eq!(Ok(range!(6..=8)), Range::inclusive(6, 8));

        let error = Result::<Range, _>::Err(InvalidRangeError(Range { start: 5, end: 3 }));
        assert_eq!(error, Range::exclusive(5, 3));
        assert_eq!(error, Range::inclusive(5, 2));
    }

    #[test]
    fn contains() {
        assert!(!range!(5..10).contains(&10));
        assert!(!range!(3..5).contains(&6));
        assert!(range!(3..5).contains(&4));
        assert!(range!(1..=1).contains(&1));
    }

    #[test]
    fn interval() {
        assert_eq!(7, range!(3..10).interval());
        assert_eq!(1, range!(1..=1).interval());
        assert_eq!(3, range!(4..=6).interval());
    }

    #[test]
    fn sum() {
        assert_eq!(7, range!(3..5).sum());
        assert_eq!(20, range!(2..=6).sum());
    }

    #[test]
    fn merge() {
        let actual = range!(5..7).merge(range!(7..=9));
        assert_eq!(Ok(range!(5..10)), actual);

        let first = range!(5..=6);
        let second = range!(8..10);
        assert_eq!(Err((first, second)), first.merge(second));
    }

    #[test]
    fn split_range() {
        let range = range!(5..10);
        
        // Inside
        let actual = range.split_range(range!(7..9));
        assert_eq!((range!(5..7), Some(range!(9..10))), actual);
        
        // Start
        let actual = range.split_range(range!(3..7));
        assert_eq!((range!(7..10), None), actual);

        // End
        let actual = range.split_range(range!(7..12));
        assert_eq!((range!(5..7), None), actual);

        // Before
        let actual = range.split_range(range!(2..5));
        assert_eq!((range, None), actual);

        // After
        let actual = range.split_range(range!(10..18));
        assert_eq!((range, None), actual);
    }
}