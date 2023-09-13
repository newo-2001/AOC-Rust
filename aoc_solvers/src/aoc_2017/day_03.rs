use std::{collections::HashMap, iter::once};

use aoc_lib::{geometry::{Point2D, CardinalDirection, RotationDirection}, NoSolutionError};
use aoc_runner_api::SolverResult;
use num::{abs, Integer};

struct SpiralIterator {
    next: Point2D,
    facing: CardinalDirection,
    start_facing: CardinalDirection,
    direction: RotationDirection,
    index: u32,
}

impl SpiralIterator {
    fn new(start: Point2D, direction: RotationDirection, facing: CardinalDirection) -> Self {
        SpiralIterator {
            next: start,
            start_facing: facing,
            index: 1,
            direction,
            facing
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = Point2D;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;

        let root = f64::sqrt(f64::from(self.index - 1));
        
        #[allow(clippy::cast_possible_truncation)]
        let is_odd_square = root % 1.0 == 0.0 && (root as i32).is_odd();
        
        if (self.facing == self.start_facing && is_odd_square) ||
           (self.facing != self.start_facing && abs(current.x()) == abs(current.y()))
        {
            self.facing = self.facing.rotate(self.direction);
        } 
        
        self.index += 1;
        self.next = current + self.facing.direction_vector();
        Some(current)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let index: usize = str::parse(input)?;
    let mut spiral = SpiralIterator::new(Point2D::zero(), RotationDirection::Left, CardinalDirection::East);
    
    let location = spiral.nth(index - 1).unwrap();
    let distance = location.manhattan_distance(&Point2D::zero());
    Ok(Box::new(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let target: u32 = str::parse(input)?;
    let spiral = SpiralIterator::new(Point2D::zero(), RotationDirection::Left, CardinalDirection::East);
    let mut grid: HashMap::<Point2D, u32> = once((Point2D::zero(), 1)).collect();
    
    for point in spiral {
        let value: u32 = point.neighbours()
            .filter_map(|neighbour| grid.get(&neighbour))
            .sum();

        if value > target { return Ok(Box::new(value)) }
        _ = grid.try_insert(point, value);
    }

    Err(Box::new(NoSolutionError))
}