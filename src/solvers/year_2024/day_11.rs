use std::{array, iter::{once, Once}};

use ahash::{HashMap, HashMapExt};
use itertools::Either;
use nom::{character::complete::{space1, u64}, multi::separated_list0, Parser};
use num::Integer;
use recursive::recursive;
use yuki::parsing::{ParserExt, ParsingResult};

use crate::SolverResult;

fn parse_stones(input: &str) -> ParsingResult<'_, Vec<u64>> {
    separated_list0(
        space1,
        u64
    )
    .parse(input)
}

const fn digits(n: u64) -> u32 {
    match n {
        0 => 1,
        _ => n.ilog10() + 1
    }
}

const fn split(n: u64) -> [u64; 2] {
    let mag = u64::pow(10, digits(n) / 2);
    [n / mag, n % mag]
}

fn replace_stone(stone: u64) -> Either<Once<u64>, array::IntoIter<u64, 2>> {
    match stone {
        0 => Either::Left(once(1)),
        stone if digits(stone).is_even() => Either::Right(split(stone).into_iter()),
        _ => Either::Left(once(stone * 2024))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    stone: u64,
    times: u32
}

#[recursive]
fn blink(cache: &mut HashMap<State, usize>, state: State) -> usize {
    if state.times == 0 { return 1; }
    if let Some(children) = cache.get(&state) { return *children; }

    let children = replace_stone(state.stone)
        .into_iter()
        .map(|stone| blink(cache, State { stone, times: state.times - 1 }))
        .sum();

    cache.insert(state, children);
    children
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut cache = HashMap::<State, usize>::new();
    let stones: usize = parse_stones.run(input)?
        .into_iter()
        .map(|stone| blink(&mut cache, State { stone, times: 25 }))
        .sum();

    Ok(Box::new(stones))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut cache = HashMap::<State, usize>::new();
    let stones: usize = parse_stones.run(input)?
        .into_iter()
        .map(|stone| blink(&mut cache, State { stone, times: 75 }))
        .sum();

    Ok(Box::new(stones))
}