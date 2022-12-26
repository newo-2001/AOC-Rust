use std::{fs, ops, collections::HashSet};

#[derive(Default, Eq, PartialEq, Hash, Copy, Clone)]
struct Coord(i32, i32);

impl ops::Add<&Coord> for &Coord {
    type Output = Coord;
    fn add(self, rhs: &Coord) -> Self::Output {
        return Coord(self.0 + rhs.0, self.1 + rhs.1);
    }
}

enum Direction {
    North, East, South, West
}

impl Direction {
    fn unit_vector(&self) -> Coord {
        use Direction::*;
        return match self {
            North => Coord(0, -1),
            East => Coord(1, 0),
            South => Coord(0, 1),
            West => Coord(-1, 0)
        }
    }

    fn parse(c: &char) -> Result<Direction, String> {
        use Direction::*;
        return match c {
            '^' => Ok(North),
            '>' => Ok(East),
            'v' => Ok(South),
            '<' => Ok(West),
            _ => Err(format!("Failed to parse direction: {}", c))
        };
    }
}

fn unique_houses<'a>(directions: impl Iterator<Item=&'a Direction>) -> HashSet<Coord> {
    let mut position = Coord::default();
    return directions.map(|direction| {
        position = &direction.unit_vector() + &position;
        return position;
    }).chain(std::iter::once(Coord::default()))
        .collect();
}

fn main() {
    let movements: Vec<Direction> = fs::read_to_string("inputs/day_3.txt")
        .expect("Failed to read input file!")
        .chars()
        .map(|c| Direction::parse(&c))
        .collect::<Result<Vec<Direction>, String>>()
        .unwrap_or_else(|err| panic!("{}", err));

    let visited_houses: usize = unique_houses(movements.iter()).len();
    println!("Santa visits {} unique houses", visited_houses);

    let santa_houses = unique_houses(movements.iter().step_by(2));
    let robo_santa_houses = unique_houses(movements.iter().skip(1).step_by(2));
    let all_houses = santa_houses.union(&robo_santa_houses);
    
    println!("Santa and Robo-Santa together visit {} unique houses", all_houses.count());
}