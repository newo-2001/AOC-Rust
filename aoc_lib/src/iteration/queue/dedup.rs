use std::{collections::HashSet, hash::Hash};

use super::{Queue, recursive_fold::{recursive_fold, try_recursive_fold}, FoldState, FindState, recursive_find::recursive_find};

pub struct DuplicateFilter<Q>
    where Q: Queue
{
    pub(crate) queue: Q
}

pub trait Dedupable
    where Self::Queue: Queue,
          Self: Queue + Sized
{
    type Queue;

    fn filter_duplicates(self) -> DuplicateFilter<Self> {
        DuplicateFilter { queue: self }
    }
}

impl<Q> Dedupable for Q where Q: Queue {
    type Queue = Q;
}

impl<Q> DuplicateFilter<Q>
    where Q::Out: Clone + Hash + Eq,
          Q: Queue
{
    pub fn recursive_fold<S, I: IntoIterator<Item=Q::In>>(
        self,
        state: S,
        folder: impl FnMut(S, Q::Out) -> FoldState<S, I>
    ) -> S {
        recursive_fold(self.queue, state, filter_fold_duplicates(folder))
    }

    pub fn try_recursive_fold<S, I: IntoIterator<Item=Q::In>, E>(
        self,
        state: S,
        folder: impl FnMut(S, Q::Out) -> Result<FoldState<S, I>, E>
    ) -> Result<S, E> {
        try_recursive_fold(self.queue, state, try_filter_fold_duplicates(folder))
    }

    pub fn recursive_find<R, I: IntoIterator<Item=Q::In>>(
        self,
        finder: impl FnMut(Q::Out) -> FindState<R, I>
    ) -> Option<R> {
        recursive_find(self.queue, filter_find_duplicates(finder))
    }
}

fn filter_fold_duplicates<S, T, O, I, F>(mut folder: F) ->
    impl FnMut(S, O) -> FoldState<S, I>
    where I: IntoIterator<Item=T>,
          F: FnMut(S, O) -> FoldState<S, I>,
          O: Clone + Hash + Eq + PartialEq
{
    let mut seen = HashSet::<O>::new();

    move |state: S, item: O| -> FoldState<S, I> {
        if !seen.insert(item.clone()) { return FoldState::Leaf(state) }
        folder(state, item)
    }
}

fn try_filter_fold_duplicates<S, T, O, I, F, E>(mut folder: F) ->
    impl FnMut(S, O) -> Result<FoldState<S, I>, E>
    where I: IntoIterator<Item=T>,
          F: FnMut(S, O) -> Result<FoldState<S, I>, E>,
          O: Clone + Hash + Eq + PartialEq
{
    let mut seen = HashSet::<O>::new();

    move |state: S, item: O| -> Result<FoldState<S, I>, E> {
        if !seen.insert(item.clone()) { return Ok(FoldState::Leaf(state)) }
        folder(state, item)
    }
}

fn filter_find_duplicates<R, T, O, I, F>(mut finder: F) ->
    impl FnMut(O) -> FindState<R, I>
    where I: IntoIterator<Item=T>,
          F: FnMut(O) -> FindState<R, I>,
          O: Clone + Hash + Eq + PartialEq
{
    let mut seen = HashSet::<O>::new();

    move |item: O| -> FindState<R, I> {
        if !seen.insert(item.clone()) { return FindState::Leaf }
        finder(item)
    }
}