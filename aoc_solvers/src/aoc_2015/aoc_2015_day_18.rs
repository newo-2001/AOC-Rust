use aoc_lib::{geometry::{Dimensions, Point2D, Grid, GridViewMut, Light, GridLike, LightGrid}, functional::repeat_apply};
use aoc_runner_api::SolverResult;

fn neighbours_on(grid: &Grid<Light>, cell: Point2D<usize>) -> usize {
    cell.neighbours()
        .filter_map(|location| grid.get(location)?.is_on().then_some(()))
        .count()
}

fn next_state(grid: Grid<Light>) -> Grid<Light> {
    grid.enumerate_map(|(location, &light)| {
        match (light, neighbours_on(&grid, location)) {
            (Light::On, 2) | (Light::On, 3) => Light::On,
            (Light::Off, 3) => Light::On,
            _ => Light::Off
        }
    })
}

fn fix_corners(grid: &mut GridViewMut<Light>) {
    let corners = grid.area().corners();

    for location in corners {
        *grid.get_mut(location).unwrap() = Light::On;
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = Grid::parse(Dimensions(100, 100), input)?;
    let grid = repeat_apply(100, grid, |grid| next_state(grid));

    Ok(Box::new(grid.count_lit()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = Grid::<Light>::parse(Dimensions(100, 100), input)?;

    let mut grid = repeat_apply(100, grid, |mut grid| {
        fix_corners(&mut grid.view_mut());
        next_state(grid)
    });

    fix_corners(&mut grid.view_mut());

    Ok(Box::new(grid.count_lit()))
}