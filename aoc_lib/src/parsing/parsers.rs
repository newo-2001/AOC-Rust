use nom::{
    branch::alt,
    combinator::eof,
    character::complete::{crlf, newline, self}, Parser,
};

use super::{TextParserResult, ignore};

pub fn optional_newline(input: &str) -> TextParserResult<()> {
    alt((
        ignore(crlf),
        ignore(newline),
        ignore(eof)
    ))(input)
}

pub fn usize(input: &str) -> TextParserResult<usize> {
    #[allow(clippy::cast_possible_truncation)]
    complete::u64.map(|n| n as usize).parse(input)
}

pub fn isize(input: &str) -> TextParserResult<isize> {
    #[allow(clippy::cast_possible_truncation)]
    complete::i64.map(|n| n as isize).parse(input)
}