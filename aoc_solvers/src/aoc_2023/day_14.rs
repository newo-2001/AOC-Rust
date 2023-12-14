use ahash::HashMap;
use aoc_lib::{geometry::{Point2D, CardinalDirection, Directional}, iteration::ExtraIter};
use aoc_runner_api::SolverResult;
use anyhow::{anyhow, Result};
use indexmap::IndexSet;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Rock {
    Round,
    Square
}

fn parse_rock(char: char) -> Result<Option<Rock>> {
    Ok(match char {
        '.' => None,
        'O' => Some(Rock::Round),
        '#' => Some(Rock::Square),
        _ => return Err(anyhow!("Invalid rock encountered: '{char}'"))
    })
}

struct Grid {
    tiles: HashMap<Point2D<usize>, Rock>,
    width: usize,
    height: usize
}

impl Grid {
    fn parse(input: &str) -> Result<Self> {
        let lines = input.lines().collect_vec();
        let height = lines.len();
        let width = lines.first().map_or(0, |line| line.chars().count());
        let rocks = lines.into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .map(parse_rock)
                    .enumerate()
                    .filter_map(move |(x, result)| match result {
                        Ok(rock) => rock.map(|rock| Ok((Point2D(x, y), rock))),
                        Err(err) => Some(Err(err))
                    })
            }).collect::<Result<HashMap<Point2D<usize>, Rock>>>()?;
        
        Ok(Self { width, height, tiles: rocks, })
    }

    fn get(&self, location: Point2D<usize>) -> Option<&Rock> {
        if location.x() >= self.width || location.y() >= self.height { 
            Some(&Rock::Square) 
        } else { self.tiles.get(&location) }
    }

    fn load(&self) -> usize {
        self.tiles.iter()
            .sum_by(|(Point2D(_, y), rock)| match rock {
                Rock::Square => 0,
                Rock::Round => self.height - y
            })
    }

    fn tilt(&mut self, direction: CardinalDirection) {
        loop {
            let mut moved = false;

            for (location, rock) in self.tiles.clone() {
                if rock != Rock::Round { continue; }

                let neighbour = location.checked_add::<isize>(direction.direction_vector());
                if let Some(neighbour) = neighbour {
                    if self.get(neighbour).is_some() { continue; }

                    self.tiles.insert(neighbour, Rock::Round);
                    self.tiles.remove(&location);
                    moved = true;
                }
            }

            if !moved { break; }
        }
    }

    fn rotate(&mut self) {
        let directions = [
            CardinalDirection::North,
            CardinalDirection::West,
            CardinalDirection::South,
            CardinalDirection::East
        ];

        for direction in directions {
            self.tilt(direction);
        }
    }

    fn state(&self) -> Vec<Point2D<usize>> {
        let mut rocks = self.tiles.iter()
            .filter_map(|(&location, &rock)| {
                (rock == Rock::Round).then_some(location)
            }).collect_vec();

        rocks.sort_unstable_by(|Point2D(x, y), Point2D(x2, y2)| {
            y.cmp(y2).then_with(|| x.cmp(x2))
        });

        rocks
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut grid = Grid::parse(input)?;
    grid.tilt(CardinalDirection::North);

    Ok(Box::new(grid.load()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut grid = Grid::parse(input)?;
    let mut seen = IndexSet::<Vec<Point2D<usize>>>::new();

    let start_index = loop {
        let state = grid.state();
        if let Some(index) = seen.get_index_of(&state) { break index; }

        seen.insert(state);
        grid.rotate();
    };

    let loop_size = seen.len() - start_index;
    let remaining_cycles = (1_000_000_000 - start_index) % loop_size;
    let last_index = start_index + remaining_cycles;
    let final_load = seen[last_index]
        .iter()
        .sum_by(|Point2D(_, y)| grid.height - y);

    Ok(Box::new(final_load))
}