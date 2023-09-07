mod bit;
pub use bit::*;

use std::ops::Range;
use num::{FromPrimitive, Integer, One};

pub fn gauss_sum<T: Integer + FromPrimitive + Copy>(n: T) -> T {
    n * (n + One::one()) / T::from_u32(2).unwrap()
}

pub fn sum_range<T: Integer + FromPrimitive + Copy>(range: Range<T>) -> T {
    let amount = range.end - range.start;
    (range.start + range.end - One::one()) * amount / T::from_u32(2).unwrap()
}

pub fn min_max<T: Ord>(a: T, b: T) -> (T, T) {
    if a > b { (b, a) } else { (a, b) }
}