use aoc_lib::{geometry::{Dimensions, Point2D, GridView, Grid, GridViewMut}, functional::repeat_apply};
use aoc_runner_api::SolverResult;

fn neighbours_on(grid: &GridView<bool>, cell: Point2D<usize>) -> usize {
    cell.neighbours()
        .filter_map(|location| grid.get(location)?.then_some(()))
        .count()
}

fn next_state(grid: GridView<bool>) -> Grid<bool> {
    grid.enumerate_map(|(location, &state)| {
        match (state, neighbours_on(&grid, location)) {
            (true, 2) | (true, 3) => true,
            (false, 3) => true,
            _ => false
        }
    })
}

fn fix_corners(grid: &mut GridViewMut<bool>) {
    let corners = grid.area().corners();

    for location in corners {
        *grid.get_mut(location).unwrap() = true;
    }
}

fn count_lights(grid: &GridView<bool>) -> usize {
    grid.into_iter()
        .filter(|&x| *x)
        .count()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = Grid::<bool>::parse(Dimensions(100, 100), input)?;
    let grid = repeat_apply(100, grid, |grid| next_state(grid.view()));

    let lights = count_lights(&grid.view());
    Ok(Box::new(lights))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = Grid::<bool>::parse(Dimensions(100, 100), input)?;

    let mut grid = repeat_apply(100, grid, |mut grid| {
        fix_corners(&mut grid.view_mut());
        next_state(grid.view())
    });

    fix_corners(&mut grid.view_mut());

    let lights = count_lights(&grid.view());

    Ok(Box::new(lights))
}