use crate::geometry::{Point2D, Area};

use super::{Grid, GridLike, grid_like::{impl_grid_traits_mut, impl_grid_traits, GridLikeMut}};

pub struct GridView<'a, T> {
    pub(crate) grid: &'a Grid<T>,
    pub(crate) area: Area<usize>
}

pub struct GridViewMut<'a, T> {
    pub(crate) grid: &'a mut Grid<T>,
    pub(crate) area: Area<usize>
}

impl<T> GridLikeMut for GridViewMut<'_, T> {
    fn get_mut(&mut self, location: Point2D<usize>) -> Option<&mut T> {
        if !self.area.contains(&location) { return None; }

        let location = location + self.area.top_left();
        let index = self.grid.backing_index(location);
        self.grid.tiles.get_mut(index)
    }

    fn get_row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        if row >= self.area.dimensions().height() { return None };

        let row = row + self.area.top();
        let start = row * self.grid.dimensions.width() + self.area.left();
        let end = start + self.area.dimensions().width();

        Some(&mut self.grid.tiles[start..end])
    }
    
    fn get_column_mut(&mut self, column: usize) -> Option<Vec<&mut T>> {
        if column >= self.area.dimensions().width() { return None };

        let offset = self.area.left() + column;
        let step_size = self.grid.dimensions.width();

        let column = self.grid.tiles[offset..]
            .iter_mut()
            .step_by(step_size)
            .collect();

        Some(column)
    }
}

#[allow(clippy::fallible_impl_from)]
impl<'a, T> From<&'a Grid<T>> for GridView<'a, T> {
    fn from(grid: &'a Grid<T>) -> Self {
        grid.sub_grid(grid.area()).unwrap()
    }
}

#[allow(clippy::fallible_impl_from)]
impl<'a, T> From<&'a mut Grid<T>> for GridViewMut<'a, T> {
    fn from(grid: &'a mut Grid<T>) -> Self {
        grid.sub_grid_mut(grid.area()).unwrap()
    }
}

macro_rules! impl_grid_like_for_view {
    ($type:ty) => {
        impl<T> GridLike for $type {
            type GridItem = T;

            fn get(&self, location: Point2D<usize>) -> Option<&T> {
                if !self.area.contains(&location) { return None; }

                let location = location + self.area.top_left();
                let index = self.grid.backing_index(location);
                self.grid.tiles.get(index)
            }

            fn get_row(&self, row: usize) -> Option<&[T]> {
                if row >= self.area.dimensions().height() { return None };

                let row = row + self.area.top();
                let start = row * self.grid.dimensions.width() + self.area.left();
                let end = start + self.area.dimensions().width();

                Some(&self.grid.tiles[start..end])
            }

            fn get_column(&self, column: usize) -> Option<Vec<&T>> {
                if column >= self.area.dimensions().width() { return None };

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

        impl<T: Clone> $type {
            #[must_use]
            pub fn owned(&self) -> Grid<T> {
                let tiles = self.iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .into_boxed_slice();

                Grid {
                    tiles,
                    dimensions: self.area.dimensions()
                }
            }
        }

        impl_grid_traits!($type);
    }
}

impl_grid_like_for_view!(GridView<'_, T>);
impl_grid_like_for_view!(GridViewMut<'_, T>);
impl_grid_traits_mut!(GridViewMut<'_, T>);