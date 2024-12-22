use std::iter::repeat;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use yuki::{errors::NoSolution, parsing::parse_lines};

use crate::SolverResult;

const fn prune(num: u64) -> u64 {
    num % 16_777_216
}

const fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

#[allow(clippy::cast_possible_truncation)]
const fn price(secret: u64) -> u32 {
    (secret % 10) as u32
}

const fn next_secret(secret: u64) -> u64 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    prune(mix(secret, secret * 2048))
}

const ITERATIONS: u32 = 2000;

pub fn solve_part_1(input: &str) -> SolverResult {
    let numbers: Vec<u64> = parse_lines(input)?;

    let sum: u64 = numbers
        .into_par_iter()
        .map(|secret| (0..ITERATIONS)
            .fold(secret, |secret, _| next_secret(secret))
        )
        .sum();

    Ok(Box::new(sum))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let numbers: Vec<u64> = parse_lines(input)?;

    #[allow(clippy::cast_possible_wrap)]
    let price_changes: Vec<Vec<(u32, i32)>> = numbers
        .into_par_iter()
        .map(|secret| (0..ITERATIONS)
            .scan(secret, |secret, _| {
                let price = price(*secret);
                *secret = next_secret(*secret);
                Some(price)
            })
            .map_windows(|&[a, b]| (b, b as i32 - a as i32))
            .collect()
        )
        .collect();

    let max_bananas = repeat(-9..=9)
        .take(4)
        .multi_cartesian_product()
        .par_bridge()
        .map(|history| price_changes
            .iter()
            .map(|monkey| monkey
                .array_windows::<4>()
                .map(|&[(_, a), (_, b), (_, c), (price, d)]| (price, [a, b, c, d]))
                .find_map(|(price, window)| (window == history.as_slice()).then_some(price))
                .unwrap_or(0)
            )
            .sum::<u32>()
        )
        .max()
        .ok_or(NoSolution)?;

    Ok(Box::new(max_bananas))
}

#[cfg(test)]
mod tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn test_prune() {
        assert_eq!(16_113_920, prune(100_000_000));
    }
    
    #[test]
    fn test_mix() {
        assert_eq!(37, mix(42, 15));
    }

    #[test]
    fn test_evolution() {
        assert_equal(
            [
                15_887_950, 16_495_136, 527_345, 704_524, 1_553_684,
                12_683_156, 11_100_544, 12_249_484, 7_753_432, 5_908_254
            ],
            (0..10).scan(123, |secret, _| {
                *secret = next_secret(*secret);
                Some(*secret)
            })
        );
    }

    #[test]
    fn test_price() {
        assert_equal(
            [3, 0, 6, 5, 4, 4, 6, 4, 4, 2],
            (0..10).scan(123, |secret, _| {
                let price = price(*secret);
                *secret = next_secret(*secret);
                Some(price)
            })
        );
    }
}