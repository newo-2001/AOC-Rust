use aoc_runner_api::SolverResult;
use hex::ToHex;
use md5;
use rayon::prelude::{ParallelBridge, ParallelIterator, IntoParallelIterator};

fn hash(hash_prefix: &str, key_prefix: &str, index: u32) -> bool {
    let key = format!("{}{}", key_prefix, index);
    let hash: String = md5::compute(key).encode_hex();
    hash.starts_with(hash_prefix)
}

fn proof_of_work(zeroes: usize, key_prefix: &str) -> u32 {
    let hash_prefix = "0".repeat(zeroes);
    let hash = |n: &u32| hash(&hash_prefix, key_prefix, *n);

    // Using parallel iterators speeds this process up significantly
    // Because the order is not deterministic without an upper bound
    // We need to find that first and then check again using that bound
    // This is still faster than iterating once sequentially on modern hardware
    let bound = (0..).par_bridge()
        .find_any(hash)
        .unwrap();

    (0..=bound).into_par_iter()
        .find_first(hash)
        .unwrap()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    Ok(Box::from(proof_of_work(5, input)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    Ok(Box::from(proof_of_work(6, input)))
}