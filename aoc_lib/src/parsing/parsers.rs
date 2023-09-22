use nom::{
    character::complete::{u64, i64}, Parser,
};

use super::TextParserResult;

pub fn usize(input: &str) -> TextParserResult<usize> {
    #[allow(clippy::cast_possible_truncation)]
    u64.map(|n| n as usize).parse(input)
}

pub fn isize(input: &str) -> TextParserResult<isize> {
    #[allow(clippy::cast_possible_truncation)]
    i64.map(|n| n as isize).parse(input)
}