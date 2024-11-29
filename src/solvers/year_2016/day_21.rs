use std::ops::RangeInclusive;

use ahash::HashMap;
use aoc_lib::parsing::{ParseError, parse_lines, TextParser, usize};
use crate::SolverResult;
use itertools::Itertools;
use nom::{combinator::opt, branch::alt, sequence::{delimited, preceded}, bytes::complete::tag, Parser, character::complete::{anychar, char}};
use thiserror::Error;

enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBasedOnPositionOfLetter(char),
    ReverseSlice(RangeInclusive<usize>),
    Move(usize, usize)
}

fn rotation_index(index: usize) -> usize { index + 1 + usize::from(index >= 4) }

impl Instruction {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let swap_positions = preceded(tag("swap position "), usize)
            .and(preceded(tag(" with position "), usize))
            .map(|(first, second)| Self::SwapPositions(first, second));

        let swap_letters = preceded(tag("swap letter "), anychar)
            .and(preceded(tag(" with letter "), anychar))
            .map(|(first, second)| Self::SwapLetters(first, second));

        let reverse_slice = preceded(tag("reverse positions "), usize)
            .and(preceded(tag(" through "), usize))
            .map(|(start, end) |Self::ReverseSlice(start..=end));
        
        let move_position = preceded(tag("move position "), usize)
            .and(preceded(tag(" to position "), usize))
            .map(|(from, to)| Self::Move(from, to));

        let steps = || preceded(tag(" step"), opt(char('s')));
        let rotate_left = delimited(tag("rotate left "), usize, steps()).map(Self::RotateLeft);
        let rotate_right = delimited(tag("rotate right "), usize, steps()).map(Self::RotateRight);
        let rotate_position = preceded(tag("rotate based on position of letter "), anychar).map(Self::RotateBasedOnPositionOfLetter);

        alt((
            swap_positions, swap_letters, reverse_slice, move_position,
            rotate_left, rotate_right, rotate_position
        )).run(input)
    }

    fn apply(self, mut password: Vec<char>, mode: Mode) -> Result<Vec<char>, PasswordError> {
        match (mode, self) {
            | (_, Self::SwapPositions(first, second)) => { password.swap(first, second); },
            | (_, Self::SwapLetters(first, second)) => {
                let (first, second) = password.iter().positions(|c| [first, second].contains(c))
                    .collect_tuple()
                    .ok_or(PasswordError::DuplicateCharacter)?; // Could actually also be `InvalidCharacter`

                password.swap(first, second);
            },
            | (Mode::Unscramble, Self::RotateRight(amount))
            | (Mode::Scramble, Self::RotateLeft(amount)) => { password.rotate_left(amount); }, 
            | (Mode::Unscramble, Self::RotateLeft(amount))
            | (Mode::Scramble, Self::RotateRight(amount)) => { password.rotate_right(amount); },
            | (Mode::Scramble, Self::RotateBasedOnPositionOfLetter(letter)) => {
                let index = password.iter()
                    .position(|&c| c == letter)
                    .ok_or(PasswordError::InvalidCharacter(letter))?;

                let rotations = rotation_index(index) % password.len();
                password.rotate_right(rotations);
            },
            // Could use deduplication
            | (Mode::Unscramble, Self::RotateBasedOnPositionOfLetter(letter)) => {
                let index = password.iter()
                    .position(|&c| c == letter)
                    .ok_or(PasswordError::InvalidCharacter(letter))?;
                
                // Could be calculated at compile-time, but the gain is nihil
                let lookup: HashMap<usize, usize> = (0..password.len())
                    .map(|index| ((rotation_index(index) + index) % password.len(), index))
                    .collect();
                
                // No need to branch here, was too lazy to figure out how to get the index to wrap
                let original_index = *lookup.get(&index).ok_or(PasswordError::InvalidRotationIndex)?;
                if original_index >= index { password.rotate_right(original_index - index); }
                else { password.rotate_left(index - original_index); }
            },
            | (_, Self::ReverseSlice(range)) => { password[range].reverse(); },
            | (Mode::Unscramble, Self::Move(to, from))
            | (Mode::Scramble, Self::Move(from, to)) => {
                let removed = password.remove(from);
                password.insert(to, removed);
            }
        };

        Ok(password)
    }
}

#[derive(Debug, Error)]
enum PasswordError {
    #[error("The character `{0}` did not appear in the password")]
    InvalidCharacter(char),
    #[error("A character in the password appeared more than once")]
    DuplicateCharacter,
    #[error("Rotation based on character position is not bijective for input length")]
    InvalidRotationIndex
}

enum Mode {
    Scramble,
    Unscramble
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let seed = "abcdefgh".chars().collect_vec();
    let password: String = parse_lines(Instruction::parse, input)?
        .into_iter()
        .try_fold(seed, |password, instruction| instruction.apply(password, Mode::Scramble))?
        .into_iter()
        .collect();

    Ok(Box::new(password))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let seed = "fbgdceah".chars().collect_vec();
    let password: String = parse_lines(Instruction::parse, input)?
        .into_iter()
        .try_rfold(seed, |password, instruction| instruction.apply(password, Mode::Unscramble))?
        .into_iter()
        .collect();

    Ok(Box::new(password))
}