use aoc_lib::{parsing::InvalidTokenError, geometry::{Point2D, grid::{Grid, GridLikeMut, GridLike}, CardinalDirection, Directional, Polygon}};
use aoc_runner_api::SolverResult;
use anyhow::{Result, Context, bail, anyhow};
use itertools::Itertools;

#[derive(Clone, Copy)]
enum Tile {
    Pipe(CardinalDirection, CardinalDirection),
    Ground
}

impl TryFrom<char> for Tile {
    type Error = InvalidTokenError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Pipe(CardinalDirection::North, CardinalDirection::South),
            '-' => Self::Pipe(CardinalDirection::East, CardinalDirection::West),
            'L' => Self::Pipe(CardinalDirection::North, CardinalDirection::East),
            'J' => Self::Pipe(CardinalDirection::North, CardinalDirection::West),
            '7' => Self::Pipe(CardinalDirection::South, CardinalDirection::West),
            'F' => Self::Pipe(CardinalDirection::South, CardinalDirection::East),
            '.' | 'S' => Self::Ground,
            c => return Err(InvalidTokenError(c))
        })
    }
}

impl Tile {
    fn exit_direction(self, facing: CardinalDirection) -> Option<CardinalDirection> {
        let entrance = facing.reverse();
        match self {
            Tile::Pipe(start, end) if entrance == start => Some(end),
            Tile::Pipe(start, end) if entrance == end => Some(start),
            _ => None,
        }
    }
}

fn construct_loop(map: &Grid<Tile>, mut position: Point2D<usize>) -> Result<Polygon<isize>> {
    let mut seen: Polygon<isize> = Polygon::new();
    let mut facing = match map[position] {
        Tile::Ground => bail!("Starting position is not a pipe"),
        Tile::Pipe(entrance, _) => entrance.reverse()
    };

    loop {
        let (prev_facing, prev_pos) = (facing, position);
        let tile = map.get(position).context("Position is not on the grid")?;
        facing = tile.exit_direction(facing).context("Pipe lead to dead end")?;
        position = position.checked_add::<isize>(facing.direction_vector())
            .context("Moved off the grid")?;

        if facing != prev_facing && !seen.push_point(prev_pos.try_map()?) {
            return Ok(seen)
        }
    }
}

fn parse_map(input: &str) -> Result<(Grid<Tile>, Point2D<usize>)> {
    let mut map: Grid<Tile> = Grid::parse(input)?;
    let start = input.lines()
        .enumerate()
        .find_map(|(y, line)| line.find('S').map(|x| Point2D(x, y)))
        .context("Input contains no starting position")?;

    let [entrance, exit]: [CardinalDirection; 2] = CardinalDirection::all()
        .into_iter()
        .filter(|direction| {
            start.checked_add::<isize>(direction.direction_vector())
                .and_then(|location| map.get(location))
                .and_then(|tile| tile.exit_direction(*direction))
                .is_some()
        }).collect_vec()
        .try_into()
        .map_err(|_| anyhow!("Starting position is not connected to exactly 2 pipes"))?;

    let tile = map.get_mut(start)
        .context("Starting position is not inside the grid")?;
    
    *tile = Tile::Pipe(entrance, exit);
    
    Ok((map, start))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (map, start) = parse_map(input)?;
    let distance = construct_loop(&map, start)?.circumference() / 2;

    Ok(Box::new(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (map, start) = parse_map(input)?;
    let polygon = construct_loop(&map, start)?;

    Ok(Box::new(polygon.pick() - polygon.circumference()))
}