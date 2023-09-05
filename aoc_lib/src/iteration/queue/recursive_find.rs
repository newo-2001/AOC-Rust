use super::Queue;

pub enum FindState<R, I: IntoIterator> {
    Result(R),
    Branch(I),
    Leaf
}

pub(crate) fn recursive_find<R, I, Q>(
    mut queue: Q,
    mut finder: impl FnMut(Q::Out) -> FindState<R, I>
) -> Option<R>
    where Q: Queue,
          I: IntoIterator<Item=Q::In>
{
    while let Some(item) = queue.pop() {
        match finder(item) {
            FindState::Branch(branches) => queue.extend(branches),
            FindState::Result(result) => return Some(result),
            FindState::Leaf => {}
        }
    }
    None
}