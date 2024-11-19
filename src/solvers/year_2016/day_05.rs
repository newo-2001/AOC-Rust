use std::sync::Arc;

use crate::SolverResult;
use hex::ToHex;
use itertools::Itertools;
use rayon::prelude::{ParallelBridge, ParallelIterator, IntoParallelIterator};
use tupletools::snd;

struct PasswordIterator<'a, T, F>
    where F: Fn(&str) -> Option<T> + 'a
{
    index: u32,
    door_id: &'a str,
    extractor: Arc<F>
}

impl<'a, T, F> Iterator for PasswordIterator<'a, T, F>
    where F: Fn(&str) -> Option<T> + 'a,
          Arc<F>: Sync,
          T: Send
{
    type Item = T;

    // Using parallel iterators speeds this process up significantly
    // Because the order is not deterministic without an upper bound
    // We need to find that first and then check again using that bound
    // This is still faster than iterating once sequentially on modern hardware
    fn next(&mut self) -> Option<Self::Item> {
        let bound = (self.index..).par_bridge()
            .find_map_any(|index| self.hash(index).and(Some(index)))
            .unwrap();
        
        let digit = (self.index..=bound).into_par_iter()
            .find_map_first(|index| self.hash(index))
            .unwrap();
        
        self.index = bound + 1;
        Some(digit)
    }
}

impl<'a, T, F> PasswordIterator<'a, T, F>
    where F: Fn(&str) -> Option<T> + 'a
{
    fn hash(&self, index: u32) -> Option<T> {
        let data = format!("{}{}", self.door_id, index);

        let digest: String = md5::compute(data).encode_hex();
        digest.starts_with("00000")
            .then(|| (self.extractor)(&digest))
            .flatten()
    }

    fn new(door_id: &'a str, extractor: F) -> PasswordIterator<'a, T, F> {
        PasswordIterator { index: 0, door_id, extractor: Arc::new(extractor) }
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_1(input: &str) -> SolverResult {
    let extractor = |hash: &str| hash.chars().nth(5);

    let password: String = PasswordIterator::new(input, extractor)
        .take(8)
        .collect();

    Ok(Box::new(password))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_2(input: &str) -> SolverResult {
    fn extractor(hash: &str) -> Option<(usize, char)> {
        let mut chars = hash.chars();
        let index = chars.nth(5)?.to_digit(8)? as usize;
        let char = chars.next()?;
        Some((index, char))
    }

    let password: String = PasswordIterator::new(input, extractor)
        .unique_by(|&(index, _)| index)
        .take(8)
        .sorted_unstable_by_key(|&(index, _)| index)
        .map(snd)
        .collect();

    Ok(Box::new(password))
}