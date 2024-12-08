use itertools::Itertools;
use num::Integer;
use yuki::spatial::{Area, Point};

use crate::SolverResult;

#[derive(Clone, Copy)]
struct Antenna {
    frequency: char,
    position: Point<i32>
}

struct Grid {
    antennas: Vec<Antenna>,
    area: Area<usize>
}

fn parse_grid(input: &str) -> Grid {
    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .filter(|(_, char)| *char != '.')
            .map(move |(x, char)| Antenna {
                #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
                position: Point { x: x as i32, y: y as i32 },
                frequency: char
            })
        )
        .collect();

    let height = input.lines().count();
    let width = input
        .lines()
        .next()
        .map(|line| line.chars().count())
        .unwrap_or_default();

    Grid { antennas, area: (width, height).into() }
}

impl Grid {
    fn count_antinodes<F, I>(&self, antinode_producer: F) -> usize where
        F: Fn(Point<i32>, Point<i32>) -> I,
        I: IntoIterator<Item=Point<i32>>
    {
        self.antennas
            .iter()
            .into_group_map_by(|antenna| antenna.frequency)
            .into_iter()
            .flat_map(|(_, locations)| locations
                .into_iter()
                .map(|antenna| antenna.position)
                .tuple_combinations()
                .flat_map(|(a, b)| antinode_producer(a, b))
            )
            .filter(|&point| self.area.contains(point))
            .unique()
            .count()
    }
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = parse_grid(input);
    let antinodes = grid.count_antinodes(|a, b| {
        let offset = a - b;
        [ a + offset, b - offset ]
    });

    Ok(Box::new(antinodes))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = parse_grid(input);
    let antinodes = grid.count_antinodes(|a, b| {
        let offset = a - b;

        // Find distance between points on the line that
        // match up with the grid lines
        let gcd = offset.x.gcd(&offset.y);
        let direction = offset / gcd;

        // Cast rays in both directions from one of the points
        let positive = (0..)
            .map(|distance| a - direction * distance)
            .take_while(|&pos| grid.area.contains(pos));

        let negative = (0..)
            .map(|distance| a + direction * distance)
            .take_while(|&pos| grid.area.contains(pos));

        positive
            .chain(negative)
            .collect_vec()
    });

    Ok(Box::new(antinodes))
}