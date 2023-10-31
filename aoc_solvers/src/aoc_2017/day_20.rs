use std::cmp::Ordering;

use aoc_lib::{geometry::Point3D, parsing::{Parsable, angle_brackets, Map3, TextParserResult, lines, TextParser}, NoInputError};
use aoc_runner_api::SolverResult;
use nom::{sequence::{tuple, terminated, delimited, preceded}, character::complete::{i32, char}, Parser, bytes::complete::tag};
use itertools::Itertools;

fn parse_vec3(input: &str) -> TextParserResult<Point3D<i32>> {
    angle_brackets(tuple((
        terminated(i32, char(',')),
        terminated(i32, char(',')), i32
    ))).map3(Point3D).parse(input)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Particle {
    position: Point3D<i32>,
    velocity: Point3D<i32>,
    acceleration: Point3D<i32>
}

impl Parsable<'_> for Particle {
    fn parse(input: &str) -> TextParserResult<Self> {
        tuple((
            delimited(tag("p="), parse_vec3, tag(", ")),
            delimited(tag("v="), parse_vec3, tag(", ")),
            preceded(tag("a="), parse_vec3)
        )).map(|(position, velocity, acceleration)| Particle {
            position, velocity, acceleration
        }).parse(input)
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Particle {
    fn cmp(&self, other: &Self) -> Ordering {
        // The only statistic that matters is the highest acceleration
        // regardless of direction
        let ordering = self.acceleration.manhattan_distance(Point3D::zero())
            .cmp(&other.acceleration.manhattan_distance(Point3D::zero()));
        
        // In case of two particles accelerating at the same speed,
        // the one with an initial velocity vector most similar
        // to its acceleration vector will have a head start
        match ordering {
            Ordering::Equal => self.acceleration.dot(self.velocity)
                .cmp(&other.acceleration.dot(other.velocity)),
            _ => ordering
        }

        // TODO: Technically a tie in velocities should also be considered
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let particles: Vec<Particle> = lines(Particle::parse).run(input)?;

    // TODO: Technically it is possible for two particles to tie
    // In that case the input should be rejected
    let fastest = *particles.iter()
        .sorted()
        .next()
        .ok_or(NoInputError)?;

    let index = particles.into_iter()
        .position(|particle| particle == fastest)
        .unwrap();

    Ok(Box::new(index))
}