use aoc_lib::{geometry::{CardinalDirection, Point2D, Directional}, parsing::{parse_lines, TextParser, ParseError}};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::multi::many0;
use thiserror::Error;

#[derive(Clone, Copy)]
enum KeyPad {
    Square,
    Round
}

#[derive(Debug, Error)]
#[error("Invalid location on the keypad `{0}`")]
struct InvalidKeyError(Point2D<i32>);

fn parse_instruction(line: &str) -> Result<Vec<CardinalDirection>, ParseError> {
    many0(CardinalDirection::parse).run(line)
}

fn digit<'a>(keypad: KeyPad, location: &mut Point2D<i32>, movements: impl IntoIterator<Item=&'a CardinalDirection>) -> Result<char, InvalidKeyError> {
    for movement in movements {
        let offset: Point2D<i32> = movement.direction_vector();
        let new_location = *location + offset;

        *location = match keypad {
            KeyPad::Square => new_location.clamp(-1, 1),
            KeyPad::Round => match new_location.manhattan_distance(&Point2D::zero()) {
                distance if distance <= 2 => new_location,
                _ => *location
            }
        }
    }

    match (keypad, *location) { 
        (KeyPad::Square, Point2D(x, y)) => {
            let digit = u32::try_from((y + 1) * 3 + x + 2)
                .map_err(|_| InvalidKeyError(*location))?;

            char::from_digit(digit, 10).ok_or(InvalidKeyError(*location))
        },
        // Ideally we would use mapping from Point2D to the index on the round keypad
        // Then the radix argument to char::from_digit() could be used to support keypads of arbitrary size
        // But alas, I am not mathematically literate enough
        (KeyPad::Round, Point2D(x, y)) => {
            let index = usize::try_from((y + 2) * 5 + x + 2)
                .map_err(|_| InvalidKeyError(*location))?;

            b"..1...234.56789.ABC...D....".get(index)
                .map(|&c| c as char)
                .ok_or(InvalidKeyError(*location))
        }
    }
}

fn compute_code<'a>(keypad: KeyPad, instructions: impl IntoIterator<Item=&'a Vec<CardinalDirection>>) -> Result<String, InvalidKeyError> {
    let mut location = match keypad {
        KeyPad::Square => Point2D::zero(),
        KeyPad::Round => Point2D(-2, 0)
    };
    
    instructions.into_iter()
        .map(|movements| digit(keypad, &mut location, movements))
        .try_collect()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let instructions = parse_lines(parse_instruction, input)?;
    let code = compute_code(KeyPad::Square, &instructions)?;
    Ok(Box::new(code))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let instructions = parse_lines(parse_instruction, input)?;
    let code = compute_code(KeyPad::Round, &instructions)?;
    Ok(Box::new(code))
}