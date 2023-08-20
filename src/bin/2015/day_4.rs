use std::error::Error;

use aoc_lib::io::read_puzzle_input;
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

fn main() -> Result<(), Box<dyn Error>> {
    let prefix = read_puzzle_input(2015, 4)?;
    
    println!("The first suffix to produce 5 leading zeros is {}", proof_of_work(5, &prefix));
    println!("The first suffix to produce 6 leading zeros is {}", proof_of_work(6, &prefix));

    Ok(())
}