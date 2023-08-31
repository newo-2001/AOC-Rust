use std::{collections::{HashMap, BTreeSet}, cmp::{min, max}};

use aoc_lib::parsing::{ParseError, Runnable, parse_lines};
use aoc_runner_api::SolverResult;
use itertools::{Itertools, Either};
use nom::{sequence::{preceded, delimited, tuple, terminated}, bytes::complete::tag, character::complete, Parser};
use thiserror::Error;

#[derive(Clone, Copy)]
enum Destination {
    Output(u16),
    Bot(u16)
}

enum Instruction {
    Unconditional(UnconditionalInstruction),
    Conditional(ConditionalInstruction)
}

struct UnconditionalInstruction {
    destination: Destination,
    value: u16
}

struct ConditionalInstruction {
    source: u16,
    high: Destination,
    low: Destination
}

fn parse_instruction(input: &str) -> Result<Instruction, ParseError> {
    let bot = || preceded(tag("bot "), complete::u16);
    let output = || preceded(tag("output "), complete::u16).map(Destination::Output);
    let destination = || bot().map(Destination::Bot).or(output());

    let value = delimited(tag("value "), complete::u16, tag(" goes to "));
    let unconditional = value.and(destination()).map(|(value, destination)| {
        Instruction::Unconditional(UnconditionalInstruction { destination, value })
    });

    let conditional = tuple((
        terminated(bot(), tag(" gives low to ")),
        terminated(destination(), tag(" and high to ")),
        destination()
    )).map(|(source, low, high)| {
        Instruction::Conditional(ConditionalInstruction { source, low, high })
    });

    conditional.or(unconditional).run(input)
}

#[derive(Clone, Copy)]
struct Robot {
    values: Option<(u16, Option<u16>)>,
    high: Destination,
    low: Destination
}

impl Robot {
    fn give(&mut self, value: u16) -> Result<(), RobotError> {
        self.values = match self.values {
            None => Some((value, None)),
            Some((first, None)) => Some((min(first, value), Some(max(first, value)))),
            Some((_, Some(_))) => return Err(RobotError::TooManyChips)
        };

        Ok(())
    }
}

#[derive(Debug, Error)]
enum RobotError {
    #[error("There is no robot with number `{0}`")]
    InvalidRobot(u16),
    #[error("Robot recieved more than two chips")]
    TooManyChips,
    #[error("The two chips are never compared")]
    NoComparison,
    #[error("One or more of the required output bins is never filled")]
    MissingOutput,
    #[error("Output bin `{0}` recieved more than one chip")]
    DuplicateOutput(u16)
}

struct State {
    robots: HashMap<u16, Robot>,
    outputs: HashMap<u16, u16>,
    queue: BTreeSet<u16>
}

fn transfer_chip(
    state: &mut State, 
    UnconditionalInstruction { value, destination }: UnconditionalInstruction
) -> Result<(), RobotError> {
    match destination {
        Destination::Output(bin) => {
            state.outputs.insert(bin, value)
                .map_or(Ok(()), |_| Err(RobotError::DuplicateOutput(bin)))
        },
        Destination::Bot(robot_number) => {
            let robot = state.robots.get_mut(&robot_number)
                .ok_or(RobotError::InvalidRobot(robot_number))?;
            robot.give(value)?;
            state.queue.insert(robot_number);
            Ok(())
        }
    }
}

fn compute_output(outputs: &HashMap<u16, u16>) -> Result<u16, RobotError> {
    const BINS: [u16; 3] = [0, 1, 2];
    let output = BINS.into_iter()
        .map(|bin| outputs.get(&bin).map(|x| *x))
        .collect::<Option<Box<[u16]>>>()
        .ok_or(RobotError::MissingOutput)?
        .into_iter()
        .product();

    Ok(output)
}

fn solve(needle: Option<(u16, u16)>, instructions: Vec<Instruction>) -> Result<u16, RobotError> {
    let (conditional, unconditional): (Vec<_>, Vec<_>) = instructions.into_iter()
        .partition_map(|instruction| match instruction {
            Instruction::Conditional(instruction) => Either::Left(instruction),
            Instruction::Unconditional(instruction) => Either::Right(instruction)
        });
    
    let mut state = State {
        queue: BTreeSet::<u16>::new(),
        outputs: HashMap::<u16, u16>::new(),
        robots: HashMap::<u16, Robot>::from_iter(
            conditional.into_iter().map(|ConditionalInstruction { source, high, low}| {
                (source, Robot { values: None, high, low })
            })
        )
    };

    unconditional.into_iter()
        .map(|instruction| transfer_chip(&mut state, instruction))
        .collect::<Result<Vec<()>, RobotError>>()?;

    while let Some(robot_number) = state.queue.pop_first() {
        let robot = state.robots.get(&robot_number)
            .ok_or(RobotError::InvalidRobot(robot_number))?;

        if let Some((low_value, Some(high_value))) = robot.values {
            if Some((low_value, high_value)) == needle { return Ok(robot_number) }
            let Robot { low, high, .. } = *robot;

            transfer_chip(&mut state, UnconditionalInstruction { destination: low, value: low_value })?;
            transfer_chip(&mut state, UnconditionalInstruction { destination: high, value: high_value })?;
        }
    }

    if needle.is_none() { compute_output(&state.outputs) }
    else { Err(RobotError::NoComparison) }

}

pub fn solve_part_1(input: &str) -> SolverResult {
    let instructions = parse_lines(parse_instruction, input)?;
    let robot = solve(Some((17, 61)), instructions)?;
    Ok(Box::new(robot))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let instructions = parse_lines(parse_instruction, input)?;
    let output = solve(None, instructions)?;
    Ok(Box::new(output))
}