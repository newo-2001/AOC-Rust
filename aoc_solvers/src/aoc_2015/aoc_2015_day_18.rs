use aoc_lib::{geometry::{Dimensions, Point2D, GridView, Grid, GridViewMut, Light}, functional::repeat_apply};
use aoc_runner_api::SolverResult;

fn neighbours_on(grid: &GridView<Light>, cell: Point2D<usize>) -> usize {
    cell.neighbours()
        .filter_map(|location| grid.get(location)?.is_on().then_some(()))
        .count()
}

fn next_state(grid: &GridView<Light>) -> Grid<Light> {
    grid.enumerate_map(|(location, &light)| {
        match (light, neighbours_on(grid, location)) {
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

fn count_lights(grid: &GridView<Light>) -> usize {
    grid.into_iter()
        .filter(|&light| light.is_on())
        .count()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = Grid::parse(Dimensions(100, 100), input)?;
    let grid = repeat_apply(100, grid, |grid| next_state(&grid.view()));

    let lights = count_lights(&grid.view());
    Ok(Box::new(lights))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = Grid::<Light>::parse(Dimensions(100, 100), input)?;

    let mut grid = repeat_apply(100, grid, |mut grid| {
        fix_corners(&mut grid.view_mut());
        next_state(&grid.view())
    });

    fix_corners(&mut grid.view_mut());

    let lights = count_lights(&grid.view());

    Ok(Box::new(lights))
}