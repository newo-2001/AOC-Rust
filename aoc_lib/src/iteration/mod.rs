mod single;
mod mode;
mod linear_sequence;

use self::linear_sequence::LinearSequence;
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

    fn linear_sequence(self, step_size: usize, offset: usize) -> LinearSequence<Self>
        where Self: Sized
    {
        LinearSequence { it: self, step_size, offset }
    }
}

impl<T: ?Sized> ExtraIter for T where T: Iterator { }