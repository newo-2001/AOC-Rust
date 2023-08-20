use std::error::Error;

use aoc_lib::{geometry::{CardinalDirection, Point2D}, parsing::{run, direction}, io::read_puzzle_input};
use nom::multi::many0;

enum KeyPad {
    Square,
    Round
}

fn parse_instruction(line: &str) -> Result<Vec<CardinalDirection>, String> {
    run(&mut many0(direction), line)
}

fn digit<'a>(keypad: &KeyPad, location: &mut Point2D<i32>, movements: impl IntoIterator<Item=&'a CardinalDirection>) -> char {
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
        (KeyPad::Square, Point2D(x, y)) => char::from_digit(((y + 1) * 3 + x + 1) as u32, 10)
            .expect("Failed to parse location on the keypad"),
        // Ideally we would use mapping from Point2D to the index on the round keypad
        // Then the radix argument to char::from_digit() could be used to support keypads of arbitrary size
        // But alas, I am not mathematically literate enough
        (KeyPad::Round, Point2D(x, y)) => "..1...234.56789.ABC...D...".as_bytes()[((y + 2) * 5 + x + 2) as usize] as char
    }
}

fn compute_code<'a>(keypad: KeyPad, instructions: impl IntoIterator<Item=&'a Vec<CardinalDirection>>) -> String {
    let mut location = match keypad {
        KeyPad::Square => Point2D::zero(),
        KeyPad::Round => Point2D(-2, 0)
    };
    
    instructions.into_iter()
        .map(|movements| digit(&keypad, &mut location, movements))
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = read_puzzle_input(2016, 2)?
        .lines()
        .map(parse_instruction)
        .collect::<Result<Vec<Vec<CardinalDirection>>, String>>()?;

    let code = compute_code(KeyPad::Square, &instructions);
    println!("The code for the square keypad is: {}", code);

    let code = compute_code(KeyPad::Round, &instructions);
    println!("The code for the round keypad is {}", code);

    Ok(())
}