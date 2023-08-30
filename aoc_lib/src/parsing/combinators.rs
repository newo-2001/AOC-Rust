use std::ops::RangeFrom;

use nom::{
    Parser,
    IResult,
    combinator::value,
    error::{ParseError, VerboseError},
    Slice, InputIter, InputLength, AsChar,
    multi::many_till, character::complete::anychar
};
use tupletools::snd;

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

pub fn parse_lines<'a, F, T, E>(parser: F, input: &'a str) -> Result<Vec<T>, E>
    where F: Fn(&'a str) -> Result<T, E>
{
    input.lines()
        .map(parser)
        .collect()
}

pub trait Runnable<'a, O> {
    fn run(&mut self, input: &'a str) -> Result<O, super::ParseError<'a>>;
}

impl<'a, O, F> Runnable<'a, O> for F where
    F: Parser<&'a str, O, VerboseError<&'a str>>
{
    fn run(&mut self, input: &'a str) -> Result<O, super::ParseError<'a>> {
        Ok(self.parse(input)?.1)
    }
}