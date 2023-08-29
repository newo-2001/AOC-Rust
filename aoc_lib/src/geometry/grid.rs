use std::{fmt::{Display, Formatter, self, Debug}, error::Error, vec, slice, iter::FlatMap, ops::Index};

use nom::{multi::many0, combinator::value, character::complete, Parser};

use crate::parsing::Runnable;

use super::{Point2D, Dimensions, WrongDimensionsError, Area};

#[derive(Clone)]
pub struct Grid<T>
{
    pub dimensions: Dimensions,
    tiles: Box<[T]>
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

    pub fn area(&self) -> Area<usize> {
        self.dimensions.into()
    }

    fn backing_index(&self, Point2D(x, y): Point2D<usize>) -> usize {
        y * self.dimensions.width() + x
    }
}

impl<'a, T> Into<GridView<'a, T>> for &'a Grid<T> {
    fn into(self) -> GridView<'a, T> {
        self.sub_grid(self.area()).unwrap()
    }
}

impl<'a, T> Into<GridViewMut<'a, T>> for &'a mut Grid<T> {
    fn into(self) -> GridViewMut<'a, T> {
        self.sub_grid_mut(self.area()).unwrap()
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.into_vec().into_iter()
    }
}

impl<T: Debug> Debug for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.view(), f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InvalidGridAreaError {
    pub dimensions: Dimensions,
    pub area: Area<usize>
}

impl Display for InvalidGridAreaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid area within a grid with dimensions {}", self.area, self.dimensions)
    }
}

impl Error for InvalidGridAreaError {}

#[derive(Clone)]
pub struct GridView<'a, T> {
    grid: &'a Grid<T>,
    pub area: Area<usize>
}

impl<'a, T> GridView<'a, T> {
    pub fn area(&self) -> Area<usize> { self.area }
    
    pub fn get(&self, location: Point2D<usize>) -> Option<&T> {
        if !self.area.contains(location) { return None; }

        let location = location + self.area.top_left();
        let index = self.grid.backing_index(location);
        self.grid.tiles.get(index)
    }

    pub fn get_row(&self, row: usize) -> Option<&[T]> {
        if row > self.area.right() { return None };

        let row = row + self.area.top();
        let start = row * self.grid.dimensions.width() + self.area.left();
        let end = start + self.area.dimensions().width();

        Some(&self.grid.tiles[start..end])
    }

    pub fn get_column(&self, column: usize) -> Option<Vec<&T>> {
        if column > self.area.bottom() { return None };

        let offset = self.area.left() + column;
        let step_size = self.grid.dimensions.width();

        let column = self.grid.tiles[offset..]
            .iter()
            .step_by(step_size)
            .collect();

        Some(column)
    }

    pub fn enumerate(&self) -> impl Iterator<Item=(Point2D<usize>, &T)> {
        self.area.iter().zip(self.into_iter())
    }

    pub fn iter_rows(&'a self) -> GridRowIterator<'a, T> {
        GridRowIterator { grid: self, row: 0 }
    }

    pub fn iter_columns(&'a self) -> GridColumnIterator<'a, T> {
        GridColumnIterator { grid: self, column: 0 }
    }

    pub fn into_grid(&self) -> Grid<T> where T: Clone {
        let items = self.into_iter().cloned();
        Grid::from_iter(self.area.dimensions(), items).unwrap()
    }

    pub fn map<U>(&self, mapper: impl Fn(&T) -> U) -> Grid<U> {
        let items = self.into_iter().map(mapper);
        Grid::from_iter(self.area.dimensions(), items).unwrap()
    }

    pub fn enumerate_map<U: Debug>(&self, mapper: impl Fn((Point2D<usize>, &T)) -> U) -> Grid<U> {
        let items = self.enumerate().map(mapper);
        Grid::from_iter(self.area.dimensions(), items).unwrap()
    }
}

impl<'a, T> Index<Point2D<usize>> for GridView<'a, T> {
    type Output = T;

    fn index(&self, index: Point2D<usize>) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<'a, T> IntoIterator for &'a GridView<'a, T> {
    type Item = &'a T;
    type IntoIter = FlatMap<
        GridRowIterator<'a, T>,
        slice::Iter<'a, T>,
        fn(slice::Iter<'a, T>) -> <slice::Iter<'a, T> as IntoIterator>::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_rows().flat_map(IntoIterator::into_iter)
    }
}

impl<'a, T> Debug for GridView<'a, T> where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rows: Vec<Vec<_>> = self.iter_rows()
            .map(Iterator::collect)
            .collect();

        writeln!(f, "")?;
        for row in rows {
            writeln!(f, "{:?}", row)?;
        }

        Ok(())
    }
}

pub struct GridColumnIterator<'a, T> {
    grid: &'a GridView<'a, T>,
    column: usize
}

impl<'a, T> Iterator for GridColumnIterator<'a, T> {
    type Item = vec::IntoIter<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.column += 1;
        Some(self.grid.get_column(self.column - 1)?.into_iter())
    }
}

pub struct GridRowIterator<'a, T> {
    grid: &'a GridView<'a, T>,
    row: usize
}

impl<'a, T> Iterator for GridRowIterator<'a, T> {
    type Item = slice::Iter<'a, T>;

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
}

#[derive(Debug)]
pub enum GridParseError {
    ParseError(String),
    WrongDimensions(WrongDimensionsError)
}

impl Display for GridParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let err = match self {
            GridParseError::ParseError(err) => err.to_string(),
            GridParseError::WrongDimensions(err) => err.to_string()
        };

        write!(f, "{}", err)
    }
}

impl Error for GridParseError {}

impl Grid<bool> {
    pub fn parse(dimensions: Dimensions, input: &str) -> Result<Grid<bool>, GridParseError> {
        let cell = value(true, complete::char('#'))
            .or(value(false, complete::char('.')));

        let input: String = input.lines().flat_map(str::chars).collect();
        let cells = many0(cell).run(&input).map_err(GridParseError::ParseError)?;
        Grid::from_iter(dimensions, cells).map_err(GridParseError::WrongDimensions)
    }
}