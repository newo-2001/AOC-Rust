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
    /// Returns `Ok` if the iterator contained exactly one element.
    /// Otherwise it will return [`SingleError`]
    /// indicating whether the iterator was empty or contained more items.
    fn single(self) -> Result<Self::Item, SingleError> {
        single::single(self)
    }

    /// Returns true if the iterator contains no elements.
    fn empty(mut self) -> bool {
        self.next().is_none()
    }

    /// Returns the most common element in the iterator.
    /// In the case of ties this function makes no guarantees
    /// about which element is returned.
    /// If you do care about this use [`ExtraIter::multi_mode`] instead.
    fn mode(self) -> Option<Self::Item>
        where Self::Item: Eq + Hash
    {
        mode::mode(self)
    }

    /// Returns a list of elements that tie for most common element.
    /// Like [`ExtraIter::mode`], but predictable in the case of ties.
    fn multi_mode(self) -> Vec<Self::Item>
        where Self::Item: Eq + Hash
    {
        mode::multi_mode(self)
    }

    /// Fold the iterator over a fallible function that can short circuit.
    fn try_fold_while<T, E, F>(self, init: T, folder: F) -> Result<T, E>
        where F: Fn(T, Self::Item) -> TryFoldWhile<T, E>
    {
        try_fold_while::try_fold_while(self, init, folder)
    }

    /// Counts the amount of items that meet a predicate.
    /// Like chaining [`Iterator::filter`] and [`Iterator::count`]
    fn count_where(self, predicate: impl Fn(Self::Item) -> bool) -> usize {
        self.counts_by(predicate)
            .remove(&true)
            .unwrap_or_default()
    }

    /// Sum the values by a particular key.
    /// Like chaining [`Iterator::map`] and [`Iterator::sum`]
    fn sum_by<T: Sum>(self, key: impl Fn(Self::Item) -> T) -> T {
        self.map(key).sum()
    }

    /// Replaces all occurances of `needle` with `replacement`.
    /// This requires the iterator to contain mutable references.
    fn replace_all<T>(self, needle: T, replacement: T)
        where Self::Item: DerefMut + Deref<Target=T>,
              T: PartialEq + Clone
    {
        for mut item in self {
            if *item == needle { *item = replacement.clone() }
        }
    }

    /// Returns true if none of the items in the iterator match the predicate.
    /// Like [`Iterator::all`], but with the condition inverted.
    fn none(&mut self, predicate: impl Fn(Self::Item) -> bool) -> bool {
        self.all(|item| !(predicate)(item))
    }
}

impl<I> ExtraIter for I where I: Iterator {}