use std::hash::Hash;

use itertools::Itertools;
use tupletools::fst;

pub fn mode<T>(it: impl Iterator<Item=T>) -> Option<T>
    where T: Eq + Hash
{
    it.counts()
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(fst)
}

pub fn multi_mode<T>(it: impl Iterator<Item=T>) -> Vec<T>
    where T: Eq + Hash
{
    it.counts()
        .into_iter()
        .max_set_by_key(|(_, count)| *count)
        .into_iter()
        .map(fst)
        .collect()
}