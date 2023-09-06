use std::fmt::{Display, Formatter, self};

use itertools::Itertools;

use crate::parsing::InvalidTokenError;

use super::{GridView, Grid, GridLike};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bit {
    On,
    Off
}

impl Bit {
    pub fn is_enabled(self) -> bool { self == Bit::On }
    pub fn is_on(self) -> bool { self == Bit::On }
    pub fn is_solid(self) -> bool { self == Bit::On }
}

impl Into<bool> for Bit {
    fn into(self) -> bool { self == Bit::On }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        if value { Bit::On } else { Bit::Off }
    }
}

impl TryFrom<char> for Bit {
    type Error = InvalidTokenError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => Self::On,
            '.' => Self::Off,
            c => Err(InvalidTokenError(c))?
        })
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.is_on() { '#' } else { '.' })
    }
}

impl Default for Bit {
    fn default() -> Self { Bit::Off }
}

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