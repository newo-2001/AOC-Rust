use std::{collections::VecDeque, iter::once, fmt::{Display, Formatter, self}};

use aoc_lib::{geometry::{Point2D, CardinalDirection, Area, Dimensions, Directional}, iteration::queue::{Queue, FindState, FoldState}};
use yuki::errors::NoSolution;
use crate::SolverResult;
use hex::ToHex;

#[derive(Clone, PartialEq, Eq)]
struct Path(Vec<CardinalDirection>);

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let path: String = self.0.iter()
            .copied()
            .map(CardinalDirection::relative_char)
            .collect();

        write!(f, "{path}")
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len().cmp(&other.len())
    }
}

impl Path {
    fn with_movement(&self, direction: CardinalDirection) -> Self {
        let mut path = self.clone();
        path.0.push(direction);
        path
    }

    fn len(&self) -> usize { self.0.len() }
    const fn new() -> Self { Self(Vec::new()) }
}

struct State {
    path: Path,
    position: Point2D<usize>
}

impl Default for State {
    fn default() -> Self {
        Self { path: Path::new(), position: Point2D::zero() }
    }
}

impl State {
    fn valid_moves<'a>(&'a self, grid: &'a Grid) -> Vec<Self> {
        use CardinalDirection as Dir;

        let key = format!("{}{}", grid.passcode, self.path);
        let hash: Vec<char> = md5::compute(key).encode_hex();
        
        hash.into_iter()
            .take(4)
            .zip([Dir::North, Dir::South, Dir::West, Dir::East])
            .filter_map(|(c, dir)| {
                let direction = "bcdef".contains(c).then_some(dir)?;
                let position = self.position.checked_add::<isize>(direction.direction_vector())?;
                grid.area.contains(&position).then(|| Self {
                    path: self.path.with_movement(direction),
                    position
                })
            }).collect()
    }
}

struct Grid<'a> {
    passcode: &'a str,
    area: Area<usize>
}

impl Grid<'_> {
    fn new(passcode: &str) -> Grid {
        Grid { passcode, area: Dimensions(4, 4).into() }
    }

    fn shortest_path_to_vault(&self) -> Result<Path, NoSolution> {
        let queue: VecDeque<State> = once(State::default()).collect();
        queue.recursive_find(|state| {
            if state.position == self.area.bottom_right() { FindState::Result(state.path) }
            else { FindState::Branch(state.valid_moves(self)) }
        })
        .ok_or(NoSolution)
    }

    fn longest_path_to_vault(&self) -> Result<Path, NoSolution> {
        let queue: VecDeque<State> = once(State::default()).collect();
        queue.recursive_fold(None, |longest: Option<Path>, state| {
            let mut moves = state.valid_moves(self);
            let vault = moves.extract_if(|state| state.position == self.area.bottom_right()).next();
            let longest = vault.map(|vault| vault.path).max(longest);
            FoldState::Branch(longest, moves)
        })
        .ok_or(NoSolution)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = Grid::new(input);
    Ok(Box::new(grid.shortest_path_to_vault()?))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = Grid::new(input);
    Ok(Box::new(grid.longest_path_to_vault()?.len()))
}