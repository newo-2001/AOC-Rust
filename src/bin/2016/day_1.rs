use std::{error::Error, collections::HashSet};

use aoc_lib::{parsing::{run, TextParserResult}, geometry::{Point2D, CardinalDirection, RotationDirection}, io::read_puzzle_input};
use nom::{Parser, character::complete, combinator::value, multi::separated_list0, bytes::complete::tag};

struct Instruction {
    direction: RotationDirection,
    amount: u32
}

#[derive(Clone)]
struct State {
    facing: CardinalDirection,
    position: Point2D<i32>
}

impl State {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        self.facing = self.facing.rotate(instruction.direction);
        self.position += self.facing.direction_vector() * instruction.amount as i32
    }
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    fn parse_instruction(input: &str) -> TextParserResult<Instruction> {
        let left = value(RotationDirection::Left, complete::char('L'));
        let right = value(RotationDirection::Right, complete::char('R'));
        let direction = left.or(right);
        
        direction.and(complete::u32)
            .map(|(direction, amount)| Instruction { direction, amount })
            .parse(input)
    }

    let mut instructions = separated_list0(tag(", "), parse_instruction);
    run(&mut instructions, input)
}

fn path_to_end<'a>(initial_state: State, instructions: impl IntoIterator<Item=&'a Instruction>) -> Vec<Point2D<i32>> {
    instructions.into_iter()
        .scan(initial_state, |state: &mut State, instruction: &Instruction| {
            let original_position = state.position;
            state.apply_instruction(instruction);
            
            let steps: Vec<Point2D<i32>> = (1..instruction.amount + 1)
                .map(|offset| original_position + state.facing.direction_vector() * offset as i32)
                .collect();

            Some(steps)
        }).flatten()
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_puzzle_input(2016, 1)?;
    let instructions = parse_instructions(&contents)?;

    let initial_state = State {
        position: Point2D::zero(),
        facing: CardinalDirection::North
    };

    let path = path_to_end(initial_state.clone(), &instructions);
    let destination = path.last().unwrap_or(&initial_state.position);
    let distance = initial_state.position.manhattan_distance(destination) as u32;
    println!("The distance from the start to the destination is {}", distance);

    let mut seen = HashSet::<Point2D<i32>>::new();
    let first_duplicate = path.into_iter()
        .skip_while(|&location| seen.insert(location))
        .next().ok_or("No location is visited twice")?;

    let distance = initial_state.position.manhattan_distance(&first_duplicate);
    println!("The distance to the first location your visit twice is {}", distance);

    Ok(())
}