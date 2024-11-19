use super::Queue;

pub enum FoldState<S, I: IntoIterator> {
    Branch(S, I),
    Leaf(S)
}

pub(super) fn recursive_fold<F, S, I, Q>(mut queue: Q, mut state: S, mut folder: F) -> S
    where F: FnMut(S, Q::Out) -> FoldState<S, I>,
          I: IntoIterator<Item=Q::In>,
          Q: Queue,
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

pub(super) fn try_recursive_fold<F, S, I, Q, E>(mut queue: Q, mut state: S, mut folder: F) -> Result<S, E>
    where F: FnMut(S, Q::Out) -> Result<FoldState<S, I>, E>,
          I: IntoIterator<Item=Q::In>,
          Q: Queue,
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