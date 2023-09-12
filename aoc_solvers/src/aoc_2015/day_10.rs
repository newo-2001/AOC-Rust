use std::error::Error;

use aoc_lib::{functional::repeat_apply, parsing::InvalidTokenError};
use aoc_runner_api::SolverResult;
use itertools::Itertools;

fn look_and_say(look: &[u8]) -> Vec<u8> {
    let mut say = look.iter()
        .tuple_windows::<(&u8, &u8)>()
        .fold(vec![1], |mut acc: Vec<u8>, (prev, current)| {
            if prev == current {
                *acc.last_mut().unwrap() += 1;
            } else {
                acc.push(*prev);
                acc.push(1);
            }

            acc
        });

    say.push(*look.last().expect("No number to say"));
    say
}

fn serialize(number: &[u8]) -> String {
    number.iter().map(|c| (c + b'0') as char).collect()
}

fn parse_seed(input: &str) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    Ok(input.chars()
        .map(|c| c.to_digit(10).ok_or(InvalidTokenError(c)))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(u8::try_from)
        .collect::<Result<Vec<_>, _>>()?)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let seed = parse_seed(input)?;
    let seed = repeat_apply(40, seed, |seq| look_and_say(&seq));
    let length = serialize(&seed).len();

    Ok(Box::new(length))
}


pub fn solve_part_2(input: &str) -> SolverResult {
    let seed = parse_seed(input)?;
    let seed = repeat_apply(50, seed, |seq| look_and_say(&seq));
    let length = serialize(&seed).len();

    Ok(Box::new(length))
}