use ahash::HashMap;
use aoc_lib::{parsing::{parse_lines, TextParser, ParseError}, errors::NoSolution};
use crate::SolverResult;

use std::{collections::VecDeque, iter::{self, once}};

use nom::{
    sequence::{tuple, preceded, terminated},
    bytes::complete::tag,
    character::complete::{alpha1, char, i32},
    Parser, combinator::opt
};

type Cookie<'a> = HashMap<&'a str, u32>;
type Ingredients<'a> = HashMap<&'a str, Ingredient<'a>>;

#[derive(Clone)]
struct Ingredient<'a> {
    name: &'a str,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

impl Ingredient<'_> {
    fn parse(input: &str) -> Result<Ingredient, ParseError> {
        let property = |name| preceded(
            tuple((char(' '), tag(name), char(' '))),
            terminated(i32, opt(char(','))));
        let name = terminated(alpha1, char(':'));

        tuple((
            name,
            property("capacity"),
            property("durability"),
            property("flavor"),
            property("texture"),
            property("calories")
        )).map(|(name, capacity, durability, flavor, texture, calories)|
            Ingredient { name, capacity, durability, flavor, texture, calories }
        ).run(input)
    }   
}

fn cookie_property(cookie: &Cookie, ingredients: &Ingredients, property: impl Fn(&Ingredient) -> i32) -> u64 {
    cookie.iter().map(|(name, &amount)| {
        let ingredient = ingredients.get(name)
            .unwrap_or_else(|| panic!("Cookie had unrecognized ingredient: {name}"));

        i64::from(property(ingredient)) * i64::from(amount)
    }).sum::<i64>()
        .try_into()
        .unwrap_or(0)
}

fn cookie_score(cookie: &Cookie, ingredients: &Ingredients) -> u64 {
    [
        |ingredient: &Ingredient| ingredient.capacity,
        |ingredient: &Ingredient| ingredient.durability,
        |ingredient: &Ingredient| ingredient.flavor,
        |ingredient: &Ingredient| ingredient.texture
    ].iter().map(|selector| cookie_property(cookie, ingredients, selector))
        .reduce(|acc, x| acc * x)
        .expect("Cookie had no ingredients")
}

fn all_cookies<'a>(teaspoons: u32, ingredients: &'a Ingredients) -> impl Iterator<Item = Cookie<'a>> {
    #[derive(Clone)]
    struct State {
        length: usize,
        total: u32,
        sum: Vec<u32>
    }
    
    let mut results: Vec<Box<dyn Iterator<Item = u32>>> = vec![];
    let initial_state = State { length: ingredients.len(), total: teaspoons, sum: vec![] };
    let mut queue: VecDeque<State> = once(initial_state).collect();

    while let Some(State { length, total, sum }) = queue.pop_front() {
        match length {
            1 => results.push(Box::from(sum.into_iter().chain(iter::once(total)))),
            _ => {
                for i in 1..total {
                    let mut new_sum = sum.clone();
                    new_sum.push(i);
                    queue.push_back(State { length: length - 1, total: total - i, sum: new_sum });
                }
            }
        }
    } 

    results.into_iter()
        .map(|spoons| ingredients.clone().into_keys().zip(spoons))
        .map(HashMap::from_iter)
}

fn best_score<'a>(cookies: impl Iterator<Item=&'a Cookie<'a>>, ingredients: &Ingredients) -> Result<u64, NoSolution> {
    cookies.map(|cookie| cookie_score(cookie, ingredients))
        .max().ok_or(NoSolution)
}

fn parse_ingredients(input: &str) -> Result<Ingredients, ParseError> {
    let ingredients = parse_lines(Ingredient::parse, input)?;

    Ok(ingredients.into_iter()
        .map(|ingredient| (ingredient.name, ingredient))
        .collect())
}

const TEASPOONS: u32 = 100;

pub fn solve_part_1(input: &str) -> SolverResult {
    let ingredients = parse_ingredients(input)?;
    let cookies: Vec<Cookie> = all_cookies(TEASPOONS, &ingredients).collect();
    let best = best_score(cookies.iter(), &ingredients)?;

    Ok(Box::new(best))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let ingredients = parse_ingredients(input)?;
    let cookies: Vec<Cookie> = all_cookies(TEASPOONS, &ingredients).collect();
    
    let exact_calories = cookies.iter()
        .filter(|cookie| cookie_property(cookie, &ingredients, |cookie| cookie.calories) == 500);
    
    let best = best_score(exact_calories, &ingredients)?;

    Ok(Box::new(best))
}