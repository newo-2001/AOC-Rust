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

    fn format(&mut self) {
        while let Some((free_pos, &Block::Free(free_length))) = self.0.iter().find_position(|block| matches!(block, Block::Free(_))) {
            let Some((data_pos, &Block::Data { length: data_length, id })) = self.0
                .iter()
                .rev()
                .find_position(|block| matches!(block, Block::Data { .. }))
                .map(|(pos, block)| (self.0.len() - 1 - pos, block)) else { break };

            if data_pos < free_pos { break }

            match free_length.cmp(&data_length) {
                Ordering::Greater => {
                    self.0[free_pos] = Block::Free(free_length - data_length);
                    self.0[data_pos] = Block::Free(data_length);
                    self.0.insert(free_pos, Block::Data { length: data_length, id });
                },
                Ordering::Equal => {
                    self.0[free_pos] = Block::Data { length: data_length, id  };
                    self.0[data_pos] = Block::Free(data_length);
                },
                Ordering::Less => {
                    self.0[free_pos] = Block::Data { length: free_length, id };
                    self.0[data_pos] = Block::Data { length: data_length - free_length, id }
                }
            }
        }
    }

    fn checksum(&self) -> usize { 
        self.0
            .iter()
            .map_while(|block| match block {
                Block::Free(_) => None,
                &Block::Data { id, length } => Some(repeat_n(id, length as usize))
            })
            .flatten()
            .enumerate()
            .map(|(i, id)| i * id)
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
    disk.format();

    Ok(Box::new(disk.checksum()))
}