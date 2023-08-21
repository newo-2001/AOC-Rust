use std::ops::RangeFrom;

use nom::{
    Parser,
    IResult,
    combinator::{value, eof, opt},
    error::{ParseError, VerboseError},
    branch::alt,
    character::complete::{crlf, newline, anychar, one_of, self},
    Slice, InputIter, InputLength, AsChar, multi::many_till, sequence::{terminated, delimited}
};
use num::{Integer, FromPrimitive};
use tupletools::snd;

use crate::geometry::{CardinalDirection, Point2D};

pub type TextParserResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

// Combinators

pub fn ignore<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, (), E>
    where E : ParseError<I>,
          F : Parser<I, O, E> {
    value((), parser)
}

pub fn skip_until<I, O, E, F, C>(parser: F) -> impl Parser<I, O, E>
    where E: ParseError<I>,
          F: Parser<I, O, E>,
          I: Clone + InputIter<Item = C> + Slice<RangeFrom<usize>> + InputLength,
          C: AsChar
{
    many_till(anychar, parser).map(snd)
}

pub fn run<'a, O, F>(parser: &mut F, input: &'a str) -> Result<O, String>
    where F: Parser<&'a str, O, VerboseError<&'a str>>
{
    Ok(parser.parse(input).map_err(|err| err.to_string())?.1)
}

// Parsers

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