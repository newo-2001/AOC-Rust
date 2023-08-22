use aoc_runner_api::SolverResult;
use itertools::Itertools;

fn look_and_say(look: &Vec<u8>) -> Vec<u8> {
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

fn serialize(number: &Vec<u8>) -> String {
    number.iter().map(|c| (c + '0' as u8) as char).collect()
}

fn look_and_say_rounds(seed: Vec<u8>, rounds: u8) -> Vec<u8> {
    (0..rounds).fold(seed, |acc, _| look_and_say(&acc))
}

fn parse_seed(input: &str) -> Result<Vec<u8>, &str>{
    input.chars()
        .map(|c| c.to_digit(10).map(|x| x as u8))
        .collect::<Option<Vec<u8>>>()
        .ok_or("Failed to parse digit")
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let seed = parse_seed(input)?;
    let seed = look_and_say_rounds(seed, 40);
    let length = serialize(&seed).len();

    Ok(Box::new(length))
}


pub fn solve_part_2(input: &str) -> SolverResult {
    let seed = parse_seed(input)?;
    let seed = look_and_say_rounds(seed, 50);
    let length = serialize(&seed).len();

    Ok(Box::new(length))
}