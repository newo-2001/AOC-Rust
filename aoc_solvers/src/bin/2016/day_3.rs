use std::error::Error;

use aoc_lib::{io::read_puzzle_input, parsing::run};
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

fn main() -> Result<(), Box<dyn Error>> {
    let triangles = read_puzzle_input(2016, 3)?
        .lines()
        .map(Triangle::parse)
        .collect::<Result<Vec<Triangle>, String>>()?;

    let number_valid = triangles.iter()
        .filter(|&triangle| triangle.is_valid())
        .count();

    println!("There are {} valid triangles", number_valid);

    let number_valid = triangles.into_iter()
        .chunks(3)
        .into_iter()
        .flat_map(transpose_triangles)
        .filter(Triangle::is_valid)
        .count();

    println!("There are actually {} valid triangles", number_valid);

    Ok(())
}