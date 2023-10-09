use ahash::HashSet;
use aoc_lib::iteration::ExtraIter;
use aoc_runner_api::SolverResult;
use itertools::Itertools;

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_1(input: &str) -> SolverResult {
    let valid_passphrases = input.lines()
        .count_where(|line| line.split_whitespace().all_unique());

    Ok(Box::new(valid_passphrases))
}

fn contains_anagram<'a>(words: impl IntoIterator<Item=&'a str>) -> bool {
    let words: HashSet<&str> = words.into_iter().collect();
    words.iter().any(|word| {
        words.iter()
            .filter(|&w| w != word && w.len() == word.len())
            .flat_map(|word| word.chars().permutations(word.len()))
            .contains(&word.chars().collect_vec())
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_2(input: &str) -> SolverResult {
    let valid_passphrases = input.lines()
        .map(str::split_whitespace)
        .count_where(|passphrase| passphrase.clone().all_unique() && !contains_anagram(passphrase));

    Ok(Box::new(valid_passphrases))
}