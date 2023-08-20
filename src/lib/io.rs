use std::{fs, io};

pub fn read_puzzle_input(year: u32, day: u32) -> io::Result<String> {
    fs::read_to_string(format!("inputs/{}/day_{}.txt", year, day))
}

pub fn read_example_input() -> io::Result<String> {
    fs::read_to_string("inputs/example.txt")
}