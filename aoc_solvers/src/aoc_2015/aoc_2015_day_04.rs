use aoc_lib::math::natural_numbers;
use aoc_runner_api::SolverResult;
use hex::ToHex;
use md5;

fn proof_of_work(zeroes: usize, prefix: &str) -> u32 {
    let hash_prefix = "0".repeat(zeroes);

    natural_numbers::<u32>(0)
        .find_map(|index| {
            let key = format!("{}{}", prefix, index);
            let hash: String = md5::compute(key).encode_hex();
            hash.starts_with(&hash_prefix).then_some(index)
        }).unwrap()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    Ok(Box::from(proof_of_work(5, input)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    Ok(Box::from(proof_of_work(6, input)))
}