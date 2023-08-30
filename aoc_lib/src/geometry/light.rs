use std::fmt::{Display, Formatter, self};

use itertools::Itertools;

use crate::parsing::InvalidTokenError;

use super::{GridView, Grid, GridLike};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Light {
    On,
    Off
}

impl Light {
    pub fn is_on(self) -> bool { self == Light::On }
}

impl Into<bool> for Light {
    fn into(self) -> bool { self.is_on() }
}

impl TryFrom<char> for Light {
    type Error = InvalidTokenError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '#' => Self::On,
            '.' => Self::Off,
            c => Err(InvalidTokenError(c))?
        })
    }
}

impl Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.is_on() { '#' } else { '.' })
    }
}

impl Default for Light {
    fn default() -> Self { Light::Off }
}

impl Display for GridView<'_, Light> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for row in self.iter_rows() {
            let line = row.map(ToString::to_string).join("");

            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

impl Display for Grid<Light> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.view(), f)
    }
}

pub trait LightGrid: GridLike<GridItem = Light> {
    fn count_lit(&self) -> usize;
}

impl<G: GridLike<GridItem = Light>> LightGrid for G {
    fn count_lit(&self) -> usize {
        self.iter()
            .filter(|&light| light.is_on())
            .count()
    }
}