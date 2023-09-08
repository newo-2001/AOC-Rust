use crate::math::Bit;

use super::GridLike;

pub trait BitGrid: GridLike<GridItem = Bit> {
    fn count_enabled(&self) -> usize;
    fn count_lit(&self) -> usize { self.count_enabled() }
    fn count_solid(&self) -> usize { self.count_enabled() }
}

impl<G: GridLike<GridItem = Bit>> BitGrid for G {
    fn count_enabled(&self) -> usize {
        self.iter()
            .filter(|&light| light.is_on())
            .count()
    }
}