
use aoc_lib::parsing::{quoted, TextParser, TextParserResult, ParseError};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    sequence::preceded,
    character::complete::none_of,
    combinator::{map_res, value},
    branch::alt,
    multi::many0,
    Parser
};

fn hex_escape_code(input: &str) -> TextParserResult<char> {
    map_res(
        preceded(tag("\\x"), take_while_m_n(2, 2, |x: char| x.is_ascii_hexdigit())),
        |x| u8::from_str_radix(x, 16).map(char::from)
    )(input)
}

fn escape_character(input: &str) -> TextParserResult<char> {
    alt((
        hex_escape_code,
        value('\\', tag("\\\\")),
        value('\"', tag("\\\"")),
    ))(input)
}

fn character(input: &str) -> TextParserResult<char> {
    escape_character.or(none_of("\"")).parse(input)
}

fn deserialize(input: &str) -> TextParserResult<String> {
    quoted(many0(character))
        .map(|x| x.iter().collect::<String>())
        .parse(input)
}

fn deserialized_size_diff(code: &str) -> Result<usize, ParseError> {
    let deserialized = deserialize.run(code)?;
    Ok(code.len() - deserialized.chars().count())
}

fn serialize_char(char: char) -> String {
    match char {
        '\"' => String::from("\\\""),
        '\\' => String::from("\\\\"),
        x => x.to_string()
    }
}

fn serialize(input: &str) -> String {
    format!("\"{}\"", input.chars().map(serialize_char).join(""))
}

fn serialized_size_diff(input: &str) -> usize {
    serialize(input).len() - input.len()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let serialized_overhead: usize = input.lines()
        .map(deserialized_size_diff)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum();

    Ok(Box::new(serialized_overhead))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_2(input: &str) -> SolverResult {
    let serialized_overhead: usize = input.lines()
        .map(serialized_size_diff)
        .sum();

    Ok(Box::new(serialized_overhead))
}