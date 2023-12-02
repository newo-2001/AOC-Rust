use std::collections::HashMap;
use aoc_lib::{parsing::{TextParserResult, TextParser, ParseError, parse_lines}, functional::swap, string_enum};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{bytes::complete::tag, Parser, sequence::{separated_pair, delimited}, character::complete::{u32, char}, multi::{separated_list0, separated_list1}};
use anyhow::Result;
use maplit::hashmap;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Blue
}

impl Color {
    fn parse(input: &str) -> TextParserResult<Self> {
        string_enum! {
            "red" => Self::Red,
            "blue" => Self::Blue,
            "green" => Self::Green
        }.parse(input)
    }
}

struct Hand(HashMap<Color, u32>);

impl Hand {
    fn parse(input: &str) -> TextParserResult<Self> {
        let cubes = || separated_pair(u32, char(' '), Color::parse).map(swap);
        separated_list1(tag(", "), cubes())
            .map(|cubes| Self(cubes.into_iter().collect()))
            .parse(input)
    }

    fn cubes(&self) -> impl Iterator<Item=(&Color, &u32)> {
        self.0.iter()
    }
}

struct Game {
    hands: Vec<Hand>,
    number: u32
}

impl Game {
    fn parse(input: &str) -> Result<Self, ParseError> {
        delimited(tag("Game "), u32, tag(": "))
            .and(separated_list0(tag("; "), Hand::parse))
            .map(|(number, hands)| Self { hands, number })
            .run(input)
    }

    fn is_possible(&self, cubes: &HashMap<Color, u32>) -> bool {
        self.hands.iter()
            .all(|hand| hand.cubes().all(|(color, amount)| {
                amount <= cubes.get(color).unwrap()
            }))
    }

    fn min_possible_power(&self) -> u32 {
        self.hands.iter()
            .flat_map(Hand::cubes)
            .into_group_map()
            .into_values()
            .map(|color| *color.into_iter().max().unwrap_or(&0))
            .product()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let total_cubes = hashmap! {
        Color::Red => 12,
        Color::Green => 13,
        Color::Blue => 14
    };

    let result: u32 = parse_lines(Game::parse, input)?
        .into_iter()
        .filter(|game| game.is_possible(&total_cubes))
        .map(|game| game.number)
        .sum();

    Ok(Box::new(result))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let result: u32 = parse_lines(Game::parse, input)?
        .iter()
        .map(Game::min_possible_power)
        .sum();

    Ok(Box::new(result))
}