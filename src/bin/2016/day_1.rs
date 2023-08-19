use std::{error::Error, fs, collections::HashSet};

use aoc_lib::{parsing::{run, TextParseResult}, geometry::{Point2D, CardinalDirection, RotationDirection}};
use itertools::{Itertools, FoldWhile::{Continue, Done}};
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
    fn after_instruction(&self, instruction: &Instruction) -> State {
        let facing = self.facing.rotate(instruction.direction);

        State {
            facing,
            position: self.position + facing.unit_vector() * instruction.amount as i32
        }
    }
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    fn parse_instruction(input: &str) -> TextParseResult<Instruction> {
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

fn distance_to_end<'a>(initial_state: State, instructions: impl IntoIterator<Item=&'a Instruction>) -> u32 {
    let final_state = instructions.into_iter()
        .fold(initial_state.clone(), |state, instruction| State::after_instruction(&state, instruction));

    initial_state.position.manhattan_distance(final_state.position) as u32
}

fn distance_to_first_duplicate<'a>(initial_state: State, instructions: impl IntoIterator<Item=&'a Instruction>) -> u32 {
    let mut seen = HashSet::<Point2D<i32>>::new();
    
    let final_state = instructions.into_iter()
        .fold_while(initial_state.clone(), |state, instruction| {
            let new_state = state.after_instruction(instruction);
            let visited = (1..instruction.amount + 1)
                .map(|offset| state.position + new_state.facing.unit_vector() * offset as i32);

            for position in visited {
                if !seen.insert(position) {
                    return Done(State { position, facing: new_state.facing })
                }
            }

            Continue(new_state)
        }).into_inner();

    initial_state.position.manhattan_distance(final_state.position) as u32
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("inputs/2016/day_1.txt")?;
    let instructions = parse_instructions(&contents)?;

    let initial_state = State {
        position: Point2D::zero(),
        facing: CardinalDirection::North
    };

    let distance = distance_to_end(initial_state.clone(), &instructions);
    println!("The distance from the start to the destination is {}", distance);

    let distance = distance_to_first_duplicate(initial_state, &instructions);
    println!("The distance to the first location your visit twice is {}", distance);

    Ok(())
}