use std::{hash::Hash, collections::{VecDeque, HashSet}};

use priority_queue::PriorityQueue;
use tupletools::fst;

pub enum FoldState<S, T, I: IntoIterator<Item=T>> {
    Branch(S, I),
    Leaf(S)
}

pub trait Queue: Extend<Self::In> {
    type In;
    type Out;

    fn pop(&mut self) -> Option<Self::Out>;
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

fn recursive_fold<S, I, Q>(
    mut queue: Q,
    mut state: S,
    mut folder: impl FnMut(S, Q::Out) -> FoldState<S, Q::In, I>
) -> S
    where Q: Queue,
          I: IntoIterator<Item=Q::In>
{
    while let Some(item) = queue.pop() {
        state = match folder(state, item) {
            FoldState::Leaf(state) => state,
            FoldState::Branch(state, branches) => {
                queue.extend(branches);
                state
            }
        }
    }
    state
}

fn try_recursive_fold<S, I, Q, E>(
    mut queue: Q,
    mut state: S,
    mut folder: impl FnMut(S, Q::Out) -> Result<FoldState<S, Q::In, I>, E>
) -> Result<S, E>
    where Q: Queue,
          I: IntoIterator<Item=Q::In>
          
{
    while let Some(item) = queue.pop() {
        state = match folder(state, item)? {
            FoldState::Leaf(state) => state,
            FoldState::Branch(state, branches) => {
                queue.extend(branches);
                state
            }
        }
    }
    Ok(state)
}

fn filter_duplicates<S, T, O, I, F>(mut folder: F) ->
    impl FnMut(S, O) -> FoldState<S, T, I>
    where I: IntoIterator<Item=T>,
          F: FnMut(S, O) -> FoldState<S, T, I>,
          O: Clone + Hash + Eq + PartialEq
{
    let mut seen = HashSet::<O>::new();

    move |state: S, item: O| -> FoldState<S, T, I> {
        if !seen.insert(item.clone()) { return FoldState::Leaf(state) }
        folder(state, item)
    }
}

fn try_filter_duplicates<S, T, O, I, F, E>(mut folder: F) ->
    impl FnMut(S, O) -> Result<FoldState<S, T, I>, E>
    where I: IntoIterator<Item=T>,
          F: FnMut(S, O) -> Result<FoldState<S, T, I>, E>,
          O: Clone + Hash + Eq + PartialEq
{
    let mut seen = HashSet::<O>::new();

    move |state: S, item: O| -> Result<FoldState<S, T, I>, E> {
        if !seen.insert(item.clone()) { return Ok(FoldState::Leaf(state)) }
        folder(state, item)
    }
}

pub trait RecursiveFold<S, I>
    where I: IntoIterator<Item=Self::In>,
          Self: Sized + Queue
{
    fn recursive_fold(
        self,
        state: S,
        folder: impl FnMut(S, Self::Out) -> FoldState<S, Self::In, I>
    ) -> S {
        recursive_fold(self, state, folder)
    }

    fn try_recursive_fold<E>(
        self,
        state: S,
        folder: impl FnMut(S, Self::Out) -> Result<FoldState<S, Self::In, I>, E>
    ) -> Result<S, E> {
        try_recursive_fold(self, state, folder)
    }
}

impl<S, I, Q> RecursiveFold<S, I> for Q
    where I: IntoIterator<Item=Q::In>,
          Q: Queue
{ }

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
        folder: impl FnMut(S, Q::Out) -> FoldState<S, Q::In, I>
    ) -> S {
        recursive_fold(self.queue, state, filter_duplicates(folder))
    }

    pub fn try_recursive_fold<S, I: IntoIterator<Item=Q::In>, E>(
        self,
        state: S,
        folder: impl FnMut(S, Q::Out) -> Result<FoldState<S, Q::In, I>, E>
    ) -> Result<S, E> {
        try_recursive_fold(self.queue, state, try_filter_duplicates(folder))
    }
}