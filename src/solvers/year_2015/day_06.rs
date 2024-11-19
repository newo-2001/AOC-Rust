use aoc_lib::{
    geometry::{Point2D, grid::{Grid, BitGrid, GridLikeMut, GridError}, Dimensions, Area},
    parsing::{TextParser, Parsable, TextParserResult, lines},
    math::Bit, string_enum
};
use crate::SolverResult;
use nom::{bytes::complete::tag, sequence::{tuple, preceded}, Parser};
use anyhow::Result;

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

impl Parsable<'_> for Step {
    fn parse(input: &str) -> TextParserResult<Step> {
        let action = string_enum! {
            "turn off " => Action::Off,
            "turn on " => Action::On,
            "toggle " => Action::Toggle
        };

        tuple((
            action,
            Point2D::parse,
            preceded(tag(" through "), Point2D::parse)
        )).map(|(action, top_left, bottom_right)| {
            let area = Area::from_corners(top_left, bottom_right);
            Step { area, action }
        }).parse(input)
    }
}

impl Step {
    fn apply(self, grid: &mut Grid<impl Togglable + Clone>) -> Result<(), GridError> {
        let mut sub_grid = grid.sub_grid_mut(self.area)?;
        let new_state = sub_grid.owned()
            .map(|state| state.toggle(self.action));

        sub_grid.replace(new_state)?;

        Ok(())
    }
}

fn parse_grid<T>(input: &str) -> Result<Grid<T>>
    where T: Default + Clone + Togglable
{
    let mut grid: Grid<T> = Grid::empty(Dimensions(1000, 1000));

    lines(Step::parse).run(input)?
        .iter()
        .try_for_each(|step| step.apply(&mut grid))?;
    
    Ok(grid)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = parse_grid(input)?;
    Ok(Box::new(grid.pop_count()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid: Grid<u32> = parse_grid(input)?;
    let total_brightness: u32 = grid.into_iter().sum();

    Ok(Box::new(total_brightness))
}