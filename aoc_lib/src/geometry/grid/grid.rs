use std::{fmt::Debug, vec};

use thiserror::Error;

use crate::{parsing::InvalidTokenError, geometry::{Point2D, Area, WrongDimensionsError, Dimensions}};

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
        if !self.area().contains(location) { return None }
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
        if !self.area().contains(location) { return None }
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
pub enum GridParseError {
    #[error(transparent)]
    InvalidToken(#[from] InvalidTokenError<char>),
    #[error(transparent)]
    WrongDimensions(#[from] WrongDimensionsError)
}

impl<T> Grid<T>
{
    pub fn empty(dimensions: Dimensions) -> Grid<T>
        where T: Default + Clone
    {
        let tiles = vec![T::default(); dimensions.surface_area()];

        Grid {
            dimensions,
            tiles: tiles.into_boxed_slice()
        }
    }

    pub fn from_iter<I>(dimensions: Dimensions, rows: I) -> Result<Grid<T>, WrongDimensionsError>
        where I: IntoIterator<Item=T>,
    {
        let tiles = rows.into_iter()
            .collect::<Vec<T>>()
            .into_boxed_slice();

        if tiles.len() == dimensions.surface_area() {
            Ok(Grid { dimensions, tiles: tiles })
        } else {
            Err(WrongDimensionsError { expected: dimensions })
        }
    }

    fn valid_sub_grid(&self, area: Area<usize>) -> Result<(), InvalidGridAreaError> {
        if area.top_left() < Point2D::zero() || area.bottom_right() >= self.dimensions.into() {
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

    pub fn view(&self) -> GridView<T> { self.into() }
    pub fn view_mut(&mut self) -> GridViewMut<T> { self.into() }

    pub(crate) fn backing_index(&self, Point2D(x, y): Point2D<usize>) -> usize {
        y * self.dimensions.width() + x
    }

    pub fn parse(dimensions: Dimensions, input: &str) -> Result<Grid<T>, GridParseError>
        where T: TryFrom<char, Error = InvalidTokenError<char>>
    {
        let cells = input.lines()
            .flat_map(|line| line.chars().map(TryInto::<T>::try_into))
            .collect::<Result<Vec<T>, InvalidTokenError<char>>>()
            .map_err(GridParseError::InvalidToken)?;

        Grid::from_iter(dimensions, cells)
            .map_err(GridParseError::WrongDimensions)
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

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