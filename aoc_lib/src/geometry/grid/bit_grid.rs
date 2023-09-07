use std::fmt::{Display, Formatter, self};

use itertools::Itertools;

use crate::math::Bit;

use super::{GridView, Grid, GridLike};

impl Display for GridView<'_, Bit> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in self.iter_rows() {
            let line = row.map(ToString::to_string).join("");

            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

impl Display for Grid<Bit> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.view(), f)
    }
}

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