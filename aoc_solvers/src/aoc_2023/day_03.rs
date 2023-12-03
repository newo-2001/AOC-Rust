use aoc_lib::geometry::{grid::{Grid, GridLike}, Point2D, Direction2D};
use aoc_runner_api::SolverResult;

#[derive(Clone, Copy)]
enum Tile {
    Symbol(char),
    Digit(u32),
    Period
}

impl Tile {
    fn is_symbol(self) -> bool {
        matches!(self, Self::Symbol(_))
    }
}

impl TryFrom<char> for Tile {
    type Error = !;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Period,
            char => char.to_digit(10)
                .map_or(Tile::Symbol(char), Tile::Digit)
        })
    }
}

#[derive(Default)]
struct Number {
    value: u32,
    tiles: Vec<Point2D<usize>>
}

impl Number {
    fn is_part_number(&self, grid: &Grid<Tile>) -> bool {
        self.tiles.iter()  
            .flat_map(|tile| {
                tile.neighbours::<isize, _>(Direction2D::all())
                    .map(|neighbour| grid.get(neighbour))
            }).flatten()
            .any(|tile| tile.is_symbol())
    }
}

// TODO: Clean this shit up
fn numbers(grid: &Grid<Tile>) -> Vec<Number> {
    let mut numbers = Vec::<Number>::new();
    let mut number: Option<Number> = None;

    for (position, tile) in grid.enumerate() {
        number = if let &Tile::Digit(digit) = tile {
            let mut number = number.unwrap_or_default();
            number.tiles.push(position);
            number.value = number.value * 10 + digit;

            Some(number)
        } else {
            if let Some(number) = number {
                numbers.push(number);
            }

            None
        }
    }

    if let Some(number) = number {
        numbers.push(number);
    }

    numbers
}

fn part_numbers(grid: &Grid<Tile>) -> impl Iterator<Item=u32> + '_ {
    numbers(grid)
        .into_iter()
        .filter(|number| number.is_part_number(grid))
        .map(|number| number.value)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid: Grid<Tile> = Grid::parse(input)?;
    let parts_sum: u32 = part_numbers(&grid).sum();

    Ok(Box::new(parts_sum))
}