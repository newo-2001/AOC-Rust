use std::{fmt::{Debug, Display}, vec};

use itertools::Itertools;
use thiserror::Error;

use crate::geometry::{Point2D, Area, Dimensions, NotRectangularError};

use super::{GridLike, GridView, GridViewMut, grid_like::{impl_grid_traits, impl_grid_traits_mut, GridLikeMut}};

#[derive(Clone)]
pub struct Grid<T>
{
    pub(crate) dimensions: Dimensions,
    pub(crate) tiles: Box<[T]>
}

impl<T> GridLike for Grid<T> {
    type GridItem = T;

    fn get(&self, location: Point2D<usize>) -> Option<&T> {
        if !self.area().contains(&location) { return None }
        self.tiles.get(self.backing_index(location))
    }

    fn get_row(&self, row: usize) -> Option<&[T]> {
        if row >= self.dimensions.height() { return None }
        let start = self.backing_index(Point2D(0, row));
        Some(&self.tiles[start..start + self.dimensions.width()])
    }

    fn get_column(&self, column: usize) -> Option<Vec<&T>> {
        if column >= self.dimensions.width() { return None };

        let column = self.tiles[column..]
            .iter()
            .step_by(self.dimensions.width())
            .collect();

        Some(column)
    }

    fn area(&self) -> Area<usize> {
        self.dimensions.into()
    }
}

impl<T> GridLikeMut for Grid<T> {
    fn get_mut(&mut self, location: Point2D<usize>) -> Option<&mut T> {
        if !self.area().contains(&location) { return None }
        self.tiles.get_mut(self.backing_index(location))
    }

    fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row >= self.dimensions.height() { return None }
        let start = self.backing_index(Point2D(0, row));
        Some(&mut self.tiles[start..start + self.dimensions.width()])
    }

    fn get_column_mut(&mut self, column: usize) -> Option<Vec<&mut T>> {
        if column >= self.dimensions.width() { return None };

        let column = self.tiles[column..]
            .iter_mut()
            .step_by(self.dimensions.width())
            .collect();

        Some(column)
    }
}

impl_grid_traits!(Grid<T>);
impl_grid_traits_mut!(Grid<T>);

#[derive(Debug, Error)]
pub enum GridParseError<E> {
    #[error(transparent)]
    InvalidToken(E),
    #[error(transparent)]
    NotRectangular(#[from] NotRectangularError)
}

impl<T> Grid<T>
{
    #[must_use]
    pub fn empty(dimensions: Dimensions) -> Self
        where T: Default + Clone
    {
        let tiles = vec![T::default(); dimensions.surface_area()];

        Self {
            dimensions,
            tiles: tiles.into_boxed_slice()
        }
    }

    pub fn new(tiles: Vec<Vec<T>>) -> Result<Self, NotRectangularError>
    {
        let dimensions: Dimensions = (&tiles).try_into()?;
        let tiles = tiles
            .into_iter()
            .flat_map(Vec::into_iter)
            .collect_vec()
            .into_boxed_slice();

        Ok(Self { dimensions, tiles })
    }

    fn valid_sub_grid(&self, area: Area<usize>) -> Result<(), InvalidGridAreaError> {
        if area.bottom() >= self.dimensions.1 || area.right() >= self.dimensions.0 {
            Err(InvalidGridAreaError { dimensions: self.dimensions, area })
        } else { Ok(()) }
    }

    pub fn sub_grid(&self, area: Area<usize>) -> Result<GridView<T>, InvalidGridAreaError> {
        self.valid_sub_grid(area)?;
        Ok(GridView { grid: self, area })
    }

    pub fn sub_grid_mut(&mut self, area: Area<usize>) -> Result<GridViewMut<T>, InvalidGridAreaError> {
        self.valid_sub_grid(area)?;
        Ok(GridViewMut { grid: self, area })
    }

    #[must_use]
    pub fn view(&self) -> GridView<T> { self.into() }
    pub fn view_mut(&mut self) -> GridViewMut<T> { self.into() }

    pub(crate) const fn backing_index(&self, Point2D(x, y): Point2D<usize>) -> usize {
        y * self.dimensions.width() + x
    }

    pub fn parse<E: Display>(input: &str) -> Result<Self, GridParseError<E>>
        where T: TryFrom<char, Error = E>
    {
        let cells: Vec<Vec<T>> = input.lines()
            .map(|line| line
                .chars()
                .map(TryInto::<T>::try_into)
                .try_collect()
            )
            .try_collect()
            .map_err(GridParseError::InvalidToken)?;

        Self::new(cells)
            .map_err(GridParseError::NotRectangular)
    }

    #[must_use]
    pub fn into_rows(self) -> IntoRowIterator<T> {
        IntoRowIterator {
            tiles: self.tiles.into_vec(),
            width: self.dimensions.width()
        }
    }

    pub fn map<U>(self, mapper: impl Fn(T) -> U) -> Grid<U> {
        let dimensions = self.dimensions;
        let tiles = self.into_iter()
            .map(mapper)
            .collect_vec()
            .into_boxed_slice();

        Grid { dimensions, tiles }
    }

    pub fn enumerate_map<U>(self, mapper: impl Fn(Point2D<usize>, T) -> U) -> Grid<U> {
        let dimensions = self.dimensions;
        let tiles = self.into_iter()
            .enumerate()
            .map(|(index, value)| mapper(Point2D(index % dimensions.0, index / dimensions.1), value))
            .collect_vec()
            .into_boxed_slice();

        Grid { dimensions, tiles }
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.into_vec().into_iter()
    }
}

#[derive(Debug, Error)]
#[error("{dimensions} is not a valid area within a grid with dimensions {area}")]
pub struct InvalidGridAreaError {
    pub dimensions: Dimensions,
    pub area: Area<usize>
}

pub struct IntoRowIterator<T> {
    tiles: Vec<T>,
    width: usize
}

impl<T> Iterator for IntoRowIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let row = self.tiles.drain(0..self.width).collect_vec();
        if row.len() < self.width { None }
        else { Some(row) }
    }
}