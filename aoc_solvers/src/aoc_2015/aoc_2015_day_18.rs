use std::error::Error;

use aoc_lib::spatial::{Grid, Coordinate, Area};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use tupletools::snd;

fn parse_cell(char: char) -> Result<bool, Box<dyn Error>> {
    match char {
        '.' => Ok(false),
        '#' => Ok(true),
        c => Err(Box::from(format!("Grid contained unexpected character: '{}'", c)))
    }
}

fn neighbours(grid: &Grid<bool>, cell: &Coordinate) -> usize {
    cell.neighbours()
        .into_iter()
        .filter_map(|neighbour| grid.value_at(&neighbour))
        .filter(|&x| *x)
        .count()
}

fn next_state<'a>(grid: &'a Grid<bool>, area: Area) -> impl Iterator<Item=(Coordinate, bool)> + 'a {
    grid.iter(area).map(|(coord, state)| {
        let state = state.expect("Grid index out of range");
        let neighbours = neighbours(grid, &coord);

        let new_state = match (state, neighbours) {
            (true, 2) | (true, 3) => true,
            (false, 3) => true,
            _ => false
        };

        (coord, new_state)
    })
}

fn animate(grid: &mut Grid<bool>, area: Area) {
    let new_state = next_state(grid, area).collect_vec();
    for (coord, state) in new_state {
        grid.squares.insert(coord, state);
    }
}

fn fix_corners(grid: &mut Grid<bool>, area: &Area) {
    let corners = area.corners();
    
    for coord in corners {
        grid.squares.insert(coord, true);
    }
}

fn animate_broken_corners(grid: &mut Grid<bool>, area: Area) {
    fix_corners(grid, &area);
    animate(grid, area);
}

fn count_lights(grid: &Grid<bool>, area: Area) -> usize {
    grid.iter(area)
        .filter_map(snd)
        .filter(|&x| *x == true)
        .count()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut grid = Grid::from_lines(input.lines(), parse_cell)?;
    let area = Area::square(100);

    for _ in 0..100 {
        animate(&mut grid, area.clone());
    }

    let lights = count_lights(&grid, area.clone());

    Ok(Box::new(lights))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut grid = Grid::from_lines(input.lines(), parse_cell)?;
    let area = Area::square(100);

    for _ in 0..100 {
        animate_broken_corners(&mut grid, area.clone());
    }

    fix_corners(&mut grid, &area);
    let lights = count_lights(&grid, area);

    Ok(Box::new(lights))
}