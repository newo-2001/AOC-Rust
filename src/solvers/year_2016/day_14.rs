use std::collections::VecDeque;

use aoc_lib::functional::repeat_apply;
use crate::SolverResult;
use hex::ToHex;
use itertools::Itertools;

struct PossibleKey {
    index: usize,
    magic: char
}

struct KeyIterator<'a, F>
    where F: Fn(String) -> String
{
    index: usize,
    salt: &'a str,
    history: Vec<PossibleKey>,
    key_indices: VecDeque<usize>,
    generator: F
}

impl<F> Iterator for KeyIterator<'_, F>
    where F: Fn(String) -> String
{
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: multithreading
        while self.key_indices.is_empty() {
            let key = format!("{}{}", self.salt, self.index);
            let hash = (self.generator)(key);

            self.history.extract_if(|key| hash.contains(&key.magic.to_string().repeat(5)))
                .map(|possible_key| possible_key.index)
                .collect_into(&mut self.key_indices);

            self.history.retain_mut(|possible_key| self.index - possible_key.index < 1000);

            hash.chars()
                .dedup_with_count()
                .find(|(length, _)| length >= &3)
                .inspect(|&(_, magic)| self.history.push(PossibleKey { index: self.index, magic }));
            
            self.index += 1;
        }

        self.key_indices.pop_front()
    }
}

impl<'a, F> KeyIterator<'a, F>
    where F: Fn(String) -> String
{
    const fn new(salt: &'a str, generator: F) -> Self {
        KeyIterator {
            salt, generator, index: 0,
            history: Vec::new(),
            key_indices: VecDeque::new()
        }
    }
}

fn hash(key: String) -> String {
    md5::compute(key).encode_hex()
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_1(input: &str) -> SolverResult {
    let key_index = KeyIterator::new(input, hash)
        .nth(63).unwrap();

    Ok(Box::new(key_index))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_2(input: &str) -> SolverResult {
    let generator = |seed| repeat_apply(2017, seed, hash);
    let key_index = KeyIterator::new(input, generator)
        .nth(63).unwrap();

    Ok(Box::new(key_index))
}