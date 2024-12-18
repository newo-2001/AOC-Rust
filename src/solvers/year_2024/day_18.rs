use std::{collections::VecDeque, iter::once};

use ahash::{HashSet, HashSetExt};
use nom::{character::complete::{char, u16}, sequence::separated_pair, Parser};
use yuki::{errors::NoSolution, parsing::{combinators::lines, ParserExt, ParsingResult}, spatial::{area::Area, direction, Point}};

use crate::SolverResult;

fn parse_byte(input: &str) -> ParsingResult<Point<usize>> {
    separated_pair(
        u16,
        char(','),
        u16
    )
    .map(|(x, y)| Point { x: x.into(), y: y.into() })
    .parse(input)
}

#[derive(Debug, Clone, Copy)]
struct State {
    position: Point<usize>,
    steps: usize
}

fn minimal_steps<I>(bytes: I, from: Point<usize>, to: Point<usize>, area: Area) -> Option<usize> where
    I: IntoIterator<Item=Point<usize>>
{
    let grid: HashSet<Point<usize>> = bytes.into_iter().collect();
    let mut seen = HashSet::<Point<usize>>::new();
    let mut queue: VecDeque<State> = once(State { position: from, steps: 0 }).collect();

    while let Some(state) = queue.pop_front() {
        if !seen.insert(state.position) { continue; }
        if state.position == to { return Some(state.steps); }

        state.position
            .neighbours::<direction::Cardinal>()
            .filter(|&pos| area.contains(pos) && !grid.contains(&pos))
            .map(|position| State { position, steps: state.steps + 1 })
            .collect_into(&mut queue);
    }

    None
}

const DIMENSIONS: usize = 70;

pub fn solve_part_1(input: &str) -> SolverResult {
    let bytes = lines(parse_byte).run(input)?
        .into_iter()
        .take(1024);

    let area = Area::from_dimensions(DIMENSIONS + 1, DIMENSIONS + 1);
    let steps = minimal_steps(bytes, Point::zero(), Point::new(DIMENSIONS, DIMENSIONS), area)
        .ok_or(NoSolution)?;

    Ok(Box::new(steps))
}