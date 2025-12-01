use nom::{Parser, character::complete::{char, u32}, combinator::value};
use yuki::parsing::{Parsable, ParsingResult, parse_lines};

use crate::SolverResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    direction: Direction,
    amount: u32,
}

impl Parsable<'_> for Direction {
    fn parse(input: &str) -> ParsingResult<'_, Direction> {
        Parser::or(
            value(Direction::Left, char('L')),
            value(Direction::Right, char('R'))
        ).parse(input)
    }
}

impl Parsable<'_> for Rotation {
    fn parse(input: &str) -> ParsingResult<'_, Rotation> {
        Parser::map(
            (Direction::parse, u32),
            |(direction, amount)| Rotation { direction, amount }
        ).parse(input)
    }
}

const DIAL_LIMIT: u32 = 100;

fn rotate_dial(dial: &mut u32, rotation: Rotation) -> u32 {
    let offset: i32 = match rotation.direction {
        Direction::Left => -(rotation.amount as i32),
        Direction::Right => rotation.amount as i32
    } % (DIAL_LIMIT as i32);

    let clicks = match dial.overflowing_add_signed(offset) {
        (value, false) => {
            *dial = value % DIAL_LIMIT;
            u32::from(value >= DIAL_LIMIT || value == 0)
        },
        (overflow, true) => {
            let clicks = u32::from(*dial != 0);
            *dial = (DIAL_LIMIT - 1) - (u32::MAX - overflow);
            clicks
        }
    };

    clicks + rotation.amount / DIAL_LIMIT
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let rotations: Vec<Rotation> = parse_lines(input)?;
    let zeros = rotations
        .into_iter()
        .scan(DIAL_LIMIT / 2, |dial, rotation| {
            rotate_dial(dial, rotation);
            Some(*dial)
        })
        .filter(|&x| x == 0)
        .count();

    Ok(Box::new(zeros))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let rotations: Vec<Rotation> = parse_lines(input)?;
    let clicks: u32 = rotations
        .into_iter()
        .scan(DIAL_LIMIT / 2, |dial, rotation| Some(rotate_dial(dial, rotation)))
        .sum();

    Ok(Box::new(clicks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_dial() {
        let mut dial = 3;
        assert_eq!(2, rotate_dial(&mut dial, Rotation { amount: 103, direction: Direction::Left }));

        dial = 0;
        assert_eq!(0, rotate_dial(&mut dial, Rotation { amount: 1, direction: Direction::Left }));
    }
}