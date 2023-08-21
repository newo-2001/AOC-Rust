use std::error::Error;

use aoc_lib::io::read_puzzle_input;
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

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_puzzle_input(2015, 12)?;
    let document = serde_json::from_str(content.as_str())?;

    let sum: i64 = find_numbers(&document, false).iter().sum();
    println!("The sum of all numbers in the JSON document is {}", sum);

    let sum: i64 = find_numbers(&document, true).iter().sum();
    println!("The sum of all the number in the JSON document, ignoring red objects, is {}", sum); 

    Ok(())
}