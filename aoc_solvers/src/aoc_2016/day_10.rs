use std::{hash::Hash, collections::VecDeque};

use ahash::{HashMap, HashMapExt};
use aoc_lib::{parsing::{ParseError, TextParser, parse_lines}, math::min_max, iteration::queue::{Dedupable, FoldState}};
use aoc_runner_api::SolverResult;
use itertools::{Itertools, Either};
use nom::{sequence::{preceded, delimited, tuple, terminated}, bytes::complete::tag, character::complete::u16, Parser};
use thiserror::Error;
use tupletools::Fst;

#[derive(Clone, Copy)]
enum Destination {
    Output(u16),
    Bot(u16)
}

enum Instruction {
    Forward(Forward),
    Compare(Comparison)
}

struct Forward {
    destination: Destination,
    value: u16
}

struct Comparison {
    source: u16,
    high: Destination,
    low: Destination
}

fn parse_instruction(input: &str) -> Result<Instruction, ParseError> {
    let bot = || preceded(tag("bot "), u16);
    let output = || preceded(tag("output "), u16).map(Destination::Output);
    let destination = || bot().map(Destination::Bot).or(output());

    let value = delimited(tag("value "), u16, tag(" goes to "));
    let forward = value.and(destination()).map(|(value, destination)| {
        Instruction::Forward(Forward { destination, value })
    });

    let comparison = tuple((
        terminated(bot(), tag(" gives low to ")),
        terminated(destination(), tag(" and high to ")),
        destination()
    )).map(|(source, low, high)| {
        Instruction::Compare(Comparison { source, high, low })
    });

    comparison.or(forward).run(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Chips {
    None,
    Single(u16),
    Double((u16, u16))
}

#[derive(Clone, Copy)]
struct Robot {
    id: u16,
    values: Chips,
    high: Destination,
    low: Destination
}

impl Eq for Robot {}
impl PartialEq for Robot {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.values == other.values
    }
}

impl Hash for Robot {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.values.hash(state);
    }
}

impl Robot {
    fn give(&mut self, value: u16) -> Result<(), RobotError> {
        self.values = match self.values {
            Chips::None => Chips::Single(value),
            Chips::Single(first) => Chips::Double(min_max(first, value)),
            Chips::Double(..) => return Err(RobotError::TooManyChips)
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
    outputs: HashMap<u16, u16>
}

impl State {
    fn transfer_chip(
        &mut self,
        Forward { destination, value }: Forward
    ) -> Result<Option<Robot>, RobotError> {
        Ok(match destination {
            Destination::Output(bin) => {
                match self.outputs.insert(bin, value) {
                    Some(..) => Err(RobotError::DuplicateOutput(bin))?,
                    None => None
                }
            },
            Destination::Bot(robot_number) => {
                let robot = self.robots.get_mut(&robot_number)
                    .ok_or(RobotError::InvalidRobot(robot_number))?;
                robot.give(value)?;
                Some(*robot)
            }
        })
    }

    fn transfer_chips(
        self: &mut State,
        forwards: impl IntoIterator<Item=Forward>
    ) -> Result<impl Iterator<Item=Robot>, RobotError> {
        Ok(forwards.into_iter()
            .map(|forward| self.transfer_chip(forward))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten())
    }
}

fn solve(instructions: Vec<Instruction>) -> Result<State, RobotError> {
    let (comparisons, forwards): (Vec<_>, Vec<_>) = instructions.into_iter()
        .partition_map(|instruction| match instruction {
            Instruction::Compare(instruction) => Either::Left(instruction),
            Instruction::Forward(instruction) => Either::Right(instruction)
        });
    
    let robots = comparisons.into_iter()
        .map(|Comparison { source, high, low}| {
            (source, Robot { values: Chips::None, high, low, id: source })
        }).collect();

    let mut state = State {
        outputs: HashMap::<u16, u16>::new(),
        robots
    };

    state.transfer_chips(forwards)?
        .collect::<VecDeque<_>>()
        .filter_duplicates()
        .try_recursive_fold(state, |mut state: State, robot: Robot| {
            if let Chips::Double((low_value, high_value)) = robot.values {
                let Robot { low, high, .. } = robot;

                let branches = state.transfer_chips([
                    Forward { destination: low, value: low_value },
                    Forward { destination: high, value: high_value }
                ])?;

                Ok(FoldState::Branch(state, branches))
            } else { Ok(FoldState::Leaf(state)) }
        })
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let instructions = parse_lines(parse_instruction, input)?;
    let state = solve(instructions)?;
    let robot = state.robots.into_iter()
        .find(|(_, robot)| robot.values == Chips::Double((17, 61)))
        .ok_or(RobotError::NoComparison)?.fst();


    Ok(Box::new(robot))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let instructions = parse_lines(parse_instruction, input)?;
    let outputs = solve(instructions)?.outputs;

    let output: u16 = [0, 1, 2].into_iter()
        .map(|bin| outputs.get(&bin).copied())
        .collect::<Option<Vec<u16>>>()
        .ok_or(RobotError::MissingOutput)?
        .into_iter()
        .product();

    Ok(Box::new(output))
}