use std::convert::identity;
use ahash::HashSet;
use nom::{character::complete::{char, i16, u16}, combinator::map, bytes::complete::tag, sequence::{preceded, separated_pair}, Parser};
use yuki::{parsing::{parse_lines, Parsable, ParsingResult}, spatial::{Area, Point}};

use crate::SolverResult;

struct Robot {
    position: Point<usize>,
    velocity: Point<isize>
}

impl<'a> Parsable<'a> for Robot {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        separated_pair(
            map(
                preceded(
                    tag("p="),
                    separated_pair(u16, char(','), u16)
                ),
                |(x, y)| Point { x: usize::from(x), y: usize::from(y) }
            ),
            char(' '),
            map(
                preceded(
                    tag("v="),
                    separated_pair(i16, char(','), i16)
                ),
                |(x, y)| Point { x: isize::from(x), y: isize::from(y) }
            )
        )
        .map(|(position, velocity)| Self { position, velocity })
        .parse(input)
    }
}

impl Robot {
    #[allow(clippy::cast_possible_wrap)]
    fn position_at_time(&self, time: usize, (width, height): (usize, usize)) -> Point<usize> {
        let Point { x, y} = self.position.cast::<isize>().unwrap() + (self.velocity * time as isize);
        Point::new(x.rem_euclid(width as isize) as usize, y.rem_euclid(height as isize) as usize)
    }
}

struct Room {
    robots: Vec<Robot>,
    dimensions: (usize, usize)
}

impl Room {
    fn safety_factor_at_time(&self, time: usize) -> usize {
        let (width, height) = self.dimensions;
        let positions: Vec<Point<usize>> = self.robots
            .iter()
            .map(|robot| robot.position_at_time(time, self.dimensions))
            .collect();

        [
            positions.iter().filter(|pos| pos.x < width / 2 && pos.y < height / 2).count(),
            positions.iter().filter(|pos| pos.x < width / 2 && pos.y > height / 2).count(),
            positions.iter().filter(|pos| pos.x > width / 2 && pos.y < height / 2).count(),
            positions.iter().filter(|pos| pos.x > width / 2 && pos.y > height / 2).count()
        ].into_iter()
            .product()
    }

    fn simulate(&mut self, time: usize) {
        for robot in &mut self.robots {
            robot.position = robot.position_at_time(time, self.dimensions);
        }
    }

    /// Detects if a contiguous line of at least 10 robots is present
    fn is_christmas_tree(&self) -> bool {
        let positions: HashSet<Point<usize>> = self.robots
            .iter()
            .map(|robot| robot.position)
            .collect();

        let (width, height) = self.dimensions;
        Area::from_dimensions(width, height)
            .into_iter()
            .map_windows::<_, _, 10>(|window| window
                .iter()
                .all(|pos| positions.contains(pos))
            )
            .any(identity)
    }
}

const ROOM_DIMENSIONS: (usize, usize) = (101, 103);

pub fn solve_part_1(input: &str) -> SolverResult {
    let room = Room {
        robots: parse_lines(input)?,
        dimensions: ROOM_DIMENSIONS
    };

    Ok(Box::new(room.safety_factor_at_time(100)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut room = Room {
        robots: parse_lines(input)?,
        dimensions: ROOM_DIMENSIONS
    };

    for i in 0.. {
        if room.is_christmas_tree() { return Ok(Box::new(i)) }
        room.simulate(1);
    }

    unreachable!()
}