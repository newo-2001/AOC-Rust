use std::{error::Error, fs, num::ParseIntError};

use itertools::Itertools;

fn different_sums(total: u32, sizes: &Vec<u32>) -> impl Iterator<Item = Vec<&u32>> {
    sizes.iter()
        .powerset()
        .filter(move |containers| containers.iter()
            .fold(0, |acc, &x| acc + x) == total)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/day_17.txt")?;

    let containers = content.lines()
        .map(|x| u32::from_str_radix(x, 10))
        .collect::<Result<Vec<u32>, ParseIntError>>()?;

    let combinations: Vec<Vec<&u32>> = different_sums(150, &containers).collect();
    println!("The total amount of different container combinations to reach 150 is {}", combinations.len());

    let sized_combinations = combinations.iter()
        .group_by(|containers| containers.len());

    let (size, combinations) = sized_combinations.into_iter()
        .sorted_unstable_by_key(|(amount, _)| *amount)
        .next().expect("No valid combinations found");

    println!("The amount of combinations to reach 150 with the minimal amount of {} containers is {}", size, combinations.count());

    Ok(())
}