use std::{error::Error, fs};

use aoc_lib::parsing::{run, skip_until};
use nom::{character::complete, bytes::complete::tag, sequence::preceded, Parser};

#[derive(Debug, PartialEq, Eq)]
struct Position(u32, u32);

impl Position {
    fn next(&self) -> Position {
        let &Position(col, row) = self;
        if row > 1 { Position(col + 1, row - 1) }
        else { Position(1, col + 1) }
    }

    fn code(&self) -> u64 {
        let mut code: u64 = 20151125;
        let mut position = Position(1, 1);

        while &position != self {
            position = position.next();
            code = (code * 252533) % 33554393;
        }

        code
    }
}

fn parse_input(input: &str) -> Result<Position, String> {
    let row = preceded(tag("row "), complete::u32);
    let col = preceded(tag(", column "), complete::u32);
    let mut position = skip_until(row.and(col))
        .map(|(row, col)| Position(col, row));

    run(&mut position, input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/2015/day_25.txt")?;
    let position = parse_input(&content)?;

    let code = position.code();
    println!("The code is {}", code);

    Ok(())
}