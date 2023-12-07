use std::{hash::Hash, iter::Sum, ops::{DerefMut, Deref}};

mod single;
mod mode;
pub mod queue;

use itertools::Itertools;

pub use self::single::SingleError;

pub trait ExtraIter: Iterator + Sized {
    fn single(self) -> Result<Self::Item, SingleError>
        where Self: Sized
    {
        single::single(self)
    }

    fn empty(mut self) -> bool {
        self.next().is_none()
    }

    fn mode(self) -> Option<Self::Item>
        where Self::Item: Eq + Hash
    {
        mode::mode(self)
    }

    fn multi_mode(self) -> Vec<Self::Item>
        where Self::Item: Eq + Hash
    {
        mode::multi_mode(self)
    }

    fn count_where(self, predicate: impl Fn(Self::Item) -> bool) -> usize {
        self.counts_by(predicate)
            .remove(&true)
            .unwrap_or_default()
    }

    fn sum_by<T: Sum>(self, key: impl Fn(Self::Item) -> T) -> T {
        self.map(key).sum()
    }

    fn replace_all<T>(self, needle: T, replacement: T)
        where Self::Item: DerefMut + Deref<Target=T>,
              T: PartialEq + Clone
    {
        for mut item in self {
            if *item == needle { *item = replacement.clone() }
        }
    }
}

impl<T> ExtraIter for T where T: Iterator {}