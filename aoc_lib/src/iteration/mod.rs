use std::hash::Hash;

mod single;
mod mode;
pub mod queue;

pub use self::single::SingleError;

pub trait ExtraIter : Iterator + Sized {
    fn single(self) -> Result<Self::Item, SingleError>
        where Self: Sized
    {
        single::single(self)
    }

    fn empty(mut self) -> bool
        where Self: Sized
    {
        self.next().is_none()
    }

    fn mode(self) -> Option<Self::Item>
        where Self: Sized,
              Self::Item: Eq + Hash
    {
        mode::mode(self)
    }

    fn multi_mode(self) -> Vec<Self::Item>
        where Self: Sized,
              Self::Item: Eq + Hash
    {
        mode::multi_mode(self)
    }
}

impl<T> ExtraIter for T where T: Iterator { }