use std::collections::HashSet;
use aoc_runner_api::SolverResult;
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

pub fn solve_part_1(input: &str) -> SolverResult {
    let nice_words = input.lines()
        .filter(|&word| is_nice(word))
        .count();

    Ok(Box::new(nice_words))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let nice_words = input.lines()
        .filter(|word| real_is_nice(word))
        .count();

    Ok(Box::new(nice_words))
}