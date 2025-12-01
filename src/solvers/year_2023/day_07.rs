use std::{cmp::{Ordering, min}, convert::TryInto};

use aoc_lib::{parsing::{TextParserResult, ParseError, parse_lines, TextParser, InvalidTokenError}, iteration::ExtraIter};
use crate::SolverResult;
use itertools::Itertools;
use nom::{Parser, multi::count, character::complete::{char, u16, anychar}, sequence::separated_pair, combinator::map_res};
use rayon::iter::{ParallelBridge, ParallelIterator};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace, King, Queen, Jack,
    Ten, Nine, Eight, Seven, Six,
    Five, Four, Three, Two, Joker
}

impl Card {
    fn parse(input: &str) -> TextParserResult<'_, Self> {
        map_res(anychar, |char| Ok(match char {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => return Err(InvalidTokenError(char))
        })).parse(input)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandKind {
    fn from_counts(counts: &[usize]) -> Self {
        match counts {
            [5] => Self::FiveOfAKind,
            [1, 4] => Self::FourOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 1, 3] => Self::ThreeOfAKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => unreachable!()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hand {
    bid: u16,
    cards: [Card; 5]
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind().cmp(&other.kind()).then_with(|| {
            self.cards.into_iter()
                .zip(other.cards)
                .find_map(|(us, them)| match us.cmp(&them) {
                    Ordering::Equal => None,
                    ordering => Some(ordering)
                }).unwrap_or(Ordering::Equal)
        })
    }
}

impl Hand {
    fn parse(input: &str) -> Result<Self, ParseError> {
        separated_pair(
            map_res(count(Card::parse, 5), TryInto::try_into),
            char(' '),
            u16
        ).map(|(cards, bid)|
            Self { bid, cards }
        ).run(input)
    }

    fn kind(self) -> HandKind {
        let mut counts = self.cards.into_iter().counts();
        let natural_counts = counts.clone()
            .into_values()
            .sorted()
            .collect_vec();

        let jokers = counts.remove(&Card::Joker).unwrap_or(0);
        let mut dirty_counts = counts.into_values().collect_vec();
        dirty_counts.sort_unstable();
        
        match dirty_counts.last_mut() {
            Some(highest) => *highest += jokers,
            None => return HandKind::FiveOfAKind
        };

        min(
            HandKind::from_counts(&dirty_counts),
            HandKind::from_counts(&natural_counts)
        )
    }
}

fn winnings(hands: impl IntoIterator<Item=Hand>) -> u32 {
    hands.into_iter()
        .sorted()
        .rev()
        .enumerate()
        .par_bridge()
        .map(|(rank, hand)| (u32::try_from(rank).unwrap() + 1) * u32::from(hand.bid))
        .sum()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let total_winnings: u32 = winnings(parse_lines(Hand::parse, input)?);
    Ok(Box::new(total_winnings))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut hands = parse_lines(Hand::parse, input)?;

    hands.iter_mut()
        .flat_map(|hand| hand.cards.iter_mut())
        .replace_all(Card::Jack, Card::Joker);

    Ok(Box::new(winnings(hands)))    
}