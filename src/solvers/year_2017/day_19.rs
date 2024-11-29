use ahash::{HashSet, HashSetExt};
use aoc_lib::{parsing::InvalidTokenError, geometry::{grid::{Grid, GridLike}, Point2D, CardinalDirection, Directional}, iteration::{ExtraIter, SingleError}};
use crate::SolverResult;
use anyhow::{anyhow, bail, Context, Result};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Straight(Option<char>),
    Corner
}

impl TryFrom<char> for Tile {
    type Error = InvalidTokenError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ' ' => Ok(Self::Empty),
            '+' => Ok(Self::Corner),
            '|' | '-' => Ok(Self::Straight(None)),
            char if char.is_ascii_uppercase() => Ok(Self::Straight(Some(char))),
            char => Err(InvalidTokenError(char))
        }
    }
}

impl Tile {
    fn is_path(self) -> bool { self != Self::Empty }
}

struct Packet {
    position: Point2D<usize>,
    facing: CardinalDirection
}

struct Map(Grid<Tile>);

impl Map {
    fn start(&self) -> Result<Point2D<usize>> {
        let first_row = self.0
            .get_row(0)
            .context("Grid is empty")?;

        let column = first_row.iter()
            .enumerate()
            .filter_map(|(column, item)| {
                item.is_path().then_some(column)
            }).single();

        match column {
            Ok(column) => Ok(Point2D(column, 0)),
            Err(SingleError::More) => Err(anyhow!("There are multiple starting locations")),
            Err(SingleError::None) => Err(anyhow!("There are no starting locations"))
        }
    }

    fn path(&self) -> Result<impl Iterator<Item=Tile>> {
        let mut packet = Packet {
            position: self.start()?,
            facing: CardinalDirection::South
        };

        let mut path = Vec::new();
        let mut seen = HashSet::new();

        loop {
            let tile = self.0
                .get(packet.position)
                .context("Packet traveled off the grid")?;

            seen.insert(packet.position);

            match tile {
                Tile::Straight(_) => {},
                Tile::Empty => break,
                Tile::Corner => {
                    let neighbours = packet.position.neighbours::<isize, _>(CardinalDirection::all())
                        .filter(|&neighbour| {
                            !seen.contains(&neighbour) &&
                            self.0.get(neighbour).is_some_and(|tile| tile.is_path())
                        }).single();

                    match neighbours {
                        Ok(neighbour) => packet.facing = packet.position.direction_to(neighbour).unwrap(),
                        Err(SingleError::More) => bail!("Corner had multiple valid paths"),
                        Err(SingleError::None) => break,
                    }
                }
            }
            
            let step: Point2D<isize> = packet.facing.direction_vector();
            packet.position = packet.position
                .checked_add(step)
                .context("Packet traveled off the grid")?;
            
            path.push(*tile);
        }

        Ok(path.into_iter())
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let map = Map(Grid::parse(input)?);
    let letters: String = map.path()?
        .filter_map(|tile| match tile {
            Tile::Straight(letter) => letter,
            _ => None
        }).collect();

    Ok(Box::new(letters))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let map = Map(Grid::parse(input)?);
    let distance = map.path()?.count();

    Ok(Box::new(distance))
}