use aoc_runner_api::SolverResult;
use md5;

fn proof_of_work(zeros: usize, prefix: &str) -> u32 {
    let mut i = 0;
    let hash_prefix = "0".repeat(zeros);

    loop {
        let key = prefix.to_owned() + i.to_string().as_str();
        let hash = format!("{:x}", md5::compute(key.as_bytes()));
        if hash.starts_with(&hash_prefix) {
            break;
        }
        i += 1;
    }
    
    i   
}

pub fn solve_part_1(input: &str) -> SolverResult {
    Ok(Box::from(proof_of_work(5, input)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    Ok(Box::from(proof_of_work(6, input)))
}