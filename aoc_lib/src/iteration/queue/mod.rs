use std::{hash::Hash, collections::VecDeque};

use priority_queue::PriorityQueue;
use tupletools::fst;

mod recursive_find;
mod recursive_fold;
mod dedup;

pub use recursive_find::FindState;
pub use recursive_fold::FoldState;
pub use dedup::{Dedupable, DuplicateFilter};

pub trait Queue: Extend<Self::In> + Sized {
    type In;
    type Out;

    fn pop(&mut self) -> Option<Self::Out>;

    fn recursive_fold<S, I: IntoIterator<Item=Self::In>>(
        self,
        state: S,
        folder: impl FnMut(S, Self::Out) -> FoldState<S, I>
    ) -> S {
        recursive_fold::recursive_fold(self, state, folder)
    }

    fn try_recursive_fold<S, I: IntoIterator<Item=Self::In>, E>(
        self,
        state: S,
        folder: impl FnMut(S, Self::Out) -> Result<FoldState<S, I>, E>
    ) -> Result<S, E> {
        recursive_fold::try_recursive_fold(self, state, folder)
    }

    fn recursive_find<R, I: IntoIterator<Item=Self::In>>(
        self,
        finder: impl FnMut(Self::Out) -> FindState<R, I>
    ) -> Option<R> {
        recursive_find::recursive_find(self, finder)
    }
}

impl<T, P> Queue for PriorityQueue<T, P>
    where T: Hash + Eq,
          P: Ord
{
    type In = (T, P);
    type Out = T;

    fn pop(&mut self) -> Option<T> {
        self.pop().map(fst)
    }
}

impl<T> Queue for VecDeque<T> {
    type In = T;
    type Out = T;

    fn pop(&mut self) -> Option<T> {
        self.pop_front()
    }
}