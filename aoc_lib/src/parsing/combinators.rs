use std::ops::RangeFrom;

use nom::{
    Parser,
    combinator::{value, all_consuming},
    error::{ParseError, VerboseError},
    Slice, InputIter, InputLength, AsChar,
    multi::many_till, character::complete::{anychar, char}, bytes::complete::{take_until, self}, FindSubstring, InputTake, Compare, sequence::delimited
};
use tupletools::snd;

pub fn ignore<I, O, E, F>(parser: F) -> impl Parser<I, (), E>
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

pub fn skip_over<I, T, E>(tag: T) -> impl Parser<I, (), E>
    where E: ParseError<I>,
          I: FindSubstring<T> + InputTake + Compare<T>,
          T: InputLength + Clone
{
    value((), take_until(tag.clone()).and(complete::tag(tag)))
}

pub fn curly_brackets<I, O, E, F>(parser: F) -> impl Parser<I, O, E>
    where F: Parser<I, O, E>,
          E: ParseError<I>,
          I: Slice<RangeFrom<usize>> + InputIter,
          <I as InputIter>::Item: AsChar
{
    delimited(char('{'), parser, char('}'))
}

pub fn square_brackets<I, O, E, F>(parser: F) -> impl Parser<I, O, E>
    where F: Parser<I, O, E>,
          E: ParseError<I>,
          I: Slice<RangeFrom<usize>> + InputIter,
          <I as InputIter>::Item: AsChar
{
    delimited(char('['), parser, char(']'))
}

pub fn angle_brackets<I, O, E, F>(parser: F) -> impl Parser<I, O, E>
    where F: Parser<I, O, E>,
          E: ParseError<I>,
          I: Slice<RangeFrom<usize>> + InputIter,
          <I as InputIter>::Item: AsChar
{
    delimited(char('<'), parser, char('>'))
}

pub fn brackets<I, O, E, F>(parser: F) -> impl Parser<I, O, E>
    where F: Parser<I, O, E>,
          E: ParseError<I>,
          I: Slice<RangeFrom<usize>> + InputIter,
          <I as InputIter>::Item: AsChar
{
    delimited(char('('), parser, char(')'))
}

pub fn quoted<I, O, E, F>(parser: F) -> impl Parser<I, O, E>
    where F: Parser<I, O, E>,
          E: ParseError<I>,
          I: Slice<RangeFrom<usize>> + InputIter,
          <I as InputIter>::Item: AsChar
{
    delimited(char('"'), parser, char('"'))
}

pub fn parse_lines<'a, F, T, E>(parser: F, input: &'a str) -> Result<Vec<T>, E>
    where F: Fn(&'a str) -> Result<T, E>
{
    input.lines()
        .map(parser)
        .collect()
}

pub trait Runnable<'a, O> {
    fn run(self, input: &'a str) -> Result<O, super::ParseError<'a>>;
}

impl<'a, O, F> Runnable<'a, O> for F where
    F: Parser<&'a str, O, VerboseError<&'a str>>
{
    fn run(self, input: &'a str) -> Result<O, super::ParseError<'a>> {
        Ok(all_consuming(self).parse(input)?.1)
    }
}