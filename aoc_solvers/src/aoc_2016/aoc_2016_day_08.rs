use std::error::Error;

use aoc_lib::{geometry::{Grid, Dimensions, Light, InvalidGridAreaError, LightGrid}, parsing::{Runnable, self, parse_lines}};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{sequence::preceded, bytes::complete::tag, Parser, branch::alt};

struct RotateInstruction {
    index: usize,
    amount: usize
}

enum Instruction {
    Fill(Dimensions),
    RotateRow(RotateInstruction),
    RotateColumn(RotateInstruction)
}

impl Instruction {
    fn parse(input: &str) -> Result<Instruction, String> {
        let rotate_instruction = || parsing::usize.and(preceded(tag(" by "), parsing::usize))
            .map(|(index, amount)| RotateInstruction { index, amount });

        let rotate_row = preceded(tag("rotate row y="), rotate_instruction()).map(Instruction::RotateRow);
        let rotate_column = preceded(tag("rotate column x="), rotate_instruction()).map(Instruction::RotateColumn);
        let fill = preceded(tag("rect "), Dimensions::parse).map(Instruction::Fill);
        
        alt((fill, rotate_row, rotate_column)).run(input)
    }

    fn apply(self, mut grid: Grid<Light>) -> Result<Grid<Light>, InvalidGridAreaError> {
        match self {
            Instruction::Fill(dimensions) => {
                grid.sub_grid_mut(dimensions.into())?.fill(Light::On);
            },
            Instruction::RotateRow(RotateInstruction { index, amount }) => {
                let mut grid = grid.view_mut();
                let mut row = grid.get_row_mut(index)
                    .expect(&format!("Grid row index out of range: {}", index));

                rotate_slice(&mut row, amount);
            },
            Instruction::RotateColumn(RotateInstruction { index, amount }) => {
                let mut grid = grid.view_mut();
                let column = grid.get_column_mut(index)
                    .expect(&format!("Grid column index out of range: {}", index));

                let mut rotated = column.iter()
                    .map(|light| **light)
                    .collect_vec();

                rotate_slice(&mut rotated, amount);
                column.into_iter()
                    .zip(rotated)
                    .for_each(|(cell, value)| *cell = value)
            }
        }

        Ok(grid)
    }
}

fn rotate_slice<T>(slice: &mut [T], amount: usize) {
    slice.rotate_right(amount % slice.len());
}

fn final_grid(input: &str) -> Result<Grid<Light>, Box<dyn Error>> {
    let grid = Grid::<Light>::empty(Dimensions(50, 6));
    let instructions = parse_lines(Instruction::parse, input)?;

    let grid = instructions.into_iter()
        .try_fold(grid, |grid, instruction| instruction.apply(grid))?;

    Ok(grid)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = final_grid(input)?;
    Ok(Box::new(grid.count_lit()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    Ok(Box::new(final_grid(input)?))
}