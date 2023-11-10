use ahash::{HashSet, HashSetExt};
use aoc_lib::{parsing::{TextParserResult, TextParser, ParseError}, geometry::{Point2D, CardinalDirection, RotationDirection, Directional}, errors::NoSolution};
use aoc_runner_api::SolverResult;
use nom::{Parser, character::complete::{char, u16}, combinator::value, multi::separated_list0, bytes::complete::tag};

struct Instruction {
    direction: RotationDirection,
    amount: u16
}

#[derive(Clone)]
struct State {
    facing: CardinalDirection,
    position: Point2D<i32>
}

impl State {
    fn apply_instruction(&mut self, instruction: &Instruction) {
        self.facing = self.facing.rotate(instruction.direction);
        
        let direction: Point2D<i32> = self.facing.direction_vector();
        self.position += direction * i32::from(instruction.amount);
    }
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, ParseError> {
    fn parse_instruction(input: &str) -> TextParserResult<Instruction> {
        let left = value(RotationDirection::Left, char('L'));
        let right = value(RotationDirection::Right, char('R'));
        let direction = left.or(right);
        
        direction.and(u16)
            .map(|(direction, amount)| Instruction { direction, amount })
            .parse(input)
    }

    separated_list0(tag(", "), parse_instruction)
        .run(input)
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
            
            let steps: Vec<Point2D<i32>> = (1..=instruction.amount)
                .map(|offset| original_position + state.facing.direction_vector() * i32::from(offset))
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
    let distance = start.manhattan_distance(*destination);
    Ok(Box::from(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let instructions = parse_instructions(input)?;
    let start = Point2D::zero();
    let path = path_to_end(start, &instructions);
    
    let mut seen = HashSet::<Point2D<i32>>::new();
    let first_duplicate = path.into_iter()
        .find(|&location| !seen.insert(location))
        .ok_or(NoSolution)?;

    let distance = start.manhattan_distance(first_duplicate);
    Ok(Box::from(distance))
}