use std::{fs, error::Error, collections::{HashSet, BTreeSet}};

use itertools::Itertools;
use nom::{character::complete::alpha1, Parser, sequence::preceded, bytes::complete::tag, error::VerboseError};

struct Replacement<'a> {
    from: &'a str,
    to: &'a str
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
struct Mutation {
    chemical: String,
    distance: usize,
}

impl Ord for Mutation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.distance).cmp(&(other.distance)).reverse()
    }
}

fn parse_replacement<'a>(input: &'a str) -> Result<Replacement, String> {
    let mut replacement = alpha1.and(preceded(tag(" => "), alpha1))
        .map(|(from, to): (&str, &str)| Replacement { from, to });
    
    Ok(replacement.parse(input).map_err(|err: nom::Err<VerboseError<&str>>| err.to_string())?.1)
}

fn mutations(chemical: &str, from: &str, to: &str) -> Vec<String> {
    chemical.match_indices(from).map(|(index, _)| {
        let (start, end) = chemical.split_at(index);

        let end = end.to_owned().replacen(from, to, 1);
        let mut start = start.to_owned();
        start.push_str(&end);
        
        start
    }).collect_vec()
}

fn forward_mutations(chemical: &str, replacement: &Replacement) -> Vec<String> {
    mutations(chemical, &replacement.from, &replacement.to)
}

fn backwards_mutations(chemical: &str, replacement: &Replacement) -> Vec<String> {
    mutations(chemical, &replacement.to, &replacement.from)
}

fn fastest_synthesis(target: &str, replacements: &Vec<Replacement>) -> Result<usize, String> {
    let mut queue: BTreeSet<Mutation> = BTreeSet::new();
    let mut cache: HashSet<String> = HashSet::new();
    
    _ = queue.insert(Mutation {
        chemical: String::from(target),
        distance: 0
    });

    while let Some(current_mutation) = queue.iter().next() {
        let Mutation { chemical, distance, .. } = current_mutation.clone();

        if chemical == "e" { return Ok(distance); }
       
        let mutations = replacements.iter()
            .flat_map(|replacement| backwards_mutations(&chemical, replacement))
            .filter(|x| !cache.contains(x))
            .collect_vec();

        for mutation in mutations.into_iter() {
            cache.insert(mutation.clone());

            _ = queue.insert(Mutation {
                chemical: mutation,
                distance: distance + 1
            });
        }
    }

    Err(String::from("Couldn't synthesize medicine"))
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/2015/day_19.txt")?;

    let (replacements, target) = content.split_once("\r\n\r\n")
        .expect("No empty line found in file");

    let replacements = replacements.lines().map(parse_replacement)
        .collect::<Result<Vec<Replacement>, String>>()?;

    let unique_mutations = replacements.iter()
        .flat_map(|replacement| forward_mutations(target, replacement))
        .unique().count();

    println!("There are {} unique mutations", unique_mutations);

    let fastest = fastest_synthesis(target, &replacements)?;
    println!("The least amount of steps required to synthesize the medicine is {}", fastest);

    Ok(())
}