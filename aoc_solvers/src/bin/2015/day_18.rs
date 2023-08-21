use std::error::Error;

use aoc_lib::{spatial::{Grid, Coordinate, Area}, io::read_puzzle_input};
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

fn main() -> Result<(), Box<dyn Error>> {
    let area = Area::square(100);
    let content = read_puzzle_input(2015, 18)?;
    let original_grid = Grid::from_lines(content.lines(), parse_cell)?;

    let mut grid = original_grid.clone();

    for _ in 0..100 {
        animate(&mut grid, area.clone());
    }

    let lights = count_lights(&grid, area.clone());
    println!("After 100 iterations {} lights remain on", lights);
    
    grid = original_grid;
    for _ in 0..100 {
        animate_broken_corners(&mut grid, area.clone());
    }

    fix_corners(&mut grid, &area);
    let lights = count_lights(&grid, area);
    println!("After 100 iterations with broken corner lights {} lights remain on", lights);

    Ok(())
}