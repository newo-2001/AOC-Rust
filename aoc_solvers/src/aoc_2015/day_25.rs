use aoc_lib::parsing::{skip_until, TextParser, ParseError};
use aoc_runner_api::SolverResult;
use nom::{character::complete::{u32, char}, bytes::complete::tag, sequence::{preceded, delimited}, Parser};

#[derive(Debug, PartialEq, Eq)]
struct Position(u32, u32);

impl Position {
    fn next(&self) -> Position {
        let &Position(col, row) = self;
        if row > 1 { Position(col + 1, row - 1) }
        else { Position(1, col + 1) }
    }

    fn code(&self) -> u64 {
        let mut code: u64 = 2015_1125;
        let mut position = Position(1, 1);

        while &position != self {
            position = position.next();
            code = (code * 252_533) % 33_554_393;
        }

        code
    }
}

fn parse_input(input: &str) -> Result<Position, ParseError> {
    let row = preceded(tag("row "), u32);
    let col = delimited(tag(", column "), u32, char('.'));
    
    skip_until(row.and(col))
        .map(|(row, col)| Position(col, row))
        .run(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let position = parse_input(input)?;
    Ok(Box::new(position.code()))
}