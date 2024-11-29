use std::fmt::Display;

use aoc_lib::{geometry::{Point3D, Point2D}, parsing::{ParseError, Parsable, TextParser, parse_lines}, ignore};
use crate::SolverResult;
use itertools::Itertools;
use nom::{sequence::separated_pair, Parser, character::complete::{char, space1}};

#[derive(Clone, Copy)]
struct Hailstone {
    position: Point3D<f64>,
    velocity: Point3D<f64>
}

impl Hailstone {
    fn parse(input: &str) -> Result<Self, ParseError> {
        separated_pair(
            Point3D::<f64>::parse,
            ignore!(space1, char('@'), space1),
            Point3D::<f64>::parse
        ).map(|(position, velocity)| Self {
            position, velocity
        }).run(input)
    }

    fn collides_2d(self, other: Self) -> Option<Point2D<f64>> {
        let Self { position: Point3D(x1, y1, _z1), velocity: Point3D(vx1, vy1, _vz1) } = self;
        let Self { position: Point3D(x2, y2, _z2), velocity: Point3D(vx2, vy2, _vz2) } = other;

        let x = ((-vy2 / vx2).mul_add(x2, y2) - (-vy1 / vx1).mul_add(x1, y1)) / (vy1 / vx1 - vy2 / vx2);
        let y = (vy1 / vx1).mul_add(x, (-vy1 / vx1).mul_add(x1, y1));

        ((x - x1) / vx1 >= 0.0 && (x - x2) / vx2 >= 0.0).then_some(Point2D(x, y))
    }
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.position, self.velocity)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    const MIN: f64 = 200_000_000_000_000.0;
    const MAX: f64 = 400_000_000_000_000.0;

    let collisions = parse_lines(Hailstone::parse, input)?
        .into_iter()
        .tuple_combinations()
        .filter_map(|(a, b)| a.collides_2d(b))
        .filter(|&Point2D(x, y)| x >= MIN && y >= MIN && x <= MAX && y <= MAX)
        .count();

    Ok(Box::new(collisions))
}