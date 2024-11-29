use std::cmp::Ordering;

use anyhow::bail;
use aoc_lib::{geometry::Point3D, parsing::{Parsable, angle_brackets, Map3, TextParserResult, lines, TextParser}, errors::{MultipleSolutions, NoInput}, iteration::{ExtraIter, SingleError}};
use crate::SolverResult;
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
        )).map(|(position, velocity, acceleration)| Self {
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
        let ordering = self.acceleration.magnitude()
            .cmp(&other.acceleration.magnitude());
        
        // In case of two particles accelerating at the same speed,
        // the one with an initial velocity vector most similar
        // to its acceleration vector will have a head start
        // In order to respect magnitude, we project the velocity vector
        // onto the acceleration vector using the dot product
        match ordering {
            Ordering::Equal => self.acceleration.normalized().dot(self.velocity)
                .cmp(&other.acceleration.normalized().dot(other.velocity)),
            _ => ordering
        }
    }
}

impl Particle {
    fn simulate(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let particles = lines(Particle::parse).run(input)?;

    let slowest = particles.into_iter()
        .enumerate()
        .min_set_by_key(|(_, particle)| *particle)
        .into_iter()
        .single();

    match slowest {
        Ok((index, _)) => Ok(Box::new(index)),
        Err(SingleError::More) => bail!(MultipleSolutions),
        Err(SingleError::None) => bail!(NoInput)
    }
}

pub fn solve_part_2(input: &str) -> SolverResult {
    // I hate this, but I can't figure out a reasonable way to find a stopping condition
    // Attempted to do it analytically, but that involved solving 3d quadratics
    const ITERATIONS: u32 = 1_000;

    let mut particles = lines(Particle::parse).run(input)?;

    for _ in 0..ITERATIONS {
        let positions = particles.iter()
            .counts_by(|particle| particle.position);
        
        particles.retain(|particle| positions[&particle.position] == 1);
        particles.iter_mut().for_each(Particle::simulate);
    }

    Ok(Box::new(particles.len()))
}