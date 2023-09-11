use aoc_lib::{geometry::{Point2D, grid::{Grid, BitGrid, GridLikeMut, GridLike, GridError}, Dimensions, Area}, parsing::{parse_lines, Runnable, ParseError}, math::Bit};
use aoc_runner_api::SolverResult;
use nom::{bytes::complete::tag, branch::alt, combinator::value, sequence::{tuple, preceded}, Parser};

#[derive(Clone, Copy)]
enum Action {
    On,
    Off,
    Toggle
}

trait Togglable: Copy {
    fn toggle(self, action: Action) -> Self;
}

impl Togglable for Bit {
    fn toggle(self, action: Action) -> Self {
        match action {
            Action::On => Bit::On,
            Action::Off => Bit::Off,
            Action::Toggle => self.invert()
        }
    }
}

impl Togglable for u32 {
    fn toggle(self, action: Action) -> Self {
        match action {
            Action::On => self + 1,
            Action::Off => self.saturating_sub(1),
            Action::Toggle => self + 2
        }
    }
}

#[derive(Clone, Copy)]
struct Step {
    area: Area<usize>,
    action: Action
}

fn parse_step(str: &str) -> Result<Step, ParseError> {
    let action = alt((
        value(Action::Off, tag("turn off ")),
        value(Action::On, tag("turn on ")),
        value(Action::Toggle, tag("toggle "))
    ));

    tuple((
        action,
        Point2D::parse,
        preceded(tag(" through "), Point2D::parse)
    )).map(|(action, top_left, bottom_right)| {
        let area = Area::from_corners(top_left, bottom_right);
        Step { area, action }
    }).run(str)
}

impl Step {
    fn apply(self, grid: &mut Grid<impl Togglable + Clone>) -> Result<(), GridError> {
        let mut sub_grid = grid.sub_grid_mut(self.area)?;
        let new_state = sub_grid.into_grid()
            .map(|state| state.toggle(self.action));

        sub_grid.replace(new_state)?;

        Ok(())
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut grid: Grid<Bit> = Grid::empty(Dimensions(1000, 1000));

    parse_lines(parse_step, input)?
        .iter().try_for_each(|step| step.apply(&mut grid))?;

    Ok(Box::new(grid.count_lit()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut grid: Grid<u32> = Grid::empty(Dimensions(1000, 1000));

    parse_lines(parse_step, input)?
        .iter().try_for_each(|step| step.apply(&mut grid))?;

    let total_brightness: u32 = grid.into_iter().sum();

    Ok(Box::new(total_brightness))
}