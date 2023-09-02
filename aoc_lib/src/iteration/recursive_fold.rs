use std::{hash::Hash, collections::{VecDeque, HashSet}};

pub enum FoldState<S, T, I: IntoIterator<Item=T>> {
    Branch(S, I),
    Leaf(S)
}

pub fn recursive_fold<S, T, I: IntoIterator<Item=T>, J: IntoIterator<Item=T>>(
    iter: J, mut state: S,
    mut folder: impl FnMut(S, T) -> FoldState<S, T, I>
) -> S {
    let mut queue = VecDeque::from_iter(iter);
    while let Some(item) = queue.pop_front() {
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

pub fn try_recursive_fold<S, T, I: IntoIterator<Item=T>, J: IntoIterator<Item=T>, E>(
    iter: J, mut state: S,
    mut folder: impl FnMut(S, T) -> Result<FoldState<S, T, I>, E>
) -> Result<S, E> {
    let mut queue = VecDeque::from_iter(iter);
    while let Some(item) = queue.pop_front() {
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

fn filter_duplicates<S, T, I, F>(mut folder: F) ->
    impl FnMut(S, T) -> FoldState<S, T, I>
    where I: Iterator<Item=T>,
          F: FnMut(S, T) -> FoldState<S, T, I>,
          T: Clone + Hash + Eq + PartialEq
{
    let mut seen = HashSet::<T>::new();

    move |state: S, item: T| -> FoldState<S, T, I> {
        if !seen.insert(item.clone()) { return FoldState::Leaf(state) }
        folder(state, item)
    }
}

fn try_filter_duplicates<S, T, I, F, E>(mut folder: F) ->
    impl FnMut(S, T) -> Result<FoldState<S, T, I>, E>
    where I: Iterator<Item=T>,
          F: FnMut(S, T) -> Result<FoldState<S, T, I>, E>,
          T: Clone + Hash + Eq + PartialEq
{
    let mut seen = HashSet::<T>::new();

    move |state: S, item: T| -> Result<FoldState<S, T, I>, E> {
        if !seen.insert(item.clone()) { return Ok(FoldState::Leaf(state)) }
        folder(state, item)
    }
}

pub struct DuplicatesFilter<I: Iterator> {
    pub(crate) iter: I
}

impl<J, T> DuplicatesFilter<J>
    where J: Iterator<Item=T>,
          T: Clone + Eq + Hash
{
    pub fn recursive_fold<S, I: Iterator<Item=T>>(
        self, state: S,
        folder: impl FnMut(S, T) -> FoldState<S, T, I>
    ) -> S {
        recursive_fold(self.iter, state, filter_duplicates(folder))
    }

    pub fn try_recursive_fold<S, I: Iterator<Item=T>, E>(
        self, state: S,
        folder: impl FnMut(S, T) -> Result<FoldState<S, T, I>, E>
    ) -> Result<S, E> {
        try_recursive_fold(self.iter, state, try_filter_duplicates(folder))
    }
}