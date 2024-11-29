use aoc_lib::{parsing::InvalidTokenError, iteration::ExtraIter};
use crate::SolverResult;
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
    const fn from_neighbours(neighbours: (&Self, &Self, &Self)) -> Self {
        match neighbours {
            | (Self::Trap, _, Self::Safe)
            | (Self::Safe, _, Self::Trap) => Self::Trap,
            _ => Self::Safe
        }
    }
}


fn parse_row(input: &str) -> Result<Vec<Tile>, InvalidTokenError<char>> {
    input.chars()
        .map(TryInto::try_into)
        .try_collect()
}

struct RowIterator {
    next: Vec<Tile>
}

impl RowIterator {
    const fn new(row: Vec<Tile>) -> Self {
        Self { next: row }
    }
}

impl Iterator for RowIterator {
    type Item = Vec<Tile>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next.clone();
        self.next = self.next.iter()
            .enumerate()
            .map(|(i, center)| {
                let left = self.next.get(i.wrapping_sub(1)).unwrap_or(&Tile::Safe);
                let right = self.next.get(i+1).unwrap_or(&Tile::Safe);
                Tile::from_neighbours((left, center, right))
            }).collect_vec();
        
        Some(current)
    }
}

fn safe_tiles_in_rows(first_row: Vec<Tile>, rows: usize) -> usize {
    RowIterator::new(first_row)
        .take(rows)
        .flatten()
        .count_where(|&tile| tile == Tile::Safe)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let first_row = parse_row(input)?;
    Ok(Box::new(safe_tiles_in_rows(first_row, 40)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let first_row = parse_row(input)?;
    Ok(Box::new(safe_tiles_in_rows(first_row, 400_000)))
}