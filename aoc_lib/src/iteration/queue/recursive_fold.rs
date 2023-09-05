use super::Queue;

pub enum FoldState<S, I: IntoIterator> {
    Branch(S, I),
    Leaf(S)
}

pub(crate) fn recursive_fold<S, I, Q>(
    mut queue: Q,
    mut state: S,
    mut folder: impl FnMut(S, Q::Out) -> FoldState<S, I>
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

pub(crate) fn try_recursive_fold<S, I, Q, E>(
    mut queue: Q,
    mut state: S,
    mut folder: impl FnMut(S, Q::Out) -> Result<FoldState<S, I>, E>
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