use std::{cmp::Ordering, fmt::Display, iter::repeat_n};

use anyhow::{Context, Result};
use itertools::Itertools;

use crate::SolverResult;

#[derive(Debug, Clone, Copy)]
enum Block {
    Free(u32),
    Data {
        id: usize,
        length: u32
    }
}

#[derive(Debug)]
struct DiskMap(Vec<Block>);

fn move_data(
    data: &mut Vec<Block>,
    free_pos: usize,
    free_length: u32,
    data_pos: usize,
    data_length: u32,
    id: usize
) {
    match free_length.cmp(&data_length) {
        Ordering::Greater => {
            data[free_pos] = Block::Free(free_length - data_length);
            data[data_pos] = Block::Free(data_length);
            data.insert(free_pos, Block::Data { length: data_length, id });
        },
        Ordering::Equal => {
            data[free_pos] = Block::Data { length: data_length, id };
            data[data_pos] = Block::Free(data_length);
        },
        Ordering::Less => {
            data[free_pos] = Block::Data { length: free_length, id };
            data[data_pos] = Block::Data { length: data_length - free_length, id }
        }
    }
}

impl DiskMap {
    fn parse(input: &str) -> Result<Self> {
        let blocks = input
            .chars()
            .map(|char| char.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .context("Failed to parse digit")?
            .into_iter()
            .array_chunks::<2>();

        let mut disk: Vec<Block> = blocks
            .clone()
            .enumerate()
            .flat_map(|(id, [data, free])| [
                Block::Data { length: data, id },
                Block::Free(free)
            ])
            .collect();

        if let Some(remainder) = blocks.into_remainder() {
            if let [length] = remainder.as_slice() {
                disk.push(Block::Data { length: *length, id: disk.len() / 2 });
            }
        }

        Ok(Self(disk))
    }

    fn format_with_fragmentation(&mut self) {
        while let Some((free_pos, &Block::Free(free_length))) = self.0.iter().find_position(|block| matches!(block, Block::Free(_))) {
            let Some((data_pos, &Block::Data { length: data_length, id })) = self.0
                .iter()
                .rev()
                .find_position(|block| matches!(block, Block::Data { .. }))
                .map(|(pos, block)| (self.0.len() - 1 - pos, block)) else { break };

            if data_pos < free_pos { break }

            move_data(&mut self.0, free_pos, free_length, data_pos, data_length, id);
        }
    }

    fn format_without_fragmentation(&mut self) {
        let max_id = self.0.len() / 2;
        for data_id in (0..=max_id).rev() {
            let Some((data_pos, data_length)) = self.0
                .iter()
                .enumerate()
                .find_map(|(pos, block)| match block {
                    &Block::Data { id, length } if id == data_id => Some((pos, length)),
                    Block::Data { ..} | Block::Free(_) => None
                }) else { return };

            if let Some((free_pos, free_length)) = self.0
                .iter()
                .enumerate()
                .find_map(|(pos, block)| match block {
                    &Block::Free(size) if size >= data_length && pos < data_pos => Some((pos, size)),
                    Block::Free(_) | Block::Data { .. } => None
                })
            {
                move_data(&mut self.0, free_pos, free_length, data_pos, data_length, data_id);
            }
        }
    }

    fn checksum(&self) -> usize { 
        self.0
            .iter()
            .flat_map(|&block| match block {
                Block::Free(length) => repeat_n(None, length as usize),
                Block::Data { id, length } => repeat_n(Some(id), length as usize)
            })
            .enumerate()
            .filter_map(|(i, id)| Some(i * id?))
            .sum()
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &block in &self.0 {
            match block {
                Block::Free(size) => write!(f, "{}", ".".repeat(size as usize))?,
                Block::Data { length, id } => write!(f, "{}", id.to_string().repeat(length as usize))?,
            }
        }

        Ok(())
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut disk = DiskMap::parse(input)?;
    disk.format_with_fragmentation();

    Ok(Box::new(disk.checksum()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut disk = DiskMap::parse(input)?;
    disk.format_without_fragmentation();

    Ok(Box::new(disk.checksum()))
}