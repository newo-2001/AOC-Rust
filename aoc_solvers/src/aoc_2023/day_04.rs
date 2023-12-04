use aoc_lib::{parsing::{TextParser, ParseError, parse_lines, usize}, iteration::ExtraIter, between, ignore};
use aoc_runner_api::SolverResult;
use nom::{sequence::{preceded, separated_pair}, bytes::complete::tag, character::complete::{char, u32, space1}, multi::separated_list1, Parser};
use num::traits::Pow;

struct ScratchCard {
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>
}

impl ScratchCard {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let numbers = || separated_list1(space1, u32);
        let prefix = ignore!(tag("Card"), space1, usize, char(':'), space1);

        preceded(prefix, separated_pair(
            numbers(), between!(space1, char('|')), numbers()
        )).map(|(winning_numbers, numbers)| Self { numbers, winning_numbers })
            .run(input)
    }

    fn winning_matches(&self) -> usize {
        self.numbers.iter()
            .filter(|x| self.winning_numbers.contains(x))
            .count()
    }

    fn points(&self) -> i32 {
        match self.winning_matches() {
            0 => 0,
            matches => 2.pow(matches - 1)
        }
    }

    fn new_cards(&self, card_number: usize) -> impl Iterator<Item=usize> + '_ {
        card_number + 1..=card_number + self.winning_matches()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let total_points: i32 = parse_lines(ScratchCard::parse, input)?
        .iter()
        .sum_by(ScratchCard::points);

    Ok(Box::new(total_points))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let cards = parse_lines(ScratchCard::parse, input)?;

    let mut copies: Box<[u32]> = vec![1; cards.len()].into_boxed_slice();
    for (card_number, card) in cards.iter().enumerate() {
        let new_amount = *copies.get(card_number).unwrap_or(&0);
        for new_card in card.new_cards(card_number) {
            if let Some(amount) = copies.get_mut(new_card) {
                *amount += new_amount;
            }
        }
    }

    Ok(Box::new(copies.iter().sum::<u32>()))
}