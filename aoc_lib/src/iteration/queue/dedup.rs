use std::{hash::Hash, mem};

use ahash::{HashSet, HashSetExt};

use super::{Queue, recursive_fold::{recursive_fold, try_recursive_fold}, FoldState, FindState, recursive_find::{recursive_find, try_recursive_find}, IterState, recursive_iter::recursive_iter};

pub struct DuplicateFilter<Q>
    where Q: Queue + Default
{
    pub(crate) queue: Q,
    pub seen: HashSet<Q::Out>
}

pub trait Dedupable
    where Self: Queue + Default
{
    fn filter_duplicates(self) -> DuplicateFilter<Self> {
        DuplicateFilter { queue: self, seen: HashSet::<Self::Out>::new() }
    }
}

impl<Q> Dedupable for Q where Q: Queue + Default {}

impl<Q> DuplicateFilter<Q>
    where Q::Out: Clone + Hash + Eq,
          Q: Queue + Default
{
    pub fn recursive_fold<S, I, F>(&mut self, state: S, folder: F) -> S
        where F: FnMut(S, Q::Out) -> FoldState<S, I>,
              I: IntoIterator<Item=Q::In>
    {
        recursive_fold(mem::take(&mut self.queue), state, self.filter_fold_duplicates(folder))
    }

    fn filter_fold_duplicates<'a, S, I, F>(&'a mut self, mut folder: F) -> impl FnMut(S, Q::Out) -> FoldState<S, I> + 'a
        where F: FnMut(S, Q::Out) -> FoldState<S, I> + 'a,
              I: IntoIterator<Item=Q::In>
    {
        move |state: S, item: Q::Out| -> FoldState<S, I> {
            if !self.seen.insert(item.clone()) { return FoldState::Leaf(state) }
            folder(state, item)
        }
    }

    pub fn try_recursive_fold<S, I, F, E>(&mut self, state: S, folder: F) -> Result<S, E>
        where F: FnMut(S, Q::Out) -> Result<FoldState<S, I>, E>,
              I: IntoIterator<Item=Q::In>
    {
        try_recursive_fold(mem::take(&mut self.queue), state, self.try_filter_fold_duplicates(folder))
    }

    fn try_filter_fold_duplicates<'a, S, I, F, E>(&'a mut self, mut folder: F) -> impl FnMut(S, Q::Out) -> Result<FoldState<S, I>, E> + 'a
        where F: FnMut(S, Q::Out) -> Result<FoldState<S, I>, E> + 'a,
            I: IntoIterator<Item=Q::In>,
            Q: Queue,
            Q::Out: Eq + Hash + Clone
    {
        move |state: S, item: Q::Out| -> Result<FoldState<S, I>, E> {
            if !self.seen.insert(item.clone()) { return Ok(FoldState::Leaf(state)) }
            folder(state, item)
        }
    }

    pub fn recursive_find<R, I, F>(&mut self, finder: F) -> Option<R>
        where F: FnMut(Q::Out) -> FindState<R, I>,
              I: IntoIterator<Item=Q::In>
    {
        recursive_find(mem::take(&mut self.queue), self.filter_find_duplicates(finder))
    }

    fn filter_find_duplicates<'a, R, I, F>(&'a mut self, mut finder: F) -> impl FnMut(Q::Out) -> FindState<R, I> + 'a
        where F: FnMut(Q::Out) -> FindState<R, I> + 'a,
              I: IntoIterator<Item=Q::In>
    {
        move |item: Q::Out| -> FindState<R, I> {
            if !self.seen.insert(item.clone()) { return FindState::Leaf }
            finder(item)
        }
    }

    pub fn try_recursive_find<R, E, I, F>(&mut self, finder: F) -> Result<Option<R>, E>
        where F: FnMut(Q::Out) -> Result<FindState<R, I>, E>,
              I: IntoIterator<Item=Q::In>
    {
        try_recursive_find(mem::take(&mut self.queue), self.try_filter_find_duplicates(finder))
    }

    fn try_filter_find_duplicates<'a, R, E, I, F>(&'a mut self, mut finder: F) -> impl FnMut(Q::Out) -> Result<FindState<R, I>, E> + 'a
        where F: FnMut(Q::Out) -> Result<FindState<R, I>, E> + 'a,
              I: IntoIterator<Item=Q::In>
    {
        move |item: Q::Out| -> Result<FindState<R, I>, E> {
            if !self.seen.insert(item.clone()) { return Ok(FindState::Leaf) }
            finder(item)
        }
    }

    pub fn recursive_iter<I, F>(&mut self, action: F)
        where F: FnMut(Q::Out) -> IterState<I>,
              I: IntoIterator<Item=Q::In>
    {
        recursive_iter(mem::take(&mut self.queue), self.filter_recursive_iter(action));
    }    

    fn filter_recursive_iter<'a, I, F>(&'a mut self, mut action: F) -> impl FnMut(Q::Out) -> IterState<I> + 'a
        where F: FnMut(Q::Out) -> IterState<I> + 'a,
              I: IntoIterator<Item=Q::In>
    {
        move |item: Q::Out| -> IterState<I> {
            if !self.seen.insert(item.clone()) { return IterState::Leaf }
            action(item)
        }
    }    
}