use std::collections::HashSet;

use aoc_lib::{parsing::{Runnable, parse_lines}, iteration::ExtraIter};
use aoc_runner_api::SolverResult;
use itertools::{traits::HomogeneousTuple, Itertools, Either};
use nom::{Parser, sequence::delimited, character::complete::{alpha0, self, alpha1}, multi::many0};

struct Ip<'a>(Vec<&'a str>, Vec<&'a str>);

enum NetworkType {
    Supernet,
    Hypernet
}

impl Ip<'_> {
    fn supports_tls(&self) -> bool {
        let is_abba = |(a, b, c, d): (char, char, char, char)| a == d && b == c && a != b;
        let has_abba = |&seq| !find_subseqs(seq, is_abba).empty();

        let Self(supernets, hypernets) = self;
        supernets.iter().any(has_abba) &&
        !hypernets.iter().any(has_abba)
    }

    fn supports_ssl(&self) -> bool {
        let is_aba = |(a, b, c)| a == c && a != b;
        let find_abas = |&seq| find_subseqs(seq, is_aba);
        let to_bab = |(a, b, _)| (b, a, b);

        let Self(supernets, hypernets) = self;
        let babs: HashSet<(char, char, char)> = supernets.iter()
            .flat_map(find_abas)
            .map(to_bab)
            .collect();

        let is_bab = |bab| babs.contains(&bab);
        let has_bab = |&seq| !find_subseqs(seq, is_bab).empty();
        hypernets.into_iter().any(has_bab)
    }
}

fn find_subseqs<'a, T>(str: &'a str, subseq: impl Fn(T) -> bool + 'a) -> impl Iterator<Item=T> + 'a
    where T: HomogeneousTuple<Item = char> + Copy + 'a
{
    str.chars()
        .into_iter()
        .tuple_windows()
        .filter(move |&seq| subseq(seq))
}

fn parse_ip(input: &str) -> Result<Ip, String> {
    let supernet = alpha1.map(|seq| (NetworkType::Supernet, seq));
    let hypernet = delimited(complete::char('['), alpha0, complete::char(']'))
        .map(|seq| (NetworkType::Hypernet, seq));

    let sequences = many0(hypernet.or(supernet)).run(input)?;
    let (supernet, hypernet) = sequences.into_iter()
        .partition_map(|(network, seq)| match network {
            NetworkType::Supernet => Either::Left(seq),
            NetworkType::Hypernet => Either::Right(seq)
        });

    Ok(Ip(supernet, hypernet))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let valid_ips: usize = parse_lines(parse_ip, input)?
        .into_iter()
        .filter(Ip::supports_tls)
        .count();

    Ok(Box::new(valid_ips))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let valid_ips: usize = parse_lines(parse_ip, input)?
        .into_iter()
        .filter(Ip::supports_ssl)
        .count();

    Ok(Box::new(valid_ips))
}