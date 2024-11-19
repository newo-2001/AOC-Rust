use ahash::HashSet;
use aoc_lib::{geometry::Point3D, parsing::{ParseError, TextParser, Parsable}, iteration::ExtraIter};
use crate::SolverResult;
use itertools::Itertools;
use nom::{character::complete::char, sequence::separated_pair};

#[derive(Clone, PartialEq, Eq)]
struct Brick {
    positions: Vec<Point3D<u32>>,
    number: usize
}

impl Brick {
    fn is_supported(&self, grid: &HashSet<Point3D<u32>>) -> bool {
        self.positions.iter()
            .copied()
            .any(|Point3D(x, y, z)| {
                let below = Point3D(x, y, z - 1);
                (z == 1) || (grid.contains(&below) && !self.positions.contains(&below))
            })
    }

    fn drop(&mut self, grid: &mut HashSet<Point3D<u32>>) {
        while !self.is_supported(grid) {
            for point in &mut self.positions {
                grid.remove(point);

                let Point3D(_, _, z) = point;
                *z -= 1;

                grid.insert(*point);
            }
        }
    }
}

impl Brick {
    fn parse(input: &str) -> Result<Vec<Point3D<u32>>, ParseError> {
        let (Point3D(x1, y1, z1), Point3D(x2, y2, z2)) = separated_pair(
                Point3D::<u32>::parse,
                char('~'),
                Point3D::<u32>::parse
            ).run(input)?;
        
        Ok((x1..=x2).flat_map(|x| {
            (y1..=y2).flat_map(move |y| {
                (z1..=z2).map(move |z| Point3D(x, y, z))
            })
        }).collect())
    }

    fn disintegrate(&self, grid: &mut HashSet<Point3D<u32>>) {
        for pos in &self.positions {
            grid.remove(pos);
        }
    }
}

fn parse_bricks(input: &str) -> Result<(Vec<Brick>, HashSet<Point3D<u32>>), ParseError> {
    let bricks: Vec<Brick> = input.lines()
        .enumerate()
        .map(|(number, line)| Ok(Brick { number, positions: Brick::parse(line)? }))
        .try_collect()?;

    let grid = bricks.iter()   
        .flat_map(|brick| brick.positions.iter().copied())
        .collect();

    Ok((bricks, grid))
}

fn stabilize(bricks: &mut Vec<Brick>, grid: &mut HashSet<Point3D<u32>>) {
    while !bricks.iter().all(|brick| brick.is_supported(grid)) {
        for brick in &mut *bricks {
            brick.drop(grid);
        }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (mut bricks, mut grid) = parse_bricks(input)?;
    stabilize(&mut bricks, &mut grid);

    let disintegratable = bricks.iter().count_where(|brick| {
        brick.disintegrate(&mut grid);

        let stable = bricks.iter().all(|other| other.is_supported(&grid));

        for &pos in &brick.positions {
            grid.insert(pos);
        }

        stable
    });

    Ok(Box::new(disintegratable))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (mut bricks, mut grid) = parse_bricks(input)?;
    stabilize(&mut bricks, &mut grid);

    let falling_bricks: usize = bricks.iter().enumerate().map(|(index, brick)| {
        let mut dropped_bricks = bricks.clone();
        let mut updated_grid = grid.clone();
        dropped_bricks.remove(index);
        
        brick.disintegrate(&mut updated_grid);
        stabilize(&mut dropped_bricks, &mut updated_grid);

        bricks.iter().count_where(|&original| original != brick && !dropped_bricks.contains(original))
    }).sum();

    Ok(Box::new(falling_bricks))
}