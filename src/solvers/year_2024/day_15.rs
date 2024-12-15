use std::{fmt::Display, iter::once};

use ahash::HashMap;
use anyhow::{bail, Context, Result};
use itertools::{Either, Itertools};
use recursive::recursive;
use yuki::{iterators::Enumerate2D, spatial::{direction::{self, Directions}, Point}};

use crate::SolverResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    LeftBox,
    RightBox
}

struct Grid {
    tiles: HashMap<Point<usize>, Tile>,
    robot: Point<usize>
}

fn parse_grid(input: &str, wide_boxes: bool) -> Result<Grid> {
    let (robot, _) = input
        .lines()
        .map(str::chars)
        .enumerate2d()
        .update(|(pos, _)| if wide_boxes { pos.x *= 2 })
        .find(|&(_, tile)| tile == '@')
        .context("Input contains no initial robot position")?;

    let tiles: HashMap<Point<usize>, Tile> = input
        .lines()
        .map(str::chars)
        .enumerate2d()
        .update(|(pos, _)| if wide_boxes { pos.x *= 2 })
        .map(|(pos, tile)| Ok(match tile {
            '#' if wide_boxes => Some(Either::Right([(pos, Tile::Wall), (pos + Point::new(1, 0), Tile::Wall)].into_iter())),
            'O' if wide_boxes => Some(Either::Right([(pos, Tile::LeftBox), (pos + Point::new(1, 0), Tile::RightBox)].into_iter())),
            '#' => Some(Either::Left(once((pos, Tile::Wall)))),
            'O' => Some(Either::Left(once((pos, Tile::Box)))),
            '.' | '@' => None,
            _ => bail!("Invalid character in grid: {tile}")
        }))
        .flatten_ok()
        .flatten_ok()
        .collect::<Result<_, _>>()?;

    Ok(Grid { tiles, robot })
} 

fn parse_input(input: &str, wide_boxes: bool) -> Result<(Grid, Vec<direction::Cardinal>)> {
    let (grid, moves) = input
        .split_once("\n\n")
        .context("Input does not contain a blank line")?;

    let grid = parse_grid(grid, wide_boxes)?;
    let moves: Vec<direction::Cardinal> = moves
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

    Ok((grid, moves))
}

impl Grid {
    #[recursive]
    fn push(&self, pos: Point<usize>, direction: direction::Cardinal) -> Option<Vec<Point<usize>>> {
        use direction::Cardinal as dir;

        let mut moved_boxes = Vec::<Point<usize>>::new();
        for offset in 1.. {
            let (dest, tile) = pos
                .add_signed(Point::<isize>::from(direction.vector()) * offset)
                .map(|pos| (pos, self.tiles.get(&pos)))
                .unwrap();

            let origin = dest.add_signed(direction.inverted().vector()).unwrap();
            moved_boxes.push(origin);

            match (tile, direction) {
                (None, _) => return Some(moved_boxes),
                (Some(Tile::Wall), _) => return None,
                (Some(Tile::Box), _) | (Some(Tile::LeftBox | Tile::RightBox), dir::East | dir::West) => {},
                (Some(Tile::LeftBox), dir::North | dir::South) => {
                    match self.push(dest + Point::new(1, 0), direction) {
                        None => return None,
                        Some(moved) => moved_boxes.extend(moved)
                    }
                },
                (Some(Tile::RightBox), dir::North | dir::South) => {
                    match self.push(dest.add_signed(Point::new(-1, 0)).unwrap(), direction) {
                        None => return None,
                        Some(moved) => moved_boxes.extend(moved)
                    }
                }
            }
        }

        unreachable!()
    }

    fn perform_move(&mut self, direction: direction::Cardinal) {
        let dest = self.robot.add_signed(direction.vector()).unwrap();
        match self.tiles.get(&dest) {
            None => self.robot = dest,
            Some(Tile::Wall) => {},
            Some(Tile::Box | Tile::LeftBox | Tile::RightBox) => {
                if let Some(mut moved_boxes) = self.push(self.robot, direction) {
                    self.robot = dest;

                    match direction {
                        direction::Cardinal::West => moved_boxes.sort_unstable_by_key(|point| point.x),
                        direction::Cardinal::North => moved_boxes.sort_unstable_by_key(|point| point.y),
                        direction::Cardinal::East => moved_boxes.sort_unstable_by(|a, b| a.x.cmp(&b.x).reverse()),
                        direction::Cardinal::South => moved_boxes.sort_unstable_by(|a, b| a.y.cmp(&b.y).reverse()),
                    }

                    for moved_box in moved_boxes {
                        let dest = moved_box.add_signed(direction.vector()).unwrap();
                        if let Some(tile) = self.tiles.remove(&moved_box) {
                            self.tiles.insert(dest, tile);
                        }
                    }
                }
            }
        }
    }

    fn gps(&self) -> usize {
        self.tiles
            .iter()
            .filter_map(|(Point { x, y }, tile)| match tile {
                Tile::Box | Tile::LeftBox => Some(100 * y + x),
                Tile::Wall | Tile::RightBox => None
            })
            .sum()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    const DIMENSIONS: (usize, usize) = (20, 10);
        let (width, height) = DIMENSIONS;

        for y in 0..height {
            for x in 0..width {
                let pos = Point::new(x, y);
                let char = match self.tiles.get(&pos) {
                    Some(Tile::Wall) => '#',
                    Some(Tile::LeftBox) => '[',
                    Some(Tile::RightBox) => ']',
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
    let (mut grid, moves) = parse_input(input, false)?;

    for direction in moves {
        grid.perform_move(direction);
    }

    Ok(Box::new(grid.gps()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (mut grid, moves) = parse_input(input, true)?;

    for direction in moves {
        grid.perform_move(direction);
    }
   
    Ok(Box::new(grid.gps()))
}