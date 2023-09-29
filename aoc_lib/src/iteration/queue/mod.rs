use std::collections::VecDeque;

mod recursive_iter;
mod recursive_find;
mod recursive_fold;
mod search_depth;
mod dedup;

pub use recursive_find::FindState;
pub use recursive_fold::FoldState;
pub use recursive_iter::IterState;
pub use search_depth::SearchDepth;
pub use dedup::{Dedupable, DuplicateFilter};

pub trait Queue: Extend<Self::In> + Sized {
    type In;
    type Out;

    fn push(&mut self, item: Self::In);
    fn pop(&mut self) -> Option<Self::Out>;

    fn recursive_fold<F, S, I>(self, state: S, folder: F) -> S
        where F: FnMut(S, Self::Out) -> FoldState<S, I>,
              I: IntoIterator<Item=Self::In>
    {
        recursive_fold::recursive_fold(self, state, folder)
    }

    fn try_recursive_fold<F, S, I, E>(self, state: S, folder: F) -> Result<S, E>
        where F: FnMut(S, Self::Out) -> Result<FoldState<S, I>, E>,
              I: IntoIterator<Item=Self::In>
    {
        recursive_fold::try_recursive_fold(self, state, folder)
    }

    fn recursive_find<F, R, I>(self, finder: F) -> Option<R>
        where F: FnMut(Self::Out) -> FindState<R, I>,
              I: IntoIterator<Item=Self::In>
    {
        recursive_find::recursive_find(self, finder)
    }

    fn try_recursive_find<F, R, E, I>(self, finder: F) -> Result<Option<R>, E>
        where F: FnMut(Self::Out) -> Result<FindState<R, I>, E>,
              I: IntoIterator<Item=Self::In>
    {
        recursive_find::try_recursive_find(self, finder)
    }

    fn recursive_iter<F, I>(self, action: F)
        where F: FnMut(Self::Out) -> IterState<I>,
              I: IntoIterator<Item=Self::In>
    {
        recursive_iter::recursive_iter(self, action);
    }
}

impl<T> Queue for VecDeque<T> {
    type In = T;
    type Out = T;

    fn pop(&mut self) -> Option<Self::Out> {
        self.pop_front()
    }

    fn push(&mut self, item: Self::In) {
        self.push_back(item);
    }
}

impl<T> Queue for Vec<T> {
    type In = T;
    type Out = T;

    fn pop(&mut self) -> Option<Self::Out> {
        self.pop()
    }

    fn push(&mut self, item: Self::In) {
        self.push(item)
    }
}