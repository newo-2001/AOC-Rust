use std::ops::Range;
use num::{FromPrimitive, Integer};

pub fn gauss_sum<T: Integer + FromPrimitive + Copy>(n: T) -> T {
    n * (n + T::from_u32(1).unwrap()) / T::from_u32(2).unwrap()
}

pub fn sum_range<T: Integer + FromPrimitive + Copy>(range: Range<T>) -> T {
    let amount = range.end - range.start;
    (range.start + range.end - T::from_u32(1).unwrap()) * amount / T::from_u32(2).unwrap()
}