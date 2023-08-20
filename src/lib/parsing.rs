use std::ops::RangeFrom;

use nom::{
    Parser,
    IResult,
    combinator::{value, eof},
    error::{ParseError, VerboseError},
    branch::alt,
    character::complete::{crlf, newline, anychar, one_of},
    Slice, InputIter, InputLength, AsChar, multi::many_till
};
use tupletools::snd;

use crate::geometry::CardinalDirection;

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
        value(CardinalDirection::North, one_of("UN")),
        value(CardinalDirection::East, one_of("RE")),
        value(CardinalDirection::South, one_of("DS")),
        value(CardinalDirection::West, one_of("LW"))
    ))(input)
}