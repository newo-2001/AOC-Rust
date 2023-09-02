use std::hash::Hash;

mod single;
mod mode;
mod recursive_fold;

use self::recursive_fold::DuplicatesFilter;
pub use self::single::SingleError;
pub use recursive_fold::FoldState;

pub trait ExtraIter : Iterator + Sized {
    fn single(self) -> Result<Self::Item, SingleError>
        where Self: Sized
    {
        single::single(self)
    }

    fn empty(mut self) -> bool
        where Self: Sized
    {
        self.next().is_none()
    }

    fn mode(self) -> Option<Self::Item>
        where Self: Sized,
              Self::Item: Eq + Hash
    {
        mode::mode(self)
    }

    fn multi_mode(self) -> Vec<Self::Item>
        where Self: Sized,
              Self::Item: Eq + Hash
    {
        mode::multi_mode(self)
    }

    fn recursive_fold<S, I: IntoIterator<Item=Self::Item>>(
        self, state: S,
        folder: impl FnMut(S, Self::Item) -> FoldState<S, Self::Item, I>
    ) -> S {
        recursive_fold::recursive_fold::<S, Self::Item, I, Self>(self, state, folder)
    }

    fn try_recursive_fold<S, I: IntoIterator<Item=Self::Item>, E>(
        self, state: S,
        folder: impl FnMut(S, Self::Item) -> Result<FoldState<S, Self::Item, I>, E>
    ) -> Result<S, E> {
        recursive_fold::try_recursive_fold::<S, Self::Item, I, Self, E>(self, state, folder)
    }

    fn filter_duplicates(self) -> DuplicatesFilter<Self> {
        DuplicatesFilter { iter: self }
    }
}

impl<T> ExtraIter for T where T: Iterator { }