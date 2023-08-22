use std::{f32::consts::E, iter::repeat};

use aoc_runner_api::SolverResult;
use itertools::Itertools;

const GAMMA: f32 = 0.5772156649;

fn prime_factors(mut n: usize, primes: &Vec<usize>) -> Vec<usize> {
    let mut factors = Vec::new();
    
    while n != 1 {
        let factor = smallest_prime_factor(n, primes);

        factors.push(factor);
        n /= factor;
    }

    factors
}

fn factors(n: usize, limit: usize) -> Vec<usize> {
    (1..limit).filter(|&x| n % x == 0).collect()
}

fn smallest_prime_factor(n: usize, primes: &Vec<usize>) -> usize {
    *primes.iter()
        .skip_while(|&x| n % x != 0)
        .next().expect(&format!("Failed to compute spf({})", n))
}

fn sieve_eratosthenes(n: usize) -> Vec<usize> {
    let mut prime_table: Vec<bool> = Vec::from_iter(repeat(true).take(n));
    
    let root_n = f32::sqrt(n as f32) as usize;
    for i in 2..root_n {
        if !*prime_table.get(i).unwrap() { continue; }
        let mut j = i * i;
        while j < n {
            prime_table[j] = false;
            j += i;
        }
    }

    prime_table.into_iter()
        .skip(2) // 0 and 1
        .enumerate()
        .filter_map(|(i, x)| x.then_some(i + 2))
        .collect()
}

fn lower_bound_sum_of_divisors(sum: usize) -> usize {
    // Robin's theorem
    fn sum_upper_bound(sum: usize) -> usize {
        let bound = sum as f32;
        let loglogn = bound.log(E).log(E);
        (f32::powf(E, GAMMA) * bound * loglogn + 0.6483 * bound / loglogn) as usize
    }

    // Binary search Robin's theorem for the lower bound
    let mut step_size = 2;
    let mut lower_bound = 1;

    while step_size > 1 {
        let new_bound = lower_bound + step_size;
        if sum_upper_bound(new_bound) < sum {
            lower_bound = new_bound;
            step_size *= 2;
        } else {
            step_size /= 2;
        }
    }

    lower_bound
}

fn first_house_with_n_presents_infinite(presents: usize) -> usize {
    let primes = sieve_eratosthenes(presents / 10);

    fn presents_for_house(house: usize, primes: &Vec<usize>) -> usize {
        prime_factors(house, primes).into_iter()
            .counts().into_iter()
            .map(|(base, amount)| {
                (1..=amount).into_iter()
                    .fold(1, |acc, exponent| acc + base.pow(exponent as u32))
            }).product::<usize>() * 10
    }

    let mut house = lower_bound_sum_of_divisors(presents / 10);
    while presents_for_house(house, &primes) < presents {
        house += 1;
    }

    house
}

fn first_house_with_n_presents_finite(presents: usize, houses_per_elf: usize) -> usize {
    let presents_for_house = |house: usize| {
        factors(house, houses_per_elf)
            .into_iter().map(|x| house / x)
            .sum::<usize>() * 11
    };

    let mut house = 2;
    while presents_for_house(house) < presents {
        house += 1;
    }

    house
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let number: usize = input.parse()?;
    let house = first_house_with_n_presents_infinite(number);

    Ok(Box::new(house))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let number: usize = input.parse()?;
    let house = first_house_with_n_presents_finite(number, 50);

    Ok(Box::new(house))
}