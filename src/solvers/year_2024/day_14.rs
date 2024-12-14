use nom::{character::complete::{char, i16, u16}, combinator::map, bytes::complete::tag, sequence::{preceded, separated_pair}, Parser};
use yuki::{parsing::{parse_lines, Parsable, ParsingResult}, spatial::Point};

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
    const fn position_at_time(&self, time: usize, (width, height): (usize, usize)) -> Point<usize> {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        Point {
            x: (self.position.x as isize + self.velocity.x * time as isize).rem_euclid(width as isize) as usize,
            y: (self.position.y as isize + self.velocity.y * time as isize).rem_euclid(height as isize) as usize
        }
    }
}

fn safety_factor(robots: impl IntoIterator<Item=Robot>, time: usize, (width, height): (usize, usize)) -> usize {
    let positions: Vec<Point<usize>> = robots
        .into_iter()
        .map(|robot| robot.position_at_time(time, (width, height)))
        .collect();

    dbg!(&positions);

    [
        positions.iter().filter(|pos| pos.x < width / 2 && pos.y < height / 2).count(),
        positions.iter().filter(|pos| pos.x < width / 2 && pos.y > height / 2).count(),
        positions.iter().filter(|pos| pos.x > width / 2 && pos.y < height / 2).count(),
        positions.iter().filter(|pos| pos.x > width / 2 && pos.y > height / 2).count()
    ].into_iter()
        .product()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let robots: Vec<Robot> = parse_lines(input)?;
    Ok(Box::new(safety_factor(robots, 100, (101, 103))))
}