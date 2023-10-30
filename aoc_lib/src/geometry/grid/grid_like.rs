use std::{slice, vec, fmt::Debug};

use thiserror::Error;

use crate::geometry::{Point2D, Area, WrongDimensionsError};

use super::{Grid, InvalidGridAreaError};

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
        self.area().into_iter().zip(self.iter())
    }

    fn iter_rows(&self) -> GridRowIterator<Self> {
        GridRowIterator { grid: self, row: 0 }
    }

    fn iter_columns(&self) -> GridColumnIterator<Self> {
        GridColumnIterator { grid: self, column: 0 }
    }
}

pub trait GridLikeMut: GridLike {
    fn get_mut(&mut self, location: Point2D<usize>) -> Option<&mut <Self as GridLike>::GridItem>;
    fn get_row_mut(&mut self, row: usize) -> Option<&mut [<Self as GridLike>::GridItem]>;
    fn get_column_mut(&mut self, column: usize) -> Option<Vec<&mut <Self as GridLike>::GridItem>>;

    fn fill(&mut self, value: <Self as GridLike>::GridItem) where <Self as GridLike>::GridItem: Clone {
        let area = self.area();
        for row in area.top()..=area.bottom() {
            let row = self.get_row_mut(row).unwrap();
            row.fill(value.clone());
        }
    }

    fn replace(&mut self, grid: Grid<Self::GridItem>) -> Result<(), WrongDimensionsError>
        where Self::GridItem: Clone
    {
        let area = self.area();
        if area.dimensions() != grid.area().dimensions() {
            return Err(WrongDimensionsError { expected: area.dimensions() })
        }

        for (row_index, new_row) in (0..area.dimensions().height()).zip(grid.into_rows()) {
            let row = self.get_row_mut(row_index).unwrap();
            row.clone_from_slice(&new_row);
        }

        Ok(())
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
        Some(self.grid.get_row(self.row - 1)?.iter())
    }
}

macro_rules! impl_grid_traits {
    ($type:ty) => {
        impl<T> std::ops::Index<Point2D<usize>> for $type {
            type Output = T;

            fn index(&self, index: crate::geometry::Point2D<usize>) -> &Self::Output {
                self.get(index).unwrap()
            }
        }

        impl<T: std::fmt::Debug> std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let rows: Vec<Vec<_>> = self.iter_rows()
                    .map(Iterator::collect)
                    .collect();
                
                writeln!(f)?;
                for row in rows {
                    writeln!(f, "{:?}", row)?;
                }

                Ok(())
            }
        }

        impl<T: std::fmt::Display> std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                use itertools::Itertools;

                writeln!(f)?;
                for row in self.iter_rows() {
                    let line = row.map(ToString::to_string).join("");

                    writeln!(f, "{line}")?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum GridError {
    #[error("Wrong dimensions were supplied")]
    WrongDimensions(#[from] WrongDimensionsError),
    #[error("Invalid grid area was supplied")]
    InvalidArea(#[from] InvalidGridAreaError)
}

macro_rules! impl_grid_traits_mut {
    ($type:ty) => {
        impl<T> std::ops::IndexMut<Point2D<usize>> for $type {
            fn index_mut(&mut self, index: crate::geometry::Point2D<usize>) -> &mut Self::Output {
                self.get_mut(index).unwrap()
            }
        }
    };
}

pub(crate) use { impl_grid_traits, impl_grid_traits_mut };