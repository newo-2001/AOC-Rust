mod combinators;
mod parsers;
mod errors;

use nom::{IResult, error::VerboseError, Parser, multi::separated_list0, character::complete::line_ending};

pub use parsers::*;
pub use combinators::*;
pub use errors::*;

pub type TextParserResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

pub trait Parsable: Sized {
    fn parse(input: &str) -> TextParserResult<Self>;
}

pub trait TextParser<'a, O>: Parser<&'a str, O, VerboseError<&'a str>> {
    fn run(self, input: &'a str) -> Result<O, ParseError<'a>>
        where Self: Sized
    {
        run(self, input)
    }
}

impl<'a, O, F> TextParser<'a, O> for F
    where F: Parser<&'a str, O, VerboseError<&'a str>>
{}

pub fn parse_lines<'a, F, T, E>(parser: F, input: &'a str) -> Result<Vec<T>, E>
    where F: Fn(&'a str) -> Result<T, E>
{
    input.lines()
        .map(parser)
        .collect()
}