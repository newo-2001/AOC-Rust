use std::{collections::HashMap, cmp::Ordering, iter};

use aoc_lib::parsing::{parse_lines, Runnable};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{character::{is_alphabetic, complete::{self, alpha1}}, bytes::complete::take_while1, sequence::{delimited, tuple}, Parser};
use tupletools::fst;

struct Room<'a> {
    name: String,
    sector_id: u32,
    checksum: &'a str
}

impl<'a> Room<'a> {
    fn letter_frequencies(&self) -> HashMap<char, u16> {
        let mut frequencies: HashMap<char, u16> = HashMap::new();
        let chars = self.name.chars()
            .into_iter()
            .filter(|&c| c != '-');

        for c in chars {
            let frequency = frequencies.get(&c)
                .unwrap_or(&0) + 1;
            frequencies.insert(c, frequency);
        }

        frequencies
    }

    fn is_real(&self) -> bool {
        fn compare_letters(left: &(char, u16), right: &(char, u16)) -> Ordering {
            match left.1.cmp(&right.1) {
                Ordering::Equal => left.0.cmp(&right.0),
                ordering => ordering.reverse()
            }
        }

        self.letter_frequencies()
            .into_iter()
            .sorted_by(compare_letters)
            .take(self.checksum.len())
            .map(fst)
            .eq(self.checksum.chars())
    }

    fn parse(input: &str) -> Result<Room, String> {
        let name = take_while1(|c| is_alphabetic(c as u8) || c == '-');
        let sector_id = complete::u32;
        let checksum = delimited(complete::char('['), alpha1, complete::char(']'));

        tuple((name, sector_id, checksum))
            .map(|(name, sector_id, checksum)| Room { name: String::from(name), sector_id, checksum})
            .run(input)
    }
}

fn rotate_letter(c: char) -> char {
    match c {
        'z' => 'a',
        c => (c as u8 + 1) as char,
    }
}

fn rotate_str(str: &str) -> String {
    str.chars()
        .into_iter()
        .map(rotate_letter)
        .collect::<String>()
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
            room.name = iter::repeat(())
                .take(room.sector_id as usize)
                .fold(room.name.to_owned(), |name, _| rotate_str(&name))
        }).find(|room| room.name.contains("northpole"))
        .ok_or("Could not find north pole")?;
    
    Ok(Box::new(north_pole.sector_id))
}