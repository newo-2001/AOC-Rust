use std::iter::once;

use ahash::{HashMap, HashSet, HashSetExt};
use yuki::{iterators::Enumerate2D, spatial::{direction::{self, Directions}, Point}};

use crate::SolverResult;

type Tile = (Point<usize>, char);

#[derive(Debug, Clone, Default)]
struct Region {
    tiles: HashSet<Point<usize>>,
    perimeter: usize
}

fn parse_grid(input: &str) -> HashMap<Point<usize>, char> {
    input
        .lines()
        .map(str::chars)
        .enumerate2d()
        .collect()
}

fn get_region(grid: &HashMap<Point<usize>, char>, tile: Tile) -> Region {
    let mut queue: Vec<Tile> = once(tile).collect();
    let mut region = Region::default();

    while let Some((pos, plant)) = queue.pop() {
        if !region.tiles.insert(pos) { continue; }
        region.perimeter += 4;

        pos
            .neighbours::<direction::Cardinal>()
            .filter_map(|pos| match grid.get(&pos) {
                Some(&p) if p == plant => {
                    region.perimeter -= 1;
                    Some((pos, plant))
                },
                _ => None
            })
            .collect_into(&mut queue);
    }

    region
}

fn fencing_cost<F>(grid: &HashMap<Point<usize>, char>, cost_function: F) -> usize where
    F: Fn(&Region) -> usize
{
    let mut visited = HashSet::<Point<usize>>::new();
    let mut cost = 0;

    for (pos, plant) in grid {
        if visited.contains(pos) { continue; }

        let region = get_region(grid, (*pos, *plant));
        cost += cost_function(&region);
        visited.extend(region.tiles);
    }

    cost
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = parse_grid(input);
    let cost = |region: &Region| region.tiles.len() * region.perimeter;
    Ok(Box::new(fencing_cost(&grid, cost)))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = parse_grid(input);
    let cost = |region: &Region| {
        let turns: usize = region.tiles
            .iter()
            .map(|pos| direction::Cardinal::all()
                .filter(|dir| {
                    let front = pos
                        .add_signed(dir.vector())
                        .is_none_or(|pos| !region.tiles.contains(&pos));
                    
                    let side_pos = pos.add_signed(dir.turn(direction::Rotation::Clockwise).vector());
                    let side = side_pos.is_none_or(|pos| !region.tiles.contains(&pos));
                    
                    let diagonal = side_pos
                        .and_then(|pos| pos.add_signed(dir.vector()))
                        .is_some_and(|pos| region.tiles.contains(&pos));

                    front && (side || diagonal)
                })
                .count()
            )
            .sum();

        region.tiles.len() * turns
    };

    Ok(Box::new(fencing_cost(&grid, cost)))
}