mod single;
mod mode;

pub use self::single::SingleError;
use std::hash::Hash;

pub trait ExtraIter : Iterator {
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

impl<T: ?Sized> ExtraIter for T where T: Iterator { }