use std::{collections::VecDeque, iter::once};

use ahash::{HashMap, HashMapExt};
use itertools::Itertools;
use nom::{character::complete::{anychar, char}, combinator::{map_opt, value}, multi::many1, Parser};
use recursive::recursive;
use yuki::{errors::NoSolution, parsing::{combinators::lines, Parsable, ParserExt, ParsingResult}, spatial::{direction::{self, Directions}, Point}};

use crate::SolverResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
    Num(u32),
    Direction(direction::Cardinal),
    Confirm
}

impl<'a> Parsable<'a> for Key {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        Parser::or(
            value(Self::Confirm, char('A')),
            map_opt(
                anychar,
                |digit| digit.to_digit(10).map(Key::Num)
            )
        )
        .parse(input)
    }
}

fn parse_code(input: &str) -> ParsingResult<'_, Vec<Key>> {
    many1(Key::parse).parse(input)
}

fn code_number(code: &[Key]) -> u32 {
    code
        .iter()
        .fold(0, |acc, digit| if let Key::Num(digit) = digit { acc * 10 + *digit } else { acc })
}

#[derive(Debug, Clone)]
struct KeyPad {
    keys: HashMap<Point<u32>, Key>,
    start: Point<u32>
}

const DPAD_START: Point<u32> = Point::new(2, 0);
const DPAD_KEYS: [(Point<u32>, Key); 5] = [
    (Point::new(1, 0), Key::Direction(direction::Cardinal::North)),
    (DPAD_START, Key::Confirm),
    (Point::new(0, 1), Key::Direction(direction::Cardinal::West)),
    (Point::new(1, 1), Key::Direction(direction::Cardinal::South)),
    (Point::new(2, 1), Key::Direction(direction::Cardinal::East))
];

const NUMPAD_START: Point<u32> = Point::new(2, 3);
const NUMPAD_KEYS: [(Point<u32>, Key); 11] = [
    (Point::new(0, 0), Key::Num(7)),
    (Point::new(1, 0), Key::Num(8)),
    (Point::new(2, 0), Key::Num(9)),
    (Point::new(0, 1), Key::Num(4)),
    (Point::new(1, 1), Key::Num(5)),
    (Point::new(2, 1), Key::Num(6)),
    (Point::new(0, 2), Key::Num(1)),
    (Point::new(1, 2), Key::Num(2)),
    (Point::new(2, 2), Key::Num(3)),
    (Point::new(1, 3), Key::Num(0)),
    (NUMPAD_START, Key::Confirm)
];

#[derive(Debug)]
struct State {
    inputs: Vec<Key>,
    position: Point<u32>
}

impl KeyPad {
    fn find_paths(&self, initial_position: Point<u32>, needle: Key) -> Vec<Vec<Key>> {
        let initial_state = State {
            inputs: Vec::new(),
            position: initial_position
        };

        let mut solutions = Vec::<Vec<Key>>::new();
        let mut queue: VecDeque<State> = once(initial_state).collect();
        let mut min_distance: Option<usize> = None;

        while let Some(state) = queue.pop_front() {
            let Some(&key) = self.keys.get(&state.position) else { continue };

            if key == needle {
                let steps = state.inputs.len();
                if steps > min_distance.unwrap_or(usize::MAX) { break; }

                let _ = min_distance.insert(steps);
                solutions.push(state.inputs);
                continue;
            }

            direction::Cardinal::all()
                .filter_map(|dir| Some((dir, state.position.add_signed(dir.vector())?)))
                .map(|(dir, position)| {
                    //println!("[{:?}] ({}, {}) -> ({}, {})", key, state.position.x, state.position.y, position.x, position.y);

                    let mut inputs = state.inputs.clone();
                    inputs.push(Key::Direction(dir));

                    State { inputs, position }
                })
                .collect_into(&mut queue);
        }

        solutions
    }

    #[recursive]
    fn solve(
        &self,
        cache: &mut HashMap<(Point<u32>, Vec<Key>), Vec<Vec<Key>>>,
        position: Point<u32>,
        code: &[Key]
    ) -> Vec<Vec<Key>> {
        let cache_key = (position, code.to_vec());
        if let Some(cached) = cache.get(&cache_key) { return cached.clone(); }

        let solutions = match code {
            [] => { vec![vec![]] }
            [head, tail @ ..] => {
                let dest = self.keys
                    .iter()
                    .find_map(|(pos, key)| (key == head).then_some(*pos))
                    .unwrap();

                let children = self.solve(cache, dest, tail);
                let start: Vec<_> = self
                    .find_paths(position, *head)
                    .into_iter()
                    .update(|path| path.push(Key::Confirm))
                    .collect();

                start
                    .into_iter()
                    .cartesian_product(children)
                    .map(|(mut start, child)| {
                        start.extend(child);
                        start
                    })
                    .collect()
            }
        };

        cache.insert(cache_key, solutions.clone());
        solutions
    }
}

fn solve(keypads: &[&KeyPad], code: Vec<Key>) -> Vec<Vec<Key>> {
    keypads
        .iter()
        .fold(vec![code], |codes, pad| {
            let mut cache = HashMap::new();

            codes
                .into_iter()
                .flat_map(|code| pad.solve(&mut cache, pad.start, &code))
                .collect()
        })
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_1(input: &str) -> SolverResult {
    let dpad = KeyPad {
        keys: DPAD_KEYS.iter().copied().collect(),
        start: DPAD_START
    };

    let numpad = KeyPad {
        keys: NUMPAD_KEYS.iter().copied().collect(),
        start: NUMPAD_START
    };

    let keypads = [
        &numpad,
        &dpad,
        &dpad
    ];

    #[allow(clippy::cast_possible_truncation)]
    let complexity: usize = lines(parse_code).run(input)?
        .into_iter()
        .map(|code| (code_number(&code), code))
        .map(|(num, code)| {
            solve(&keypads, code)
                .into_iter()
                .map(|code| code.len())
                .min()
                .map(|length| length * num as usize)
                .ok_or(NoSolution)
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum();

    Ok(Box::new(complexity))
}