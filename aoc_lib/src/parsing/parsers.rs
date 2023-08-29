use nom::{
    branch::alt,
    combinator::eof,
    character::complete::{crlf, newline},
};

use super::{TextParserResult, ignore};

pub fn optional_newline(input: &str) -> TextParserResult<()> {
    alt((
        ignore(crlf),
        ignore(newline),
        ignore(eof)
    ))(input)
}