use std::ops::{Range, RangeFrom};

use nom::{
    Parser,
    IResult,
    combinator::{value, eof},
    error::ParseError,
    branch::alt,
    character::complete::{crlf, newline},
    Slice, Compare, InputIter, InputLength, AsChar,
};

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