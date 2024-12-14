use mathru::{algebra::{abstr::AbsDiffEq, linear::{matrix::{General, Solve}, vector::Vector}}, matrix, vector};
use nom::{bytes::complete::tag, character::complete::{char, newline, u64}, multi::{count, separated_list0}, sequence::{delimited, preceded, separated_pair, tuple}, Parser};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use yuki::{parsing::{NomError, Parsable, ParserExt, ParsingResult}, spatial::point::Point};

use crate::SolverResult;

#[derive(Debug)]
struct ClawMachine {
    a: Point<u64>,
    b: Point<u64>,
    prize: Point<u64>
}

fn parse_button<'a>(button: char) -> impl Parser<&'a str, Point<u64>, NomError<'a>> {
    delimited(
        delimited(tag("Button "), char(button), tag(": X+")),
        separated_pair(u64, tag(", Y+"), u64),
        newline
    )
    .map(Point::from)
}

impl<'a> Parsable<'a> for ClawMachine {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        tuple((
            parse_button('A'),
            parse_button('B'),
            preceded(
                tag("Prize: X="),
                separated_pair(u64, tag(", Y="), u64)
            ).map(Point::from)
        ))
        .map(|(a, b, prize)| Self { a, b, prize })
        .parse(input)
    }
}

fn parse_claw_machines(input: &str) -> ParsingResult<Vec<ClawMachine>> {
    separated_list0(
        count(newline, 2),
        ClawMachine::parse
    )
    .parse(input)
}

impl ClawMachine {
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn prize_cost(&self) -> Option<u64> {
        // Solve the system of linear equations
        let system= matrix![
            self.a.x as f64, self.b.x as f64;
            self.a.y as f64, self.b.y as f64
        ];

        let constants = vector![self.prize.x as f64; self.prize.y as f64];
        let solution = system.solve(&constants).ok()?;
        let a = solution[0].round();
        let b = solution[1].round();

        // The epsilon is finicky
        // I'll implement my own linear algebra solution with integers at some point
        (solution.abs_diff_eq(&vector![a; b], 0.0001))
            .then_some(a as u64 * 3 + b as u64)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let cost: u64 = parse_claw_machines.run(input)?
        .par_iter()
        .filter_map(ClawMachine::prize_cost)
        .sum();

    Ok(Box::new(cost))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    const OFFSET: u64 = 10_000_000_000_000;

    let cost: u64 = parse_claw_machines.run(input)?
        .into_par_iter()
        .map(|ClawMachine { a, b, prize }| ClawMachine {
            a, b,
            prize: prize + Point::new(OFFSET, OFFSET)
        })
        .filter_map(|machine| machine.prize_cost())
        .sum();

    Ok(Box::new(cost))
}