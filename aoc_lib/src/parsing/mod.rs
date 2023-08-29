mod combinators;
mod parsers;
mod errors;

use nom::{IResult, error::VerboseError};

pub use parsers::*;
pub use combinators::*;
pub use errors::*;

pub type TextParserResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;
