use aoc_lib::parsing::{run, parse_lines};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{sequence::{tuple, preceded}, character::complete::{self, multispace1}, Parser};
use transpose::transpose;

struct Triangle(u32, u32, u32);

impl Triangle {
    fn is_valid(&self) -> bool {
        let &Self(x, y, z) = self;

        x + y > z &&
        y + z > x &&
        x + z > y
    }

    fn parse(input: &str) -> Result<Triangle, String> {
        let mut triangle = tuple((
            preceded(multispace1, complete::u32),
            preceded(multispace1, complete::u32),
            preceded(multispace1, complete::u32)
        )).map(|(x, y, z)| Triangle(x, y, z));

        run(&mut triangle, input)
    }
}

fn transpose_triangles(triangles: impl IntoIterator<Item=Triangle>) -> impl Iterator<Item=Triangle> {
    let origin: Vec<u32> = triangles.into_iter()
        .flat_map(|Triangle(x, y, z)| [x, y, z])
        .collect();

    let mut dest = [0u32; 9];
    transpose(&origin, &mut dest, 3, 3);

    dest.into_iter()
        .tuples()
        .map(|(x, y, z)| Triangle(x, y, z))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let number_valid = parse_lines(Triangle::parse, input)?
        .iter()
        .filter(|&triangle| triangle.is_valid())
        .count();

    Ok(Box::new(number_valid))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let number_valid = parse_lines(Triangle::parse, input)?
        .into_iter()
        .chunks(3)
        .into_iter()
        .flat_map(transpose_triangles)
        .filter(Triangle::is_valid)
        .count();

    Ok(Box::new(number_valid))
}