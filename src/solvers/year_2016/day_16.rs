use std::{fmt::Display, iter::once};

use aoc_lib::{parsing::InvalidTokenError, math::Bit, functional::repeat_apply_while};
use crate::SolverResult;
use itertools::Itertools;
use num::Integer;

#[derive(Clone)]
struct Data(Vec<Bit>);

impl Data {
    fn parse(input: &str) -> Result<Self, InvalidTokenError<char>> {
        Ok(Data(input.chars().map(TryInto::try_into).try_collect()?))
    }

    fn len(&self) -> usize { self.0.len() }

    fn extend_length(&mut self, length: usize) {
        while self.len() < length {
            let b = self.clone().0
                .into_iter()
                .rev()
                .map(Bit::invert);

            self.0.extend(once(Bit::Off).chain(b));
        }

        _ = self.0.split_off(length);
    }

    fn checksum(&self) -> Self {
        fn checksum(data: Data) -> Data {
            data.0.into_iter()
                .tuples()
                .map(|(a, b)| (a == b).into())
                .collect()
        }

        repeat_apply_while(self.clone(), checksum, |data| data.len().is_even())
    }
}

impl FromIterator<Bit> for Data {
    fn from_iter<T: IntoIterator<Item = Bit>>(iter: T) -> Self {
        Data(iter.into_iter().collect())
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self.0.iter().map(|&bit| Bit::digit(bit)).collect::<String>();
        write!(f, "{str}")
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut data = Data::parse(input)?;
    data.extend_length(272);

    Ok(Box::new(data.checksum()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut data = Data::parse(input)?;
    data.extend_length(35_651_584);

    Ok(Box::new(data.checksum()))
}