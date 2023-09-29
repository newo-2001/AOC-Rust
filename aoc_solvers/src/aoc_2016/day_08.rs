use std::error::Error;

use aoc_lib::{geometry::{Dimensions, grid::{Grid, InvalidGridAreaError, GridLikeMut, BitGrid}}, parsing::{Runnable, parse_lines, ParseError, usize}, math::Bit};
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
    fn parse(input: &str) -> Result<Instruction, ParseError> {
        let rotate_instruction = || usize.and(preceded(tag(" by "), usize))
            .map(|(index, amount)| RotateInstruction { index, amount });

        let rotate_row = preceded(tag("rotate row y="), rotate_instruction()).map(Instruction::RotateRow);
        let rotate_column = preceded(tag("rotate column x="), rotate_instruction()).map(Instruction::RotateColumn);
        let fill = preceded(tag("rect "), Dimensions::parse).map(Instruction::Fill);
        
        alt((fill, rotate_row, rotate_column)).run(input)
    }

    fn apply(self, mut grid: Grid<Bit>) -> Result<Grid<Bit>, InvalidGridAreaError> {
        match self {
            Instruction::Fill(dimensions) => {
                grid.sub_grid_mut(dimensions.into())?.fill(Bit::On);
            },
            Instruction::RotateRow(RotateInstruction { index, amount }) => {
                let row = grid.get_row_mut(index)
                    .unwrap_or_else(|| panic!("Grid row index out of range: {}", index));

                rotate_slice(row, amount);
            },
            Instruction::RotateColumn(RotateInstruction { index, amount }) => {
                let column = grid.get_column_mut(index)
                    .unwrap_or_else(|| panic!("Grid column index out of range: {}", index));

                let mut rotated = column.iter()
                    .map(|light| **light)
                    .collect_vec();

                rotate_slice(&mut rotated, amount);
                column.into_iter()
                    .zip(rotated)
                    .for_each(|(cell, value)| *cell = value);
            }
        }

        Ok(grid)
    }
}

fn rotate_slice<T>(slice: &mut [T], amount: usize) {
    slice.rotate_right(amount % slice.len());
}

fn final_grid(input: &str) -> Result<Grid<Bit>, Box<dyn Error + '_ + Send + Sync>> {
    let grid = Grid::<Bit>::empty(Dimensions(50, 6));
    let instructions = parse_lines(Instruction::parse, input)?;

    let grid = instructions.into_iter()
        .try_fold(grid, |grid, instruction| instruction.apply(grid))?;

    Ok(grid)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = final_grid(input)?;
    Ok(Box::new(grid.pop_count()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    Ok(Box::new(final_grid(input)?))
}