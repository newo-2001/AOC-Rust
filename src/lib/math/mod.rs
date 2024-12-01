mod bit;
mod range;
mod quadratic;

use std::cmp::Ordering;

pub use bit::*;
pub use range::*;
pub use quadratic::*;

pub fn min_max_by<T>(a: T, b: T, compare: impl FnOnce(&T, &T) -> Ordering) -> (T, T) {
    match compare(&a, &b) {
        Ordering::Equal | Ordering::Less => (a, b),
        Ordering::Greater => (b, a)
    }
}

pub fn min_max_by_key<T, K: Ord>(a: T, b: T, key: impl Fn(&T) -> &K) -> (T, T) {
    if key(&a) < key(&b) { (a, b) } else { (b, a) }
}