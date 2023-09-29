use std::ops::BitXor;

use hex::ToHex;
use itertools::Itertools;

pub(super) fn rounds(lengths: &[u8], rounds: u32) -> impl Iterator<Item=u8> {
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

pub(super) fn hash(seed: impl AsRef<str>) -> String {
    let lengths: Box<[u8]> = seed.as_ref()
        .bytes()
        .chain([17, 31, 73, 47, 23])
        .collect();

    rounds(&lengths, 64)
        .array_chunks::<16>()
        .map(|block| block.into_iter().reduce(BitXor::bitxor).unwrap())
        .collect_vec()
        .encode_hex()
}