use std::{cmp::minmax, collections::BTreeSet};

use ahash::{HashSet, HashSetExt};
use anyhow::{anyhow, Result};
use itertools::{Either, Itertools};
use yuki::{errors::NoInput, parsing::parse_lines, spatial::{Area, Point, direction}};

use crate::SolverResult;

fn area(a: Point<i64>, b: Point<i64>) -> u64 {
    let Point { x: width, y: height} = a.abs_diff(b) + Point::one();
    width * height
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let tiles: Vec<Point<i64>> = parse_lines(input)?;

    let max_size = tiles
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| area(a, b))
        .max()
        .ok_or(NoInput)?;

    Ok(Box::new(max_size))
}

struct CompressionSpace<T> {
    rows: Vec<T>,
    cols: Vec<T>
}

impl<T> FromIterator<Point<T>> for CompressionSpace<T> where
    T: Ord
{
    fn from_iter<I>(iter: I) -> Self where
        I: IntoIterator<Item=Point<T>>
    {
        let (mut rows, mut cols): (Vec<T>, Vec<T>) = iter
            .into_iter()
            .map(|Point { x, y }| (x, y))
            .unzip();

        rows.sort_unstable();
        cols.sort_unstable();

        rows.dedup();
        cols.dedup();

        Self { rows, cols }
    }
}

impl<T> CompressionSpace<T> where
    T: Ord
{
    fn compress(&self, point: Point<T>) -> Point<usize> {
        let convert = |row: &[T], needle: T| row
            .binary_search(&needle)
            .map(|idx| idx * 2)
            .unwrap_or_else(|idx| idx * 2 + 1);

        Point {
            x: convert(&self.rows, point.x),
            y: convert(&self.cols, point.y)
        }
    }
}

fn rasterize_polygon<I>(points: I) -> Result<HashSet<Point<usize>>> where
    I: IntoIterator<Item=Point<usize>>,
    I::IntoIter: Clone + ExactSizeIterator
{
    points
        .into_iter()
        .circular_tuple_windows()
        .map(|(from, to)| {
            if from.x == to.x {
                let [min_y, max_y] = minmax(from.y, to.y);
                let edge = (min_y..=max_y).map(move |y| Point { x: from.x, y });
                Ok(Either::Left(edge))
            } else if from.y == to.y {
                let [min_x, max_x] = minmax(from.x, to.x);
                let edge = (min_x..=max_x).map(move |x| Point { x, y: from.y });
                Ok(Either::Right(edge))
            } else {
                Err(anyhow!("Edge is not axis aligned"))
            }
        })
        .flatten_ok()
        .collect::<Result<HashSet<Point<usize>>>>()
}

fn fill_outside(polygon: &HashSet<Point<usize>>) -> HashSet<Point<usize>> {
    let area = Area::bounding_area(polygon.iter().copied());
    let perimeter = rect_edges(
        area.position,
        area.position + Point::from(area.dimensions) - Point::one()
    );

    let mut queue: BTreeSet<Point<usize>> = perimeter
        .filter(|point| !polygon.contains(point))
        .collect();

    let mut outside = HashSet::new();
    while let Some(point) = queue.pop_first() {
        outside.insert(point);

        point
            .neighbours::<direction::Cardinal>()
            .filter(|&point|
                !polygon.contains(&point) &&
                !outside.contains(&point) &&
                area.contains(point)
            ) 
            .collect_into(&mut queue);
    }

    outside
}

fn rect_edges(a: Point<usize>, b: Point<usize>) -> impl Iterator<Item=Point<usize>> {
    let [min_x, max_x] = minmax(a.x, b.x);
    let [min_y, max_y] = minmax(a.y, b.y);
    let horizontal = (min_x..=max_x).flat_map(move |x| [ Point { x, y: a.y }, Point { x, y: b.y }]);
    let vertical = (min_y..=max_y).flat_map(move |y| [ Point { x: a.x, y }, Point { x: b.x, y }]);

    horizontal.chain(vertical)
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let corners: Vec<Point<i64>> = parse_lines(input)?;
    let space: CompressionSpace<i64> = corners.iter().copied().collect();

    let compressed_corners = corners
        .iter()
        .map(|&point| space.compress(point));

    let polygon = rasterize_polygon(compressed_corners)?;
    let outside = fill_outside(&polygon);

    let max_contained_rect = corners
        .into_iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            rect_edges(space.compress(a), space.compress(b))
                .all(|point| !outside.contains(&point))
                .then(|| area(a, b))
        })
        .max()
        .ok_or(NoInput)?;

    Ok(Box::new(max_contained_rect))
}