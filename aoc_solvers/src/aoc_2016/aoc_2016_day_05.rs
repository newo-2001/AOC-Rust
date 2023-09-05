use aoc_runner_api::SolverResult;
use hex::ToHex;
use itertools::Itertools;
use tupletools::snd;

fn password<'a, T>(door_id: &'a str, extractor: impl Fn(&str) -> Option<T> + 'a) -> impl Iterator<Item=T> + 'a {
    (0..).filter_map(move |index| {
        let data = format!("{}{}", door_id, index);
        let digest: String = md5::compute(data).encode_hex();
        digest.starts_with("00000")
            .then(|| extractor(&digest))
            .flatten()
    })
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let extractor = |hash: &str| hash.chars().nth(5);

    let password: String = password(input, extractor)
        .take(8)
        .collect();

    Ok(Box::new(password))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    fn extractor(hash: &str) -> Option<(usize, char)> {
        let mut chars = hash.chars();
        let index = chars.nth(5)?.to_digit(8)? as usize;
        let char = chars.nth(0)?;
        Some((index, char))
    }

    let password: String = password(input, extractor)
        .unique_by(|&(index, _)| index)
        .into_iter()
        .take(8)
        .sorted_unstable_by_key(|&(index, _)| index)
        .map(snd)
        .collect();

    Ok(Box::new(password))
}