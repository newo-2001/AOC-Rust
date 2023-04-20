
use std::{error::Error, fs};

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_while_m_n},
    sequence::{preceded, delimited},
    character::complete::{self},
    IResult,
    combinator::{map_res, value},
    branch::alt,
    multi::many0,
    Parser
};

fn hex_escape_code(input: &str) -> IResult<&str, char> {
    map_res(
        preceded(tag("\\x"), take_while_m_n(2, 2, |x: char| x.is_digit(16))),
        |x| u8::from_str_radix(x, 16).map(char::from)
    )(input)
}

fn escape_character(input: &str) -> IResult<&str, char> {
    alt((
        hex_escape_code,
        value('\\', tag("\\\\")),
        value('\"', tag("\\\"")),
    ))(input)
}

fn character(input: &str) -> IResult<&str, char> {
    alt((escape_character, complete::none_of("\"")))(input)
}

fn deserialize<'a>(input: &'a str) -> Result<String, Box<dyn Error + 'a>> {
    let quoted = |p| delimited(complete::char('\"'), p, complete::char('\"'));
    Ok(quoted(many0(character).map(|x| x.iter().collect::<String>()))(input)?.1)
}

fn deserialized_size_diff<'a>(code: &'a str) -> Result<usize, Box<dyn Error + 'a>> {
    Ok(code.len() - deserialize(code)?.chars().count())
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

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/2015/day_8.txt")?;

    let serialized_overhead: usize = content
        .lines()
        .map(deserialized_size_diff)
        .collect::<Result<Vec<usize>, Box<dyn Error>>>()
        .expect("Failed to read string")
        .iter().sum();

    println!("The deserialized code saved {} characters", serialized_overhead);

    let serialized_overhead: usize = content
        .lines()
        .map(serialized_size_diff)
        .sum();

    println!("The serialized code takes up {} extra characters", serialized_overhead);

    Ok(())
}