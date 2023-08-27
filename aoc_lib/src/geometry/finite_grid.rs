
use std::{fmt::{Display, Formatter, self, Debug}, error::Error, marker::PhantomData};

use itertools::Itertools;

use crate::iteration::ExtraIter;

use super::{Point2D, Dimensions, WrongDimensionsError, Area};

#[derive(Debug, Clone)]
pub struct FiniteGrid<T>
{
    dimensions: Dimensions,
    tiles: Box<[T]>
}

impl<T> FiniteGrid<T>
{
    pub fn empty(dimensions: Dimensions) -> FiniteGrid<T>
        where T: Default + Clone
    {
        let tiles = vec![T::default(); dimensions.surface_area()];

        FiniteGrid {
            dimensions,
            tiles: tiles.into_boxed_slice()
        }
    }

    pub fn from_rows<I, Row>(dimensions: Dimensions, rows: I) -> Result<FiniteGrid<T>, WrongDimensionsError>
        where I: IntoIterator<Item=Row>,
              Row: IntoIterator<Item=T>
    {
        let tiles = rows.into_iter()
            .flatten()
            .collect::<Vec<T>>()
            .into_boxed_slice();

        if tiles.len() == dimensions.surface_area() {
            Ok(FiniteGrid { dimensions, tiles })
        } else {
            Err(WrongDimensionsError { expected: dimensions })
        }
    }

    fn valid_sub_grid(&self, area: &Area<usize>) -> Result<(), InvalidGridAreaError> {
        if area.top_left() < Point2D::zero() || area.bottom_right() >= self.dimensions.clone().into() {
            Err(InvalidGridAreaError { dimensions: self.dimensions, area: *area })
        } else { Ok(()) }
    }

    pub fn sub_grid(&self, area: Area<usize>) -> Result<FiniteGridView<T>, InvalidGridAreaError> {
        self.valid_sub_grid(&area)?;
        Ok(FiniteGridView { grid: self, area, phantom: PhantomData })
    }

    pub fn sub_grid_mut<'a>(&'a mut self, area: Area<usize>) -> Result<FiniteGridViewMut<T>, InvalidGridAreaError> {
        self.valid_sub_grid(&area)?;
        Ok(FiniteGridViewMut { grid: self, area, phantom: PhantomData })
    }

    pub fn view(&self) -> FiniteGridView<T> { self.into() }
    pub fn view_mut(&mut self) -> FiniteGridViewMut<T> { self.into() }

    pub fn area(&self) -> Area<usize> { self.dimensions.into() }
}

impl<T: Debug> Display for FiniteGrid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.view().fmt(f)
    }
}

impl<T> AsRef<FiniteGrid<T>> for FiniteGrid<T> {
    fn as_ref(&self) -> &FiniteGrid<T> { self }
}

impl<T> AsMut<FiniteGrid<T>> for FiniteGrid<T> {
    fn as_mut(&mut self) -> &mut FiniteGrid<T> { self }
}

impl<'a, T> Into<FiniteGridView<'a, T>> for &'a FiniteGrid<T> {
    fn into(self) -> FiniteGridView<'a, T> {
        self.sub_grid(self.area()).unwrap()
    }
}

impl<'a, T> Into<FiniteGridViewMut<'a, T>> for &'a mut FiniteGrid<T> {
    fn into(self) -> FiniteGridViewMut<'a, T> {
        self.sub_grid_mut(self.area()).unwrap()
    }
}

impl<T> IntoIterator for FiniteGrid<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.into_vec().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a FiniteGrid<T>
{
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.tiles.into_iter()
    }
}

#[derive(Debug)]
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

pub struct AbstractFiniteGridView<T, G: AsRef<FiniteGrid<T>>>
{
    grid: G,
    area: Area<usize>,
    phantom: PhantomData<T>
}

pub type FiniteGridView<'a, T> = AbstractFiniteGridView<T, &'a FiniteGrid<T>>;
pub type FiniteGridViewMut<'a, T> = AbstractFiniteGridView<T, &'a mut FiniteGrid<T>>;

impl<T, G> AbstractFiniteGridView<T, G> where G: AsRef<FiniteGrid<T>> {
    fn backing_index(&self, Point2D(x, y): Point2D<usize>) -> usize {
        y * self.grid.as_ref().dimensions.width() + x
    }
    
    fn absolute_location(&self, location: Point2D<usize>) -> Point2D<usize> {
        location + self.area.top_left()
    }

    pub fn get(&self, location: Point2D<usize>) -> Option<&T> {
        let location = self.absolute_location(location);
        let index = self.backing_index(location);
        self.grid.as_ref().tiles.get(index)
    }

    pub fn iter_rows(&self) -> impl Iterator<Item=impl Iterator<Item=&T>> {
        let grid = self.grid.as_ref();
        let width = grid.dimensions.width();

        (self.area.top_left().y()..=self.area.bottom_right().y()).map(move |row| {
            let start = row * width;
            grid.tiles[start..start + width].iter()
        })
    }

    pub fn iter_columns(&self) -> impl Iterator<Item=impl Iterator<Item=&T>> {
        let Point2D(min_x, min_y) = self.area.top_left();
        let Point2D(max_x, max_y) = self.area.bottom_right();
        let grid = self.grid.as_ref();

        (min_x..=max_x).map(move |column| {
            (min_y..=max_y).map(move |row| {
                &grid.tiles[row * grid.dimensions.width() + column]
            })
        })
    }

    pub fn get_row(&self, row: usize) -> &[T] {
        let grid = self.grid.as_ref();
        let start = row * grid.dimensions.width() + self.area.left();
        let end = start + self.area.dimensions().width();

        &grid.tiles[start..end]
    }

    pub fn get_column(&self, column: usize) -> Box<[&T]> {
        let grid = self.grid.as_ref();
        let offset = self.area.left() + column;
        let step_size = grid.dimensions.width();

        grid.tiles.iter()
            .linear_sequence(step_size, offset)
            .collect()
    }
}

impl<T, G> AbstractFiniteGridView<T, G> where G: AsRef<FiniteGrid<T>> + AsMut<FiniteGrid<T>> {
    pub fn get_mut(&mut self, location: Point2D<usize>) -> Option<&mut T> {
        let location = self.absolute_location(location);
        let index = self.backing_index(location);
        self.grid.as_mut().tiles.get_mut(index)
    }

    pub fn get_row_mut(&mut self, row: usize) -> &mut [T] {
        let grid = self.grid.as_mut();
        let start = row * grid.dimensions.width() + self.area.left();
        let end = start + self.area.dimensions().width();

        &mut grid.tiles[start..end]
    }

    pub fn get_column_mut(&mut self, column: usize) -> Box<[&mut T]> {
        let grid = self.grid.as_mut();
        let offset = self.area.left() + column;
        let step_size = grid.dimensions.width();

        grid.tiles.iter_mut()
            .linear_sequence(step_size, offset)
            .collect()
    }
}

impl<T, G> Display for AbstractFiniteGridView<T, G>
    where G: AsRef<FiniteGrid<T>>,
          T: Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.iter_rows() {
            writeln!(f, "{:?}", row.collect_vec())?;
        }

        Ok(())
    }
}