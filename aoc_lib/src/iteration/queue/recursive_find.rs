use super::Queue;

pub enum FindState<R, I: IntoIterator> {
    Result(R),
    Branch(I),
    Leaf
}

pub(super) fn recursive_find<F, R, I, Q>( mut queue: Q, mut finder: F) -> Option<R>
    where F: FnMut(Q::Out) -> FindState<R, I>,
          I: IntoIterator<Item=Q::In>,
          Q: Queue
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