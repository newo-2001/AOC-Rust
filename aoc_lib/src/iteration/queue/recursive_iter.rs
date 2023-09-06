use super::Queue;

pub enum IterState<I> {
    Branch(I),
    Leaf
}

pub(super) fn recursive_iter<F, I, Q>(mut queue: Q, mut folder: F)
    where F: FnMut(Q::Out) -> IterState<I>,
          I: IntoIterator<Item=Q::In>,
          Q: Queue,
{
    while let Some(item) = queue.pop() {
        match folder(item) {
            IterState::Branch(branches) => queue.extend(branches),
            IterState::Leaf => {},
        }
    }
}