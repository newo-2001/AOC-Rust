use std::fs;
use itertools::Itertools;

type Present := Vec<u32>;

fn parse_present(str: &str) -> Result<Present, String> {
    return str.split('x')
        .map(|side| side.parse::<u32>().map_err(|err| err.to_string()))
        .collect::<Result<Vec<u32>, String>>();
}

fn required_wrapping_paper(present: &Present) -> u32 {
    let areas: Vec<u32> = present.iter()
        .combinations(2)
        .map(|sides| sides.into_iter().fold(1, |acc, x| acc * x))
        .collect();

    let min_area: &u32 = areas.iter().min().unwrap_or(&0);
    let total_area: u32 = areas.iter().sum::<u32>() * 2;
    return min_area + total_area;
}

fn required_ribbon(present: &Present) -> u32 {
    let smallest_perimeter: u32 = present.iter()
        .combinations(2)
        .map(|sides| sides.into_iter().sum::<u32>() * 2)
        .min().unwrap_or(0);

    let volume: u32 = present.iter()
        .fold(1, |acc, x| acc * x);
    
    return smallest_perimeter + volume;
}

fn main() {
    let presents: Vec<Present> = fs::read_to_string("inputs/day_2.txt")
        .expect("Failed to find input file!")
        .lines()
        .map(parse_present)
        .collect::<Result<Vec<Present>, String>>()
        .unwrap_or_else(|err| panic!("{}", err));

    let wrapping_paper: u32 = presents.iter()
        .map(required_wrapping_paper)
        .sum();

    println!("The elves need to order {} square feet of wrapping paper", wrapping_paper);

    let ribbon: u32 = presents.iter()
        .map(required_ribbon)
        .sum();
        
    println!("The elves need to order {} feet of ribbon", ribbon);
}