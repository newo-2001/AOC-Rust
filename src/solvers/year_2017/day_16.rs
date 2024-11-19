use ahash::{HashMap, HashMapExt};
use aoc_lib::parsing::{ParseError, TextParser, usize};
use crate::SolverResult;
use itertools::Itertools;
use nom::{branch::alt, multi::separated_list0, character::complete::{char, anychar}, sequence::{preceded, separated_pair}, Parser};

#[derive(Clone, Copy)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

impl Move {
    fn perform(self, programs: &mut [char]) {
        match self {
            Move::Spin(length) => programs.rotate_right(length),
            Move::Exchange(a, b) => programs.swap(a, b),
            Move::Partner(a, b) => {
                let a = programs.iter().position(|x| x == &a).unwrap();
                let b = programs.iter().position(|x| x == &b).unwrap();
                programs.swap(a, b);
            }
        }
    }
}

fn parse_moves(input: &str) -> Result<Vec<Move>, ParseError> {
    let parse_move = alt((
        preceded(char('s'), usize).map(Move::Spin),
        preceded(char('x'), separated_pair(usize, char('/'), usize))
            .map(|(a, b)| Move::Exchange(a, b)),
        preceded(char('p'), separated_pair(anychar, char('/'), anychar))
            .map(|(a, b)| Move::Partner(a, b))
    ));
    
    separated_list0(char(','), parse_move).run(input)
}

fn perform_dance<'a>(programs: &mut [char], moves: impl IntoIterator<Item=&'a Move>) {
    for dance_move in moves {
        dance_move.perform(programs);
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut programs = ('a'..='p').collect_vec();
    perform_dance(&mut programs, &parse_moves(input)?);

    Ok(Box::new(programs.into_iter().collect::<String>()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    const DANCES: usize = 1_000_000_000;

    let moves = parse_moves(input)?;
    let mut programs  = ('a'..='p').collect_vec();

    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut dance_number = 0;
    while dance_number < DANCES {
        let order = programs.iter().copied().collect(); 
        if let Err(err) = cache.try_insert(order, dance_number) {
            let loop_size = dance_number - err.entry.get();
            dance_number = DANCES - ((DANCES - dance_number) % loop_size);
        };

        perform_dance(&mut programs, &moves);
        dance_number += 1;
    }

    Ok(Box::new(programs.into_iter().collect::<String>()))
}