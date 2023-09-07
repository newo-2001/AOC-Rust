use std::fmt::Display;

use crate::parsing::InvalidTokenError;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bit {
    On,
    Off
}

impl Bit {
    pub fn is_enabled(self) -> bool { self == Bit::On }
    pub fn is_on(self) -> bool { self == Bit::On }
    pub fn is_solid(self) -> bool { self == Bit::On }

    pub fn digit(self) -> char {
        match self {
            Self::Off => '0',
            Self::On => '1'
        }
    }

    pub fn invert(self) -> Self {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off
        }
    }
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
            | '#' 
            | '1' => Self::On,
            | '.'
            | '0' => Self::Off,
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