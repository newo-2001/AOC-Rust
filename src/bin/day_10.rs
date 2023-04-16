use std::{error::Error, fs};

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut seed: Vec<u8> = fs::read_to_string("inputs/day_10.txt")?
        .chars().map(|c| c.to_digit(10).map(|x| x as u8))
        .collect::<Option<Vec<u8>>>()
        .expect("Failed to parse digit");

    seed = look_and_say_rounds(seed, 40);
    println!("The length of the result after 40 rounds is {}", serialize(&seed).len());

    seed = look_and_say_rounds(seed, 10);
    println!("The length of the result after 50 rounds is {}", serialize(&seed).len());

    Ok(())
}