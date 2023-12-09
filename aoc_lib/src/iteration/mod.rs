use std::{hash::Hash, iter::Sum, ops::{DerefMut, Deref}};
use itertools::Itertools;

mod single;
mod mode;
mod try_fold_while;
mod generator;
pub mod queue;

pub use generator::{Generator, generate};
pub use single::SingleError;
pub use try_fold_while::TryFoldWhile;

pub trait ExtraIter: Iterator + Sized {
    fn single(self) -> Result<Self::Item, SingleError> {
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

    fn try_fold_while<T, E, F>(self, init: T, folder: F) -> Result<T, E>
        where F: Fn(T, Self::Item) -> TryFoldWhile<T, E>
    {
        try_fold_while::try_fold_while(self, init, folder)
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

impl<I> ExtraIter for I where I: Iterator {}