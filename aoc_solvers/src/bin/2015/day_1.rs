use std::error::Error;

use aoc_lib::io::read_puzzle_input;

fn parse_floor(char: char) -> Result<i32, String> {
    match char {
        '(' => Ok(1),
        ')' => Ok(-1),
        c => Err(format!("Failed to parse floor: {}", c))
    }
}

fn find_basement<'a>(directions: impl Iterator<Item=&'a i32>) -> Option<usize> {
    let mut floor: i32 = 0;
    for (i, direction) in directions.enumerate() {
        floor += direction;
        if floor == -1 {
            return Some(i);
        }
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let directions: Vec<i32> = read_puzzle_input(2015, 1)?
        .chars()
        .map(parse_floor)
        .collect::<Result<Vec<i32>, String>>()?;

    let destination: i32 = directions.iter().sum();
    println!("Santa has to deliver the presents to floor {}", destination);

    let index = find_basement(directions.iter())
        .ok_or("Santa never enters the basement")?;

    println!("Santa enter the basement at position {}", index + 1);

    Ok(())
}