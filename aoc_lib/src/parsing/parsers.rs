use nom::{
    branch::alt,
    combinator::{value, eof, opt},
    character::complete::{one_of, crlf, newline, self},
    Parser,
    sequence::{terminated, delimited}
};
use num::{FromPrimitive, Integer};

use crate::geometry::{CardinalDirection, Point2D};

use super::{TextParserResult, ignore};

pub fn optional_newline(input: &str) -> TextParserResult<()> {
    alt((
        ignore(crlf),
        ignore(newline),
        ignore(eof)
    ))(input)
}

pub fn direction(input: &str) -> TextParserResult<CardinalDirection> {
    alt((
        value(CardinalDirection::North, one_of("UuNn^")),
        value(CardinalDirection::East, one_of("RrEe>")),
        value(CardinalDirection::South, one_of("DdSsVv")),
        value(CardinalDirection::West, one_of("LlWw<"))
    ))(input)
}

pub fn point2d<T>(input: &str) -> TextParserResult<Point2D<T>>
    where T: Integer + FromPrimitive
{
    let sep = complete::char(',').and(opt(complete::char(' ')));
    let point = terminated(complete::i64, sep).and(complete::i64);
    
    delimited(opt(complete::char('(')), point, opt(complete::char(')')))
        .map(|(x, y)| Point2D(T::from_i64(x).unwrap(), T::from_i64(y).unwrap()))
        .parse(input)
}