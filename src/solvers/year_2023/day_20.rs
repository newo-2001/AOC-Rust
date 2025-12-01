use std::{iter::{once, repeat}, collections::VecDeque};

use ahash::{HashMap, HashMapExt};
use aoc_lib::{iteration::queue::{Queue, FoldState}, parsing::{parse_lines, ParseError, TextParser, TextParserResult}, string_enum};
use yuki::iterators::{ExtraIter, SingleError};
use crate::SolverResult;
use anyhow::{Result, anyhow, Context, bail};
use itertools::Itertools;
use nom::{Parser, sequence::separated_pair, combinator::value, character::complete::alpha1, multi::separated_list1, bytes::complete::tag};
use num::Integer;

#[derive(Clone, PartialEq, Eq, Debug)]
enum ModuleType<'a> {
    FlipFlop,
    Broadcast,
    Conjunction {
        incoming: HashMap<&'a str, bool>
    }
}

impl ModuleType<'_> {
    fn parse(input: &str) -> TextParserResult<'_, Self> {
        string_enum!(
            "broadcaster" => Self::Broadcast,
            "%" => Self::FlipFlop,
            "&" => Self::Conjunction { incoming: HashMap::new() }
        ).parse(input)
    }
}

#[derive(Clone)]
struct Module<'a> {
    state: bool,
    outgoing: Vec<&'a str>,
    r#type: ModuleType<'a>
}

impl<'a> Module<'a> {
    fn parse(input: &'a str) -> Result<(&'a str, Self), ParseError> {
        separated_pair(
            Parser::or(
                value((ModuleType::Broadcast, "broadcaster"), tag("broadcaster")),
                ModuleType::parse.and(alpha1)
            ),
            tag(" -> "),
            separated_list1(tag(", "), alpha1)
        ).map(|((r#type, name), outgoing)| (name, Self {
            r#type, outgoing, state: false
        })).run(input)
    }

    fn pulse(&mut self, pulse: &Pulse) -> Result<Vec<Pulse>> {
        Ok(match &mut self.r#type {
            ModuleType::FlipFlop => {
                if pulse.state { return Ok(Vec::new()) }

                self.state = !self.state;
                self.outgoing.iter().map(|&edge| Pulse {
                    destination: edge.to_owned(),
                    source: pulse.destination.clone(),
                    state: self.state
                }).collect_vec()
            },
            ModuleType::Conjunction { incoming } => {
                let internal_state = incoming.get_mut(pulse.source.as_str())
                    .ok_or_else(|| anyhow!("Conjuction module can't receive pulses from: {}", pulse.source))?;
                
                *internal_state = pulse.state;

                self.outgoing.iter().map(|&edge| Pulse {
                    destination: edge.to_owned(),
                    source: pulse.destination.clone(),
                    state: !incoming.values().all(|&state| state)
                }).collect_vec()
            },
            ModuleType::Broadcast => {
                self.outgoing.iter().map(|&edge| Pulse {
                    destination: edge.to_owned(),
                    source: pulse.destination.clone(),
                    state: pulse.state
                }).collect_vec()
            }
        })
    }
}

// Owned strings because the borrowchecker is a PITA
#[derive(Debug)]
struct Pulse {
    source: String,
    destination: String,
    state: bool
}

struct Network<'a>(HashMap<&'a str, Module<'a>>);

impl<'a> Network<'a> {
    fn parse(input: &'a str) -> Result<Self> {
        let mut modules: HashMap<&str, Module> = parse_lines(Module::parse, input)?
            .into_iter()
            .collect();
        
        // Lazy clone because I can't be arsed to deal with the borrowchecker rn
        let all_modules = modules.clone();
        let mut conjuction_modules: HashMap<&str, &mut HashMap<&str, bool>> = modules.iter_mut()
            .filter_map(|(&name, module)| match &mut module.r#type {
                ModuleType::Conjunction { incoming } => Some((name, incoming)),
                _ => None
            }).collect();
        
        // Make conjunction modules aware of all incoming signals
        for (name, module) in all_modules {
            for edge in module.outgoing {
                if let Some(conjuction_module) = conjuction_modules.remove(edge) {
                    (*conjuction_module).insert(name, false);
                    conjuction_modules.insert(edge, conjuction_module);
                }
            }
        }

        Ok(Self(modules))
    }

    fn find_output(&'a self, needle: &'a str) -> impl Iterator<Item=(&'a str, &'a Module<'a>)> {
        self.0.iter()
            .filter_map(move |(&name, module)| {
                module.outgoing.contains(&needle).then_some((name, module))
            })
    }

    fn find_counters(&'a self, output: &'a str) -> Result<(String, HashMap<String, Option<u64>>)> {
        match self.find_output(output).single() {
            Ok((name, module)) => match &module.r#type {
                ModuleType::Conjunction { incoming } => {
                    let counters: HashMap<String, Option<u64>> = incoming.keys()
                        .map(|&name| (name.to_owned(), None))
                        .collect();

                    Ok((name.to_owned(), counters))
                },
                _ => bail!("Module '{output}' is not a conjugation module")
            },
            Err(SingleError::None) => bail!("Module '{output}' has no inputs"),
            Err(SingleError::More) => bail!("Module '{output}' has more than one input")
        }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut network = Network::parse(input)?;
    let (low, high) = repeat(())
        .take(1000)
        .try_fold((0u64, 0u64), |counts, ()| {
            let queue: VecDeque<Pulse> = once(Pulse {
                source: String::from("button"),
                destination: String::from("broadcaster"),
                state: false
            }).collect();

            queue.try_recursive_fold(counts, |(mut low_count, mut high_count), pulse| {
                // Apparently this is not a hard error
                let branches = network.0.get_mut(pulse.destination.as_str())
                    .map_or_else(|| Ok(Vec::new()), |module| module.pulse(&pulse))?;

                if pulse.state { high_count += 1; }
                else { low_count += 1; }

                anyhow::Ok(FoldState::Branch((low_count, high_count), branches))
            })
        })?;

    Ok(Box::new(low * high))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut network = Network::parse(input)?;
    let (conjugate, mut counters) = network.find_counters("rx")?;

    for iteration in 1u64.. {
        let mut queue: VecDeque<Pulse> = once(Pulse {
            source: String::from("button"),
            destination: String::from("broadcaster"),
            state: false
        }).collect();

        while let Some(pulse) = queue.pop_front() {
            if let Some(module) = network.0.get_mut(pulse.destination.as_str()) {
                queue.extend(module.pulse(&pulse)?);                
            }

            if pulse.destination == conjugate && pulse.state {
                if let Some(loop_size) = counters.get_mut(pulse.source.as_str()) {
                    let _ = loop_size.insert(iteration);
                }
            }
        }

        if let Some(loop_sizes) = counters.values().copied().collect::<Option<Vec<_>>>() {
            let lcm =  loop_sizes.into_iter().reduce(|a, b| a.lcm(&b))
                .context("The conjucation module connected to 'rx' does not have any incoming signals")?;

            return Ok(Box::new(lcm));
        }
    };

    unreachable!();
}