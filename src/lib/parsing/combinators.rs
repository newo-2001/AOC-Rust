use std::fmt::Display;
use crate::between;

use nom::{
    AsChar, Compare, FindSubstring, Input, Parser, bytes::complete::{self, take_until}, character::complete::{anychar, char, line_ending}, combinator::{all_consuming, value}, error::ParseError, multi::{many_till, separated_list0}, sequence::delimited
};
use yuki::tuples::{snd, Snd};

pub fn skip_until<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E>
    where E: ParseError<I>,
          F: Parser<I, Output = O, Error = E>,
          I: Clone + Input,
          I::Item: AsChar
{
    many_till(anychar, parser).map(snd)
}

pub fn skip_over<I, T, E>(tag: T) -> impl Parser<I, Output = (), Error = E>
    where E: ParseError<I>,
          I: FindSubstring<T> + Compare<T> + Input,
          T: Clone + Input
{
    value((), take_until(tag.clone()).and(complete::tag(tag)))
}

pub fn curly_brackets<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E>
    where F: Parser<I, Output = O, Error = E>,
          E: ParseError<I>,
          I: Input,
          I::Item: AsChar
{
    delimited(char('{'), parser, char('}'))
}

pub fn square_brackets<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E>
    where F: Parser<I, Output = O, Error = E>,
          E: ParseError<I>,
          I: Input,
          I::Item: AsChar
{
    delimited(char('['), parser, char(']'))
}

pub fn angle_brackets<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E>
    where F: Parser<I, Output = O, Error = E>,
          E: ParseError<I>,
          I: Input,
          I::Item: AsChar
{
    delimited(char('<'), parser, char('>'))
}

pub fn parens<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E>
    where F: Parser<I, Output = O, Error = E>,
          E: ParseError<I>,
          I: Input,
          I::Item: AsChar
{
    delimited(char('('), parser, char(')'))
}

pub fn quoted<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E>
    where F: Parser<I, Output = O, Error = E>,
          E: ParseError<I>,
          I: Input,
          I::Item: AsChar
{
    between!(char('"'), parser)
}

pub fn map2<I, O, E, F, M, O1, O2>(parser: F, mapper: M) -> impl Parser<I, Output = O, Error = E>
    where F: Parser<I, Output = (O1, O2), Error = E>,
          M: Fn(O1, O2) -> O,
          E: ParseError<I>
{
    parser.map(move |(a, b)| mapper(a, b))
}

pub trait Map2<I, O1, O2, E>: Parser<I, Output = (O1, O2), Error = E> + Sized
    where E: ParseError<I>
{
    fn map2<O, M>(self, mapper: M) -> impl Parser<I, Output = O, Error = E>
        where M: Fn(O1, O2) -> O
    {
        map2(self, mapper)
    }
}

impl<I, O1, O2, E, F> Map2<I, O1, O2, E> for F
    where F: Parser<I, Output = (O1, O2), Error = E>,
          E: ParseError<I>
{}

pub fn map3<I, O, E, F, M, O1, O2, O3>(parser: F, mapper: M) -> impl Parser<I, Output = O, Error = E>
    where F: Parser<I, Output = (O1, O2, O3), Error = E>,
          M: Fn(O1, O2, O3) -> O,
          E: ParseError<I>
{
    parser.map(move |(a, b, c)| mapper(a, b, c))
}

pub trait Map3<I, O1, O2, O3, E>: Parser<I, Output = (O1, O2, O3), Error = E> + Sized
    where E: ParseError<I>
{
    fn map3<O, M>(self, mapper: M) -> impl Parser<I, Output = O, Error = E>
        where M: Fn(O1, O2, O3) -> O
    {
        map3(self, mapper)
    }
}

impl<I, O1, O2, O3, E, F> Map3<I, O1, O2, O3, E> for F
    where F: Parser<I, Output = (O1, O2, O3), Error = E>,
          E: ParseError<I>
{}

pub fn lines<I, O, E, F>(parser: F) -> impl Parser<I, Output = Vec<O>, Error = E>
    where F: Parser<I, Output = O, Error = E>,
          E: ParseError<I>,
          I: Clone + Compare<&'static str> + Input
{
    separated_list0(line_ending, parser)
}

pub fn run<I, O, E, F>(parser: F, input: I) -> Result<O, super::ParseError>
    where F: Parser<I, Output = O, Error = E> + Sized,
          E: ParseError<I>,
          nom::Err<E>: Display,
          I: Input
{
    Ok(
        all_consuming(parser)
            .parse(input)
            .map_err(|e| super::ParseError::new(e.to_string()))?
            .snd()
    )
}

#[macro_export]
macro_rules! between {
    ($delimiter: expr, $parser: expr) => {
        nom::sequence::delimited($delimiter, $parser, $delimiter)
    };
}

#[macro_export]
macro_rules! ignore {
    ($($parser: expr),*) => {
        nom::combinator::value((), ($($parser,)*))
    };
}