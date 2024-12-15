use std::{collections::VecDeque, fmt::Display};

use ahash::HashMap;
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use yuki::{iterators::Enumerate2D, spatial::{direction::{self, Directions}, Point}};

use crate::SolverResult;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box
}

struct Grid {
    tiles: HashMap<Point<usize>, Tile>,
    robot: Point<usize>
}

fn parse_grid(input: &str) -> Result<(Grid, VecDeque<direction::Cardinal>)> {
    let (grid, moves) = input
        .split_once("\n\n")
        .context("Input does not contain a blank line")?;

    let (robot, _) = grid
        .lines()
        .map(str::chars)
        .enumerate2d()
        .find(|&(_, tile)| tile == '@')
        .context("Input contains no initial robot position")?;

    let tiles: HashMap<Point<usize>, Tile> = grid
        .lines()
        .map(str::chars)
        .enumerate2d()
        .map(|(pos, tile)| Ok(match tile {
            '#' => Some((pos, Tile::Wall)),
            'O' => Some((pos, Tile::Box)),
            '.' | '@' => None,
            _ => bail!("Invalid character in grid: {tile}")
        }))
        .flatten_ok()
        .collect::<Result<_, _>>()?;

    let moves: VecDeque<direction::Cardinal> = moves
        .lines()
        .flat_map(str::chars)
        .map(|dir| Ok(match dir {
            '^' => direction::Cardinal::North,
            '>' => direction::Cardinal::East,
            'v' => direction::Cardinal::South,
            '<' => direction::Cardinal::West,
            _ => bail!("Invalid character in moves: {dir}")
        }))
        .collect::<Result<_, _>>()?;

    Ok((Grid { tiles, robot }, moves))
}

impl Grid {
    fn perform_move(&mut self, direction: direction::Cardinal) {
        let dest = self.robot.add_signed(direction.vector()).unwrap();
        match self.tiles.get(&dest) {
            None => self.robot = dest,
            Some(Tile::Wall) => {},
            Some(Tile::Box) => {
                #[allow(clippy::maybe_infinite_iter)]
                let (box_dest_pos, box_dest_tile) = (1..)
                    .map(|offset| dest
                        .add_signed(Point::<isize>::from(direction.vector()) * offset)
                        .map(|pos| (pos, self.tiles.get(&pos)))
                        .unwrap()
                    )
                    .find(|&(_, tile)| !matches!(tile, Some(Tile::Box)))
                    .unwrap();

                if box_dest_tile.is_none() {
                    self.robot = dest;
                    self.tiles.remove(&dest);
                    self.tiles.insert(box_dest_pos, Tile::Box);
                }
            }
        }
    }

    fn gps(&self) -> usize {
        self.tiles
            .iter()
            .filter_map(|(Point { x, y }, tile)| match tile {
                Tile::Box => Some(100 * y + x),
                Tile::Wall => None
            })
            .sum()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const DIMENSIONS: (usize, usize) = (8, 8);
        let (width, height) = DIMENSIONS;

        for y in 0..height {
            for x in 0..width {
                let pos = Point::new(x, y);
                let char = match self.tiles.get(&pos) {
                    Some(Tile::Wall) => '#',
                    Some(Tile::Box) => 'O',
                    None if pos == self.robot => '@',
                    None => '.'
                };

                write!(f, "{char}")?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (mut grid, moves) = parse_grid(input)?;

    for direction in moves {
        grid.perform_move(direction);
    }

    Ok(Box::new(grid.gps()))
}