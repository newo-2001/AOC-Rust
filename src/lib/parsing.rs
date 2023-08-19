use std::ops::{Range, RangeFrom};

use nom::{
    Parser,
    IResult,
    combinator::{value, eof},
    error::{ParseError, VerboseError},
    branch::alt,
    character::complete::{crlf, newline, anychar},
    Slice, Compare, InputIter, InputLength, AsChar, multi::many_till,
};
use tupletools::snd;

pub type TextParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

pub fn ignore<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, (), E>
    where E : ParseError<I>,
          F : Parser<I, O, E> {
    value((), parser)
}

pub fn optional_newline<I, E>(input: I) -> IResult<I, (), E>
    where E : ParseError<I>,
          I : Clone + Slice<RangeFrom<usize>> + Slice<Range<usize>> + Compare<&'static str> + InputIter + InputLength,
          <I as InputIter>::Item: AsChar {
    alt((
        ignore(crlf),
        ignore(newline),
        ignore(eof)
    ))(input)
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