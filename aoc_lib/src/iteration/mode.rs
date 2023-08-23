use std::hash::Hash;

use itertools::Itertools;
use tupletools::fst;

pub trait Mode<'a, T> {
    fn mode(self) -> Box<dyn Iterator<Item=T> + 'a>;
}

impl<'a, I, T> Mode<'a, T> for I
    where I: Iterator<Item=T> + 'a,
          T: Eq + Hash + 'a
{
    fn mode(self) -> Box<dyn Iterator<Item=T> + 'a> {
        let iter = self.counts()
            .into_iter()
            .max_set_by_key(|&(_, frequency)| frequency)
            .into_iter()
            .map(fst);

        Box::new(iter)
    }
}