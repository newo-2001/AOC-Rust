use std::{collections::HashMap, cmp::Ordering};

use aoc_lib::{parsing::{parse_lines, TextParser, ParseError, square_brackets}, functional::repeat_apply};
use yuki::{errors::NoSolution, tuples::fst};
use crate::SolverResult;
use itertools::Itertools;
use nom::{character::{is_alphabetic, complete::{alpha1, u32}}, bytes::complete::take_while1, sequence::tuple, Parser};

struct Room<'a> {
    name: String,
    sector_id: u32,
    checksum: &'a str
}

impl Room<'_> {
    fn letter_frequencies(&self) -> HashMap<char, usize> {
        self.name
            .chars()
            .filter(|&c| c != '-')
            .counts()
    }

    fn is_real(&self) -> bool {
        fn compare_letters(left: &(char, usize), right: &(char, usize)) -> Ordering {
            match left.1.cmp(&right.1) {
                Ordering::Equal => left.0.cmp(&right.0),
                ordering => ordering.reverse()
            }
        }

        self
            .letter_frequencies()
            .into_iter()
            .sorted_unstable_by(compare_letters)
            .take(self.checksum.len())
            .map(fst)
            .eq(self.checksum.chars())
    }

    fn parse(input: &str) -> Result<Room, ParseError> {
        let name = take_while1(|c| is_alphabetic(c as u8) || c == '-');
        let checksum = square_brackets(alpha1);

        tuple((name, u32, checksum))
            .map(|(name, sector_id, checksum)| Room { name: String::from(name), sector_id, checksum})
            .run(input)
    }
}

const fn rotate_letter(c: char) -> char {
    if c == 'z' { 'a' }
    else { (c as u8 + 1) as char }
}

fn rotate_str(str: impl AsRef<str>) -> String {
    str
        .as_ref()
        .chars()
        .map(rotate_letter)
        .collect()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let sum_real_sector_ids: u32 = parse_lines(Room::parse, input)?
        .into_iter()
        .filter(Room::is_real)
        .map(|room| room.sector_id)
        .sum();

    Ok(Box::new(sum_real_sector_ids))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let north_pole = parse_lines(Room::parse, input)?
        .into_iter()
        .filter(Room::is_real)
        .update(|room| {
            room.name = repeat_apply(room.sector_id, room.name.clone(), rotate_str);
        })
        .find(|room| room.name.contains("northpole"))
        .ok_or(NoSolution)?;
    
    Ok(Box::new(north_pole.sector_id))
}