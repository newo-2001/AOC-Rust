use aoc_runner_api::SolverResult;
use itertools::Itertools;
use serde_json::{Value, Map};

fn contains_red(object: &Map<String, Value>) -> bool {
    object.values().contains(&Value::String(String::from("red")))
}

fn find_numbers(document: &Value, ignore_red: bool) -> Vec<i64> {
    use Value::*;
    
    match document {
        Null | String(_) | Bool(_) => vec![],
        Number(n) => vec![n.as_i64().expect("Found unexpected floating point number")],
        Array(arr) => arr.iter().flat_map(|x| find_numbers(x, ignore_red)).collect(),
        Object(obj) => {
            if ignore_red && contains_red(obj) { vec![] }
            else { obj.values().flat_map(|x| find_numbers(x, ignore_red)).collect() }
        }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let document = serde_json::from_str(input)?;
    let sum: i64 = find_numbers(&document, false).iter().sum();

    Ok(Box::new(sum))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let document = serde_json::from_str(input)?;
    let sum: i64 = find_numbers(&document, true).iter().sum();

    Ok(Box::new(sum))
}