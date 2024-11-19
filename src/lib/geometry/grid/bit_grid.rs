use crate::{math::Bit, iteration::ExtraIter};

use super::GridLike;

pub trait BitGrid: GridLike<GridItem = Bit> {
    fn pop_count(&self) -> usize;
}

impl<G: GridLike<GridItem = Bit>> BitGrid for G {
    /// Computes the "population count" of the grid
    /// (The amount of cells that are on)
    fn pop_count(&self) -> usize {
        self.iter().count_where(|light| light.is_on())
    }
}