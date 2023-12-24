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

/// Note that this function only parses whole integers and *converts* them to floats.
/// It does not parse anything with decimal places.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn f32(input: &str) -> TextParserResult<f32> {
    i64.map(|n| n as f32).parse(input)
}

/// Note that this function only parses whole integers and *converts* them to floats.
/// It does not parse anything with decimal places.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
pub fn f64(input: &str) -> TextParserResult<f64> {
    i64.map(|n| n as f64).parse(input)
}

#[macro_export]
macro_rules! string_enum {
    {$($name:literal => $value:expr),*} => {
        nom::branch::alt(($(
            nom::combinator::value($value, nom::bytes::complete::tag($name)),
        )*))
    };
}