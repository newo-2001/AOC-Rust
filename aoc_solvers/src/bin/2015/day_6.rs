use std::{cmp, error::Error};

use aoc_lib::{io::read_puzzle_input, geometry::Point2D, parsing::{point2d, run}};
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

fn main() -> Result<(), Box<dyn Error>> {
    let steps: Vec<Step> = read_puzzle_input(2015, 6)?
        .lines()
        .map(parse_step)
        .collect::<Result<Vec<Step>, String>>()?;
     
    let mut grid = vec![[false; 1000]; 1000];
    let mut grid_dimmed = vec![[0; 1000]; 1000];

    for step in steps {
        set_square(&mut grid, &step);
        set_square(&mut grid_dimmed, &step);
    }

    let active_lights = *grid.iter()
        .flatten()
        .counts()
        .get(&true).unwrap_or(&0usize);
    
    println!("After following the instructions {} lights are on", active_lights);

    let total_brightness: i32 = grid_dimmed.iter()
        .flatten()
        .sum();

    println!("After following the new instructions the total brightness is {}", total_brightness);

    Ok(())
}