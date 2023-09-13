use aoc_lib::{geometry::{Dimensions, Point2D, grid::{Grid, GridLike, BitGrid, GridLikeMut}}, functional::{repeat_apply, consume}, math::Bit, iteration::ExtraIter};
use aoc_runner_api::SolverResult;

fn neighbours_on(grid: &Grid<Bit>, cell: Point2D<usize>) -> usize {
    cell.neighbours()
        .count_where(|location| grid.get(location).is_some_and(|light| light.is_on()))
}

fn next_state(grid: &Grid<Bit>) -> Grid<Bit> {
    grid.enumerate_map(|(location, &light)| {
        match (light, neighbours_on(grid, location)) {
            | (Bit::On, 2 | 3)
            | (Bit::Off, 3) => Bit::On,
            _ => Bit::Off
        }
    })
}

fn fix_corners(grid: &mut Grid<Bit>) {
    let corners = grid.area().corners();

    for location in corners {
        *grid.get_mut(location).unwrap() = Bit::On;
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = Grid::parse(Dimensions(100, 100), input)?;
    let grid = repeat_apply(100, grid, consume(next_state));

    Ok(Box::new(grid.count_lit()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = Grid::<Bit>::parse(Dimensions(100, 100), input)?;

    let mut grid = repeat_apply(100, grid, |mut grid| {
        fix_corners(&mut grid);
        next_state(&grid)
    });

    fix_corners(&mut grid);

    Ok(Box::new(grid.count_lit()))
}