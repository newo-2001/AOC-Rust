use std::{fmt::{Formatter, self, Debug}, vec, slice, ops::Index};

use thiserror::Error;

use crate::parsing::InvalidTokenError;

use super::{Point2D, Dimensions, WrongDimensionsError, Area};

pub trait GridLike: Sized {
    type GridItem;

    fn area(&self) -> Area<usize>;
    fn get(&self, location: Point2D<usize>) -> Option<&Self::GridItem>;
    fn get_column(&self, column: usize) -> Option<Vec<&Self::GridItem>>;
    fn get_row(&self, row: usize) -> Option<&[Self::GridItem]>;

    fn iter(&self) -> impl Iterator<Item=&Self::GridItem> {
        self.iter_rows().flat_map(IntoIterator::into_iter)
    }

    fn enumerate(&self) -> impl Iterator<Item=(Point2D<usize>, &Self::GridItem)> {
        self.area().iter().zip(self.iter())
    }

    fn iter_rows(&self) -> GridRowIterator<Self> {
        GridRowIterator { grid: self, row: 0 }
    }

    fn iter_columns(&self) -> GridColumnIterator<Self> {
        GridColumnIterator { grid: self, column: 0 }
    }

    fn map<U>(&self, mapper: impl Fn(&Self::GridItem) -> U) -> Grid<U> {
        let items = self.iter().map(mapper);
        Grid::from_iter(self.area().dimensions(), items).unwrap()
    }

    fn enumerate_map<U: Debug>(&self, mapper: impl Fn((Point2D<usize>, &Self::GridItem)) -> U) -> Grid<U> {
        let items = self.enumerate().map(mapper);
        Grid::from_iter(self.area().dimensions(), items).unwrap()
    }
}

fn debug_grid<T: Debug, G: GridLike<GridItem=T>>(grid: &G, f: &mut Formatter<'_>) -> fmt::Result {
    let rows: Vec<Vec<_>> = grid.iter_rows()
        .map(Iterator::collect)
        .collect();
    
    writeln!(f)?;
    for row in rows {
        writeln!(f, "{:?}", row)?;
    }

    Ok(())
}

#[derive(Clone)]
pub struct Grid<T>
{
    pub dimensions: Dimensions,
    tiles: Box<[T]>
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

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        debug_grid(self, f)
    }
}

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

    fn backing_index(&self, Point2D(x, y): Point2D<usize>) -> usize {
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

impl<'a, T> Into<GridView<'a, T>> for &'a Grid<T> {
    fn into(self) -> GridView<'a, T> {
        self.sub_grid(self.area()).unwrap()
    }
}

impl<'a, T> Into<GridViewMut<'a, T>> for &'a mut Grid<T> {
    fn into(self) -> GridViewMut<'a, T> {
        self.sub_grid_mut(self.dimensions.into()).unwrap()
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

#[derive(Clone)]
pub struct GridView<'a, T> {
    grid: &'a Grid<T>,
    pub area: Area<usize>
}

impl<'a, T> GridLike for GridView<'a, T> {
    type GridItem = T;

    fn get(&self, location: Point2D<usize>) -> Option<&T> {
        if !self.area.contains(location) { return None; }

        let location = location + self.area.top_left();
        let index = self.grid.backing_index(location);
        self.grid.tiles.get(index)
    }

    fn get_row(&self, row: usize) -> Option<&[T]> {
        if row > self.area.bottom() { return None };

        let row = row + self.area.top();
        let start = row * self.grid.dimensions.width() + self.area.left();
        let end = start + self.area.dimensions().width();

        Some(&self.grid.tiles[start..end])
    }

    fn get_column(&self, column: usize) -> Option<Vec<&T>> {
        if column > self.area.right() { return None };

        let offset = self.area.left() + column;
        let step_size = self.grid.dimensions.width();

        let column = self.grid.tiles[offset..]
            .iter()
            .step_by(step_size)
            .collect();

        Some(column)
    }

    fn area(&self) -> Area<usize> { self.area }
}

impl<T: Debug> Debug for GridView<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        debug_grid(self, f)
    }
}

impl<'a, T> GridView<'a, T> {
    pub fn into_grid(&self) -> Grid<T> where T: Clone {
        let items = self.iter().cloned();
        Grid::from_iter(self.area.dimensions(), items).unwrap()
    }
}

impl<'a, T> Index<Point2D<usize>> for GridView<'a, T> {
    type Output = T;

    fn index(&self, index: Point2D<usize>) -> &Self::Output {
        self.get(index).unwrap()
    }
}

pub struct GridColumnIterator<'a, G: GridLike> {
    grid: &'a G,
    column: usize,
}

impl<'a, G: GridLike> Iterator for GridColumnIterator<'a, G> {
    type Item = vec::IntoIter<&'a G::GridItem>;

    fn next(&mut self) -> Option<Self::Item> {
        self.column += 1;
        Some(self.grid.get_column(self.column - 1)?.into_iter())
    }
}

pub struct GridRowIterator<'a, G: GridLike> {
    grid: &'a G,
    row: usize,
}

impl<'a, G: GridLike> Iterator for GridRowIterator<'a, G> {
    type Item = slice::Iter<'a, G::GridItem>;

    fn next(&mut self) -> Option<Self::Item> {
        self.row += 1;
        Some(self.grid.get_row(self.row - 1)?.into_iter())
    }
}

pub struct GridViewMut<'a, T> {
    grid: &'a mut Grid<T>,
    area: Area<usize>
}

impl<'a, T> GridViewMut<'a, T> {
    pub fn area(&self) -> Area<usize> { self.area }

    pub fn get_mut(&mut self, location: Point2D<usize>) -> Option<&mut T> {
        if !self.area.contains(location) { return None; }

        let location = location + self.area.top_left();
        let index = self.grid.backing_index(location);
        self.grid.tiles.get_mut(index)
    }

    pub fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row > self.area.bottom() { return None };

        let row = row + self.area.top();
        let start = row * self.grid.dimensions.width() + self.area.left();
        let end = start + self.area.dimensions().width();

        Some(&mut self.grid.tiles[start..end])
    }
    
    pub fn get_column_mut(&mut self, column: usize) -> Option<Vec<&mut T>> {
        if column > self.area.right() { return None };

        let offset = self.area.left() + column;
        let step_size = self.grid.dimensions.width();

        let column = self.grid.tiles[offset..]
            .iter_mut()
            .step_by(step_size)
            .collect();

        Some(column)
    }

    pub fn fill(&mut self, value: T) where T: Clone {
        for row in self.area.top()..=self.area.bottom() {
            let row = self.get_row_mut(row).unwrap();
            row.fill(value.clone());
        }
    }
}