use std::{collections::HashSet, error::Error};

use aoc_lib::{geometry::{CardinalDirection, Point2D}, io::read_puzzle_input, parsing::{run, direction}};
use nom::multi::many0;

fn unique_houses<'a>(directions: impl IntoIterator<Item=&'a CardinalDirection>) -> HashSet<Point2D<i32>> {
    let mut position = Point2D::<i32>::zero();

    directions.into_iter()
        .map(|direction| {
            position += direction.direction_vector();
            position
        }).chain(std::iter::once(Point2D::zero()))
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_puzzle_input(2015, 3)?;
    let movements = run(&mut many0(direction), &content)?;

    let visited_houses = unique_houses(movements.iter()).len();
    println!("Santa visits {} unique houses", visited_houses);

    let santa_houses = unique_houses(movements.iter().step_by(2));
    let robo_santa_houses = unique_houses(movements.iter().skip(1).step_by(2));
    let all_houses = santa_houses.union(&robo_santa_houses);
    
    println!("Santa and Robo-Santa together visit {} unique houses", all_houses.count());

    Ok(())
}