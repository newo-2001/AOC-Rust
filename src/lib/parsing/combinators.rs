use std::{fmt::Display, ops::{RangeFrom, Range, RangeTo}};
use crate::between;

use nom::{
    combinator::{value, all_consuming},
    error::ParseError,
    character::complete::{anychar, char, line_ending},
    bytes::complete::{take_until, self},
    Slice, InputIter, InputLength, AsChar,
    FindSubstring, InputTake, Compare, Parser,
    sequence::delimited,
    multi::{many_till, separated_list0}
};
use yuki::tuples::{snd, Snd};

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

pub fn parens<I, O, E, F>(parser: F) -> impl Parser<I, O, E>
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
    between!(char('"'), parser)
}

pub fn map2<I, O, E, F, M, O1, O2>(parser: F, mapper: M) -> impl Parser<I, O, E>
    where F: Parser<I, (O1, O2), E>,
          M: Fn(O1, O2) -> O,
          E: ParseError<I>
{
    parser.map(move |(a, b)| mapper(a, b))
}

pub trait Map2<I, O1, O2, E>: Parser<I, (O1, O2), E> + Sized
    where E: ParseError<I>
{
    fn map2<O, M>(self, mapper: M) -> impl Parser<I, O, E>
        where M: Fn(O1, O2) -> O
    {
        map2(self, mapper)
    }
}

impl<I, O1, O2, E, F> Map2<I, O1, O2, E> for F
    where F: Parser<I, (O1, O2), E>,
          E: ParseError<I>
{}

pub fn map3<I, O, E, F, M, O1, O2, O3>(parser: F, mapper: M) -> impl Parser<I, O, E>
    where F: Parser<I, (O1, O2, O3), E>,
          M: Fn(O1, O2, O3) -> O,
          E: ParseError<I>
{
    parser.map(move |(a, b, c)| mapper(a, b, c))
}

pub trait Map3<I, O1, O2, O3, E>: Parser<I, (O1, O2, O3), E> + Sized
    where E: ParseError<I>
{
    fn map3<O, M>(self, mapper: M) -> impl Parser<I, O, E>
        where M: Fn(O1, O2, O3) -> O
    {
        map3(self, mapper)
    }
}

impl<I, O1, O2, O3, E, F> Map3<I, O1, O2, O3, E> for F
    where F: Parser<I, (O1, O2, O3), E>,
          E: ParseError<I>
{}

pub fn lines<I, O, E, F>(parser: F) -> impl Parser<I, Vec<O>, E>
    where F: Parser<I, O, E>,
          E: ParseError<I>,
          I: Clone + nom::Compare<&'static str> +
             InputLength + InputTake + InputIter +
             Slice<RangeFrom<usize>> + Slice<Range<usize>> + Slice<RangeTo<usize>>
{
    separated_list0(line_ending, parser)
}

pub fn run<I, O, E, F>(parser: F, input: I) -> Result<O, super::ParseError>
    where F: Parser<I, O, E> + Sized,
          I: InputLength,
          E: ParseError<I>,
          nom::Err<E>: Display
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
        nom::combinator::value((), nom::sequence::tuple(($($parser,)*)))
    };
}