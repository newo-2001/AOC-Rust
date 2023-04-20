use std::fs;

fn parse_floor(char: char) -> Result<i32, String> {
    match char {
        '(' => Ok(1),
        ')' => Ok(-1),
        c => Err(format!("Failed to parse floor: {}", c))
    }
}

fn find_basement<'a>(directions: impl Iterator<Item=&'a i32>) -> Option<usize> {
    let mut floor: i32 = 0;
    for (i, direction) in directions.enumerate() {
        floor += direction;
        if floor == -1 {
            return Some(i);
        }
    }
    return None;
}

fn main() {
    let directions: Vec<i32> = fs::read_to_string("inputs/2015/day_1.txt")
        .expect("Failed to find input file!")
        .chars()
        .map(parse_floor)
        .collect::<Result<Vec<i32>, String>>()
        .unwrap_or_else(|err| panic!("{}", err));

    let destination: i32 = directions.iter().sum();
    println!("Santa has to deliver the presents to floor {}", destination);

    match find_basement(directions.iter()) {
        None => println!("Santa never enters the basement"),
        Some(index) => println!("Santa enter the basement at position {}", index + 1)
    }
}