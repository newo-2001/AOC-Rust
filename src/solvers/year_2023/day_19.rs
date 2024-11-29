use std::collections::HashMap;
use anyhow::{anyhow, bail, Context, Result};
use aoc_lib::{parsing::{TextParserResult, curly_brackets, ParseError, lines, TextParser, Map2}, math::Range, range};
use crate::SolverResult;
use nom::{combinator::map_res, character::complete::{anychar, char, u16, alpha1, line_ending}, multi::{separated_list1, count}, sequence::{preceded, pair, separated_pair, terminated}, Parser};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Category {
    ExtremelyCool,
    Musical,
    Aerodynamic,
    Shiny
}

impl Category {
    fn parse(input: &str) -> TextParserResult<Self> {
        map_res(anychar, |char| Ok(match char {
            'x' => Self::ExtremelyCool,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => bail!("Invalid category: {char}")
        })).parse(input)
    }
}

#[derive(Debug)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16
}

impl Part {
    fn parse(input: &str) -> TextParserResult<Self> {
        map_res(curly_brackets(separated_list1(
            char(','),
            preceded(pair(anychar, char('=')), u16)
        )), |values| {
            let [x, m, a, s] = values.try_into()?;
            Result::<_, <Vec<u16> as TryInto::<[u16; 4]>>::Error>::Ok(Self { x, m, a, s })
        }).parse(input)
    }

    const fn get(&self, category: Category) -> u16 {
        match category {
            Category::Aerodynamic => self.a,
            Category::Musical => self.m,
            Category::ExtremelyCool => self.x,
            Category::Shiny => self.s
        }
    }

    fn total_rating(&self) -> u32 {
        [self.x, self.m, self.a, self.s].into_iter()
            .map(Into::<u32>::into)
            .sum()
    }
}

#[derive(Debug)]
enum Condition {
    Greater(Category, u16),
    Smaller(Category, u16),
    Always
}

impl Condition {
    fn parse(input: &str) -> TextParserResult<Self> {
        Parser::or(
            separated_pair(Category::parse, char('>'), u16).map2(Self::Greater),
            separated_pair(Category::parse, char('<'), u16).map2(Self::Smaller),
        ).parse(input)
    }
}

#[derive(Debug)]
enum Action<'a> {
    RunWorkflow(&'a str),
    Accept,
    Reject
}

impl<'a> Action<'a> {
    fn parse(input: &'a str) -> TextParserResult<'a, Self> {
        alpha1.map(|action| match action {
            "A" => Self::Accept,
            "R" => Self::Reject,
            name => Self::RunWorkflow(name)
        }).parse(input)
    }
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Condition,
    action: Action<'a>
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> TextParserResult<'a, Self> {
        Parser::or(
            pair(
                terminated(Condition::parse, char(':')),
                Action::parse
            ), Action::parse.map(|action| (Condition::Always, action))
        ).map(|(condition, action)| Self {
            condition, action
        }).parse(input)
    }

    const fn apply(&self, part: &Part) -> bool {
        match self.condition {
            Condition::Greater(category, value) => part.get(category) > value,
            Condition::Smaller(category, value) => part.get(category) < value,
            Condition::Always => true
        }
    }

    fn match_area(&self, area: Area) -> (Area, Option<Area>) {
        match self.condition {
            Condition::Always => (area, None),
            Condition::Greater(dimension, partition_point) => {
                let (remaining, matched) = area.partition(dimension, partition_point + 1);
                (matched, Some(remaining))
            },
            Condition::Smaller(dimension, partition_point) => {
                let (matched, remaining) = area.partition(dimension, partition_point);
                (matched, Some(remaining))
            }
        }
    }
}

#[derive(Debug)]
struct Workflow<'a>(Vec<Rule<'a>>);

impl<'a> Workflow<'a> {
    fn parse(input: &'a str) -> TextParserResult<'a, (&'a str, Self)> {
        pair(
            alpha1,
            curly_brackets(
                separated_list1(char(','), Rule::parse)
            ).map(Self)
        ).parse(input)
    }

    fn run(&self, part: &Part, workflows: &HashMap<&str, Workflow>) -> Result<bool> {
        let rule = self.0.iter()
            .find(|rule| rule.apply(part))
            .ok_or_else(|| anyhow!("The part: {part:?} was not applicable to any of the rules: {:?}", self.0))?;

        match rule.action {
            Action::Accept => Ok(true),
            Action::Reject => Ok(false),
            Action::RunWorkflow(name) => {
                workflows.get(name)
                    .ok_or_else(|| anyhow!("No workflow with the name: '{name}' exists"))?
                    .run(part, workflows)
            }
        }
    }

    fn accepts(&self, mut area: Area, workflows: &HashMap<&str, Workflow>) -> Result<u64> {
        let mut accepted = 0;

        for rule in &self.0 {
            let (matched, remaining) = rule.match_area(area);
            accepted += match rule.action {
                Action::Accept => matched.area(),
                Action::Reject => 0,
                Action::RunWorkflow(name) => {
                    workflows.get(name)
                        .ok_or_else(|| anyhow!("No workflow with the name: '{name}' exists"))?
                        .accepts(matched, workflows)?
                }
            };

            if let Some(remaining) = remaining {
                area = remaining;
            } else { break; }
        }

        Ok(accepted)
    }
}

fn parse(input: &str) -> Result<(HashMap<&str, Workflow<'_>>, Vec<Part>), ParseError> {
    separated_pair(
        lines(Workflow::parse).map(|workflows| workflows.into_iter().collect()),
        count(line_ending, 2),
        lines(Part::parse)
    ).run(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (workflows, parts) = parse(input)?;

    let start = workflows.get("in").context("No workflow named 'in'")?;
    let total_rating: u32 = parts.into_iter()
        .try_fold(0, |acc, part| match start.run(&part, &workflows) {
            Err(err) => Err(err),
            Ok(true) => Ok(acc + part.total_rating()),
            Ok(false) => Ok(acc)
        })?;

    Ok(Box::new(total_rating))
}

#[derive(Clone, Copy)]
struct Area {
    x: Range<u16>,
    m: Range<u16>,
    a: Range<u16>,
    s: Range<u16>
}

impl Area {
    fn get_dimension_mut(&mut self, category: Category) -> &mut Range<u16> {
        match category {
            Category::ExtremelyCool => &mut self.x,
            Category::Musical => &mut self.m,
            Category::Aerodynamic => &mut self.a,
            Category::Shiny => &mut self.s
        }
    }

    fn area(self) -> u64 {
        let Self { x, m, a, s } = self;
        [x, m, a, s].into_iter()
            .map(|range| u64::from(range.interval()))
            .product()
    }

    fn partition(mut self, dimension: Category, partition_point: u16) -> (Self, Self) {
        let mut left = self;
        let range = left.get_dimension_mut(dimension);
        range.end = partition_point;

        let range = self.get_dimension_mut(dimension);
        range.start = partition_point;

        (left, self)
    }
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (workflows, _) = parse(input)?;

    let start = workflows.get("in").context("No workflow named 'in'")?;
    let accepted = start.accepts(Area {
        x: range!(1..=4000),
        m: range!(1..=4000),
        a: range!(1..=4000),
        s: range!(1..=4000)
    }, &workflows)?;

    Ok(Box::new(accepted))
}