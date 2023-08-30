use std::ops::Range;
use num::{FromPrimitive, Integer, Unsigned, One};

pub fn gauss_sum<T: Integer + FromPrimitive + Copy>(n: T) -> T {
    n * (n + One::one()) / T::from_u32(2).unwrap()
}

pub fn sum_range<T: Integer + FromPrimitive + Copy>(range: Range<T>) -> T {
    let amount = range.end - range.start;
    (range.start + range.end - One::one()) * amount / T::from_u32(2).unwrap()
}

pub struct NaturalNumbers<T: Integer + Unsigned + Copy> {
    next: T
}

impl<T: Integer + Unsigned + Copy> Iterator for NaturalNumbers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;
        self.next = self.next + One::one();
        Some(current)
    }
}

pub fn natural_numbers<T: Integer + Unsigned + Copy>(first: T) -> impl Iterator<Item=T> {
    NaturalNumbers { next: first }
}