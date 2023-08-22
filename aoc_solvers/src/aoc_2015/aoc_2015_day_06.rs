use std::cmp;

use aoc_lib::{geometry::Point2D, parsing::{point2d, run, parse_lines}};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{bytes::complete::tag, branch::alt, combinator::value, sequence::{tuple, preceded}, Parser};

type Grid<T> := Vec<[T; 1000]>;

#[derive(Clone, Copy)]
enum Action {
    On,
    Off,
    Toggle
}

trait Togglable {
    fn toggle(&mut self, action: &Action);
}

impl Togglable for bool {
    fn toggle(&mut self, action: &Action) {
        use Action::*;
        *self = match action {
            On => true,
            Off => false,
            Toggle => !(*self)
        };
    }
}

impl Togglable for i32 {
    fn toggle(&mut self, action: &Action) {
        use Action::*;
        *self = match action {
            On => *self + 1,
            Off => cmp::max(0, *self - 1),
            Toggle => *self + 2
        }
    }
}

struct Step {
    top_left: Point2D<usize>,
    bottom_right: Point2D<usize>,
    action: Action
}

fn set_square<T : Togglable>(grid: &mut Grid<T>, step: &Step) {
    for col in grid[step.top_left.1..step.bottom_right.1].iter_mut() {
        for tile in &mut col[step.top_left.0..step.bottom_right.0] {
            tile.toggle(&step.action);
        }
    }
}

fn parse_step(str: &str) -> Result<Step, String> {
    let action = alt((
        value(Action::Off, tag("turn off ")),
        value(Action::On, tag("turn on ")),
        value(Action::Toggle, tag("toggle "))
    ));

    let mut step = tuple((
        action,
        point2d,
        preceded(tag(" through "), point2d)
    )).map(|(action, top_left, bottom_right)|
        Step { action, top_left, bottom_right: bottom_right + Point2D::one() });

    run(&mut step, str)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut grid = vec![[false; 1000]; 1000];

    let steps = parse_lines(parse_step, input)?;
    for step in steps {
        set_square(&mut grid, &step);
    }

    let active_lights = *grid.iter()
        .flatten()
        .counts()
        .get(&true).unwrap_or(&0usize);

    Ok(Box::new(active_lights))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut grid = vec![[0; 1000]; 1000];

    let steps = parse_lines(parse_step, input)?;
    for step in steps {
        set_square(&mut grid, &step);
    }

    let total_brightness: i32 = grid.iter()
        .flatten().sum();

    Ok(Box::new(total_brightness))
}