use ahash::HashSet;
use aoc_lib::{geometry::{Point2D, grid::{Grid, GridLike}}, iteration::ExtraIter, math::Bit};
use aoc_runner_api::SolverResult;
use anyhow::Result;
use itertools::Itertools;

fn parse_stars(input: &str, expansion_rate: usize) -> Result<HashSet<Point2D<usize>>> {
    let universe: Grid<Bit> = Grid::parse(input)?;

    let expansion_rows = universe.enumerate_rows()
        .filter_map(|(index, mut row)| {
            row.none(|x| x.is_solid()).then_some(index)
        }).collect_vec();

    let expansion_columns = universe.enumerate_columns()
        .filter_map(|(index, mut col)| {
            col.none(|x| x.is_solid()).then_some(index)
        }).collect_vec();

    let universe = universe.enumerate()
        .filter(|(_, tile)| tile.is_solid())
        .map(move |(Point2D(x, y), _)| {
            let x = x + expansion_columns.iter().take_while(|&col| *col < x).count() * expansion_rate;
            let y = y + expansion_rows.iter().take_while(|&row| *row < y).count() * expansion_rate;
            Point2D(x, y)
        }).collect();

    Ok(universe)
}

fn total_distance<'a, I>(galaxies: I) -> usize
    where I: IntoIterator<Item=&'a Point2D<usize>>,
          I::IntoIter: Clone
{
    galaxies.into_iter()
        .tuple_combinations()
        .sum_by(|(a, b)| a.manhattan_distance(*b))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let galaxies = parse_stars(input, 1)?;
    Ok(Box::new(total_distance(&galaxies)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let galaxies = parse_stars(input, 999_999)?;
    Ok(Box::new(total_distance(&galaxies)))
}