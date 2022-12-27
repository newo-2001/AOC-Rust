use std::{fs, cmp};

use itertools::Itertools;
use regex::Regex;

type Point = (usize, usize);
type Grid<T> := Vec<[T; 1000]>;

fn add(p1: &Point, p2: &Point) -> Point {
    return (p1.0 + p2.0, p1.1 + p2.1);
}

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

impl Action {
    fn parse(str: &str) -> Result<Action, String> {
        return match str {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            "toggle" => Ok(Self::Toggle),
            _ => Err(format!("Invalid action: {}", str))
        };
    }
}

fn parse_point(str: &str) -> Result<Point, String>{
    let tokens = str.split_once(',')
        .ok_or("Point did not contain comma delimiter")?;

    fn parse_int(str: &str) -> Result<usize, String> {
        return str::parse::<usize>(str)
            .map_err(|err| err.to_string());
    }

    return Ok((parse_int(tokens.0)?, parse_int(tokens.1)?));
}

struct Step {
    top_left: Point,
    bottom_right: Point,
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
    let regex = Regex::new("(on|off|toggle) (\\d+,\\d+) (?:through) (\\d+,\\d+)").unwrap();
    let captures = regex.captures(str).ok_or("Failed to parse step")?;

    return Ok(Step {
        action: Action::parse(&captures[1])?,
        top_left: parse_point(&captures[2])?,
        bottom_right: add(&(1, 1), &parse_point(&captures[3])?),
    });
}

fn main() {
    let steps: Vec<Step> = fs::read_to_string("inputs/day_6.txt")
        .expect("Failed to read input file!")
        .lines()
        .map(parse_step)
        .collect::<Result<Vec<Step>, String>>()
        .unwrap_or_else(|err| panic!("{}", err));
     
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
}