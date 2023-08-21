use std::{collections::HashSet, error::Error};
use aoc_lib::io::read_puzzle_input;
use itertools::Itertools;

fn is_vowel(char: &char) -> bool {
    "aioeu".contains(*char)
}

fn count_vowels(str: &str) -> usize {
    str.chars().filter(is_vowel).count()
}

fn has_consecutive_duplicates(str: &str) -> bool {
    str.chars().dedup().count() != str.len()
}

fn has_naughty_substring(str: &str) -> bool {
    ["ab", "cd", "pq", "xy"].iter()
        .any(|x| str.contains(x))
}

fn is_nice(str: &str) -> bool {
    count_vowels(str) >= 3 &&
    has_consecutive_duplicates(str) &&
    !has_naughty_substring(str)
}

// Tf is this function??
fn has_duplicate_consecutive_duplicate(str: &str) -> bool {
    let pairs = str.chars().tuple_windows::<(char, char)>();
    let mut seen = HashSet::new();
    let mut double = false;

    for pair in pairs {
        if pair.0 == pair.1 {
            if double {
                double = false;
                continue;
            }
            double = true;
        } else {
            double = false;
        }
        
        if seen.contains(&pair) {
            return true;
        }

        seen.insert(pair);
    }

    false
}

fn has_seperated_repeating_letter(str: &str) -> bool {
    str.chars()
        .skip(1)
        .step_by(2)
        .dedup_with_count()
        .chain(str.chars().step_by(2).dedup_with_count())
        .any(|(i, _)| i > 1)
}

fn real_is_nice(str: &str) -> bool {
    has_duplicate_consecutive_duplicate(str) &&
    has_seperated_repeating_letter(str)
}

fn main() -> Result<(), Box<dyn Error>> {
    let words: Vec<String> = read_puzzle_input(2015, 5)?
        .lines()
        .map(String::from)
        .collect();

    let nice_words = words.iter()
        .filter(|&x| is_nice(x))
        .count();
    
    println!("{} string are nice", nice_words);

    let real_nice_words = words.iter()
        .filter(|&x| real_is_nice(x))
        .count();

    println!("{} string are actually nice", real_nice_words);

    Ok(())
}