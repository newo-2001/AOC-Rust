use aoc_lib::parsing::InvalidTokenError;
use aoc_runner_api::SolverResult;
use itertools::Itertools;

fn parse_floor(char: char) -> Result<i32, InvalidTokenError<char>> {
    match char {
        '(' => Ok(1),
        ')' => Ok(-1),
        c => Err(InvalidTokenError(c))
    }
}

fn find_basement<'a>(directions: impl Iterator<Item=&'a i32>) -> Option<usize> {
    let mut floor: i32 = 0;
    for (i, direction) in directions.enumerate() {
        floor += direction;
        if floor == -1 {
            return Some(i);
        }
    }

    None
}

fn parse_directions(input: &str) -> Result<Vec<i32>, InvalidTokenError<char>> {
    input.chars()
        .map(parse_floor)
        .try_collect()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let directions = parse_directions(input)?;
    let destination: i32 = directions.iter().sum();
    Ok(Box::new(destination))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let directions = parse_directions(input)?;
    let index = find_basement(directions.iter())
        .ok_or("Santa never enter the basement")?;

    Ok(Box::new(index + 1))
}