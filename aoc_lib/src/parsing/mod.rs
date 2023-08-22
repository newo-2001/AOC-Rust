mod combinators;
mod parsers;

use nom::{IResult, error::VerboseError};

pub use parsers::*;
pub use combinators::*;

pub type TextParserResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;
