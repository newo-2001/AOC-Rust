use std::{error::Error, fs};

use itertools::Itertools;

type Password = Vec<char>;

fn inc(c: char) -> char {
    (c as u8 + 1) as char
}

fn increment_digit(digit: &mut char) -> bool {
    let rollover = *digit == 'z';
    if rollover { *digit = 'a' } else { *digit = inc(*digit)};
    rollover
}

fn increment(password: &mut Password) {
    for c in password.iter_mut().rev() {
        if !increment_digit(c) { break }
    }
}

fn is_increasing_straight(seq: (&char, &char, &char)) -> bool {
    let (&a, &b, &c) = seq;
    inc(a) == b && inc(b) == c
}

fn has_increasing_straight(password: &Password) -> bool {
    password.iter().tuple_windows().any(is_increasing_straight)
}

fn has_confusing_letters(password: &Password) -> bool {
    password.iter().any(|&c| "iol".contains(c))
}

fn non_overlapping_pairs(password: &Password) -> u8 {
    let mut it = password.iter().tuple_windows();
    let mut pairs: Vec<char> = vec![];

    while let Some((a, b)) = it.next() {
        if a == b {
            pairs.push(*a);
            it.next();
        }
    }

    pairs.iter().unique().count() as u8
}

fn is_valid(password: &Password) -> bool {
    has_increasing_straight(password) &&
    !has_confusing_letters(password) &&
    non_overlapping_pairs(password) >= 2
}

fn change_password(password: &mut Password) {
    loop {
        increment(password);
        if is_valid(&password) { break }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut password: Password = fs::read_to_string("inputs/2015/day_11.txt")?
        .chars().collect();
    
    change_password(&mut password);
    println!("The new password is {}", password.iter().collect::<String>());

    change_password(&mut password);
    println!("The next password is {}", password.iter().collect::<String>());

    Ok(())
}