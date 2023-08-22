use std::ops::Range;
use num::{FromPrimitive, Integer, Unsigned, One};
use gen_iter::gen_iter;

pub fn gauss_sum<T: Integer + FromPrimitive + Copy>(n: T) -> T {
    n * (n + One::one()) / T::from_u32(2).unwrap()
}

pub fn sum_range<T: Integer + FromPrimitive + Copy>(range: Range<T>) -> T {
    let amount = range.end - range.start;
    (range.start + range.end - One::one()) * amount / T::from_u32(2).unwrap()
}

pub fn natural_numbers<T: Integer + Unsigned + Copy>(mut first: T) -> impl Iterator<Item=T> {
    gen_iter!(move {
        loop {
            first = first + One::one();
            yield first;
        }
    })
}