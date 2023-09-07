use std::iter::once;

use aoc_lib::parsing::InvalidTokenError;
use aoc_runner_api::SolverResult;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Safe,
    Trap
}

impl TryFrom<char> for Tile {
    type Error = InvalidTokenError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '^' => Self::Trap,
            '.' => Self::Safe,
            char => Err(InvalidTokenError(char))?
        })
    }
}

impl Tile {
    fn from_neighbours(neighbours: (&Tile, &Tile, &Tile)) -> Tile {
        match neighbours {
            | (Tile::Trap, Tile::Trap, Tile::Safe)
            | (Tile::Safe, Tile::Trap, Tile::Trap)
            | (Tile::Trap, Tile::Safe, Tile::Safe)
            | (Tile::Safe, Tile::Safe, Tile::Trap) => Tile::Trap,
            _ => Tile::Safe
        }
    }
}


fn parse_row(input: &str) -> Result<Vec<Tile>, InvalidTokenError<char>> {
    input.chars()
        .map(|c| c.try_into())
        .try_collect()
}

struct RowIterator {
    previous: Vec<Tile>
}

impl RowIterator {
    fn new(row: Vec<Tile>) -> impl Iterator<Item=Vec<Tile>> {
        once(row.clone()).chain(RowIterator { previous: row })
    }
}

impl Iterator for RowIterator {
    type Item = Vec<Tile>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.previous.iter()
            .enumerate()
            .map(|(i, center)| {
                let left = self.previous.get(i.wrapping_sub(1)).unwrap_or(&Tile::Safe);
                let right = self.previous.get(i+1).unwrap_or(&Tile::Safe);
                Tile::from_neighbours((left, center, right))
            }).collect_vec();
        
        self.previous = row.clone();
        Some(row)
    }
}

fn safe_tiles_in_rows(first_row: Vec<Tile>, rows: usize) -> usize {
    RowIterator::new(first_row)
        .take(rows)
        .flatten()
        .filter(|&tile| tile == Tile::Safe)
        .count()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let first_row = parse_row(input)?;
    Ok(Box::new(safe_tiles_in_rows(first_row, 40)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let first_row = parse_row(input)?;
    Ok(Box::new(safe_tiles_in_rows(first_row, 400000)))
}