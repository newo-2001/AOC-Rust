mod combinators;
mod parsers;
mod errors;

use nom::{IResult, error::VerboseError, Parser, character::complete::{u8, u16, u32, u64, u128, i8, i16, i32, i64, i128}};

pub use parsers::*;
pub use combinators::*;
pub use errors::*;

pub type TextParserResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

pub trait Parsable<'a>: Sized {
    fn parse(input: &'a str) -> TextParserResult<'a, Self>;
}

pub trait TextParser<'a, O>: Parser<&'a str, O, VerboseError<&'a str>> {
    fn run(self, input: &'a str) -> Result<O, ParseError>
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

macro_rules! impl_parsable {
    ($type: ty, $parser: expr) => {
        impl Parsable<'_> for $type {
            fn parse(input: &str) -> TextParserResult<Self> {
                $parser.parse(input)
            }
        }
    };
}

impl_parsable!(u8, u8);
impl_parsable!(u16, u16);
impl_parsable!(u32, u32);
impl_parsable!(u64, u64);
impl_parsable!(u128, u128);
impl_parsable!(usize, usize);
impl_parsable!(i8, i8);
impl_parsable!(i16, i16);
impl_parsable!(i32, i32);
impl_parsable!(i64, i64);
impl_parsable!(i128, i128);
impl_parsable!(isize, isize);
impl_parsable!(f32, f32);
impl_parsable!(f64, f64);