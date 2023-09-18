use std::ops::BitXor;

use aoc_lib::NoSolutionError;
use aoc_runner_api::SolverResult;
use hex::ToHex;
use itertools::Itertools;

fn knot_hash_rounds(lengths: &[u8], rounds: u32) -> impl Iterator<Item=u8> {
    let mut data = (0..=255).collect_vec();
    let mut position = 0;
    let mut skip_size = 0;

    for _ in 0..rounds {
        for length in lengths.iter().copied() {
            for offset in 0..(usize::from(length) / 2) {
                let left = (position + offset) % data.len();
                let right = (position + usize::from(length) - offset - 1) % data.len();
                data.swap(left, right);
            }

            position += usize::from(length) + skip_size;
            skip_size += 1;
        }
    }

    data.into_iter()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let lengths: Box<[u8]> = input.split(',').map(str::parse).try_collect()?;
    let hash: u16 = knot_hash_rounds(&lengths, 1)
        .next_chunk::<2>()
        .map_err(|_| NoSolutionError)?
        .map(u16::from)
        .into_iter()
        .product();

    Ok(Box::new(hash))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_2(input: &str) -> SolverResult {
    let lengths: Box<[u8]> = input.bytes().chain([17, 31, 73, 47, 23]).collect();
    let hash: String = knot_hash_rounds(&lengths, 64)
        .array_chunks::<16>()
        .map(|block| block.into_iter().reduce(BitXor::bitxor).unwrap())
        .collect_vec()
        .encode_hex();

    Ok(Box::new(hash))
}