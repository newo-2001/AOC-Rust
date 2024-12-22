use rayon::iter::{IntoParallelIterator, ParallelIterator};
use yuki::parsing::parse_lines;

use crate::SolverResult;

const fn prune(num: u64) -> u64 {
    num % 16_777_216
}

const fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

const fn next_secret(secret: u64) -> u64 {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    prune(mix(secret, secret * 2048))
}


pub fn solve_part_1(input: &str) -> SolverResult {
    const ITERATIONS: u32 = 2000;
    let numbers: Vec<u64> = parse_lines(input)?;

    let sum: u64 = numbers
        .into_par_iter()
        .map(|secret| (0..ITERATIONS)
            .fold(secret, |secret, _| next_secret(secret))
        )
        .sum();

    Ok(Box::new(sum))
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
}