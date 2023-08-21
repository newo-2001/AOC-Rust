use std::collections::HashSet;

use aoc_lib::{parsing::{run, TextParserResult}, geometry::{Point2D, CardinalDirection, RotationDirection}};
use aoc_runner_api::SolverResult;
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

fn path_to_end<'a>(from: Point2D<i32>, instructions: impl IntoIterator<Item=&'a Instruction>) -> Vec<Point2D<i32>> {
    let initial_state = State {
        position: from,
        facing: CardinalDirection::North
    };

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

pub fn solve_part_1(input: &str) -> SolverResult {
    let instructions = parse_instructions(input)?;
    let start = Point2D::zero();
    let path = path_to_end(start, &instructions);

    let destination = path.last().unwrap_or(&start);
    let distance = start.manhattan_distance(destination) as u32;
    Ok(Box::from(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let instructions = parse_instructions(input)?;
    let start = Point2D::zero();
    let path = path_to_end(start, &instructions);
    
    let mut seen = HashSet::<Point2D<i32>>::new();
    let first_duplicate = path.into_iter()
        .skip_while(|&location| seen.insert(location))
        .next().ok_or("No location is visited twice")?;

    let distance = start.manhattan_distance(&first_duplicate);
    Ok(Box::from(distance))
}