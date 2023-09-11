use std::iter::once;

use aoc_lib::parsing::{TextParserResult, self};
use aoc_runner_api::SolverResult;
use nom::{sequence::{delimited, preceded}, character::complete::{self, anychar}, Parser, multi::{many0, many_till}};

struct Marker<'a> {
    times: usize,
    data: &'a str
}

struct CompressedChunk<'a> {
    children: Vec<Chunk<'a>>,
    marker: Marker<'a>,
}

enum Chunk<'a> {
    Compressed(CompressedChunk<'a>),
    Uncompressed(String)
}

fn parse_marker(input: &str) -> TextParserResult<Marker> {
    let parse_marker = parsing::usize.and(preceded(complete::char('x'), parsing::usize));

    let (input, (length, times)) = delimited(complete::char('('), parse_marker, complete::char(')')).parse(input)?;
    let (data, remaining) = input.split_at(length);

    Ok((remaining, Marker { times, data }))
}

fn decompress(input: &str) -> Result<Vec<Chunk>, String> {
    let (remaining, chunks) = many0(many_till(anychar, parse_marker))
        .parse(input)
        .map_err(|err| err.to_string())?;
    
    chunks.into_iter()
        .flat_map(|(uncompressed, marker)| {
            [
                Ok(Chunk::Uncompressed(uncompressed.into_iter().collect())),
                decompress(marker.data).map(|children| {
                    Chunk::Compressed(CompressedChunk { children, marker })
                })
            ]
        }).chain(once(Ok(Chunk::Uncompressed(remaining.to_owned()))))
        .collect()
}

fn decompressed_length(chunks: &[Chunk], compressed_length: impl Fn(&CompressedChunk) -> usize) -> usize {
    chunks.iter()
        .map(|chunk| match chunk {
            Chunk::Uncompressed(chunk) => chunk.len(),
            Chunk::Compressed(chunk) => compressed_length(chunk) * chunk.marker.times
        }).sum()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let compressed_length = |chunk: &CompressedChunk| chunk.marker.data.len();
    let compressed = decompress(input)?;
    let length = decompressed_length(&compressed, compressed_length);

    Ok(Box::new(length))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    fn compressed_length(chunk: &CompressedChunk) -> usize {
        decompressed_length(&chunk.children, compressed_length)
    }

    let compressed = decompress(input)?;
    let length = decompressed_length(&compressed, compressed_length);

    Ok(Box::new(length))
}