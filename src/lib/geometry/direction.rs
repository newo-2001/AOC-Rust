use std::ops::Neg;

use nom::{branch::alt, combinator::value, character::complete::one_of, bytes::complete::tag, Parser};
use num::{One, Zero};

use crate::parsing::{TextParserResult, Parsable};

use super::{Point2D, Point3D};

pub trait Directional<V>: Sized {
    fn direction_vector(self) -> V;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Axis {
    Horizontal,
    Vertical
}

/// Directions that are relative to the observer in 2D space
/// These can be used to turn to a different [`CardinalDirection`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum RotationDirection {
    Left,
    Right
}

impl RotationDirection {
    /// Inverts the direction
    /// ```
    /// # use aoc_lib::geometry::RotationDirection;
    /// assert_eq!(RotationDirection::Left, RotationDirection::Right.inverse());
    /// assert_eq!(RotationDirection::Right, RotationDirection::Left.inverse());
    /// ```
    #[must_use]
    pub const fn inverse(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left
        }
    }
}

impl Parsable<'_> for RotationDirection {
    fn parse(input: &str) -> TextParserResult<Self> {
        Parser::or(
            value(Self::Left, one_of("Ll")),
            value(Self::Right, one_of("Rr"))
        ).parse(input)
    }
}

/// Directions that move along the axis of 2D space
/// ```text
///   N
/// W   E
///   S
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West
}

impl<T> Directional<Point2D<T>> for CardinalDirection
    where T: Zero + One + Neg<Output=T>
{
    /// Used to calculate the offset created by one step in a direction
    fn direction_vector(self) -> Point2D<T>
    {
        match self {
            Self::North => Point2D(T::zero(), -T::one()),
            Self::East => Point2D(T::one(), T::zero()),
            Self::South => Point2D(T::zero(), T::one()),
            Self::West => Point2D(-T::one(), T::zero())
        }
    }
}

impl CardinalDirection {
    /// Rotate this direction by a [`RotationDirection`].
    /// This has the effect of turning relative to an observer.
    #[must_use]
    pub const fn rotate(self, rotation_direction: RotationDirection) -> Self {
        match (self, rotation_direction) {
            | (Self::North, RotationDirection::Left)
            | (Self::South, RotationDirection::Right) => Self::West,
            | (Self::North, RotationDirection::Right)
            | (Self::South, RotationDirection::Left) => Self::East,
            | (Self::East, RotationDirection::Left)
            | (Self::West, RotationDirection::Right) => Self::North,
            | (Self::East, RotationDirection::Right)
            | (Self::West, RotationDirection::Left) => Self::South
        }
    }

    /// The direction opposite of this one
    #[must_use]
    pub const fn reverse(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East
        }
    }

    #[must_use]
    pub const fn all() -> [Self; 4] {
        [Self::North, Self::East, Self::South, Self::West]
    }

    #[must_use]
    pub const fn relative_char(self) -> char {
        match self {
            Self::North => 'U',
            Self::East => 'R',
            Self::South => 'D',
            Self::West => 'L'
        }
    }

    #[must_use]
    pub const fn absolute_char(self) -> char {
        match self {
            Self::North => 'N',
            Self::East => 'E',
            Self::South => 'S',
            Self::West => 'W'
        }
    }
}

impl Parsable<'_> for CardinalDirection {
    /// Parse a [`CardinalDirection`] from a variety of representations like:
    /// ```text
    ///   U   |   N   |   ^
    /// L   R | W   E | <   >
    ///   D   |   S   |   V
    /// ```
    fn parse(input: &str) -> TextParserResult<Self> {
        alt((
            value(Self::North, one_of("UuNn^")),
            value(Self::East, one_of("RrEe>")),
            value(Self::South, one_of("DdSsVv")),
            value(Self::West, one_of("LlWw<"))
        ))(input)
    }
}


/// Directions that move diagonally in 2D space.
/// ```text
/// NW NE
///  
/// SW WE
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]

pub enum OrdinalDirection {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest
}

impl<T> Directional<Point2D<T>> for OrdinalDirection
    where T: Zero + One + Neg<Output=T>
{
    /// Used to calculate the offset created by one step in a direction
    fn direction_vector(self) -> Point2D<T>
    {
        use CardinalDirection as Dir;
        match self {
            Self::NorthEast => Dir::North.direction_vector() + Dir::East.direction_vector(),
            Self::NorthWest => Dir::North.direction_vector() + Dir::West.direction_vector(),
            Self::SouthEast => Dir::South.direction_vector() + Dir::East.direction_vector(),
            Self::SouthWest => Dir::South.direction_vector() + Dir::West.direction_vector()
        }
    }
}

impl OrdinalDirection {
    #[must_use]
    pub const fn all() -> [Self; 4] {
        [Self::NorthEast, Self::SouthEast, Self::SouthWest, Self::NorthWest]
    }
}

/// Combines [`CardinalDirection`] and [`OrdinalDirection`].
/// This allows for movement along 8 directions in 2D space.
/// ```text
/// NW N NE
/// W     E
/// SW S WE
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction2D {
    Cardinal(CardinalDirection),
    Ordinal(OrdinalDirection)
}

impl<T> Directional<Point2D<T>> for Direction2D
    where T: Zero + One + Neg<Output=T>
{
    /// Used to calculate the offset created by one step in a direction
    fn direction_vector(self) -> Point2D<T> {
        match self {
            Self::Cardinal(direction) => direction.direction_vector(),
            Self::Ordinal(direction) => direction.direction_vector()
        }
    }
}

impl Direction2D {
    #[must_use]
    pub const fn all() -> [Self; 8] {
        use CardinalDirection as Card;
        use OrdinalDirection as Ord;

        [
            Self::Cardinal(Card::North),
            Self::Ordinal(Ord::NorthEast),
            Self::Cardinal(Card::East),
            Self::Ordinal(Ord::SouthEast),
            Self::Cardinal(Card::South),
            Self::Ordinal(Ord::SouthWest),
            Self::Cardinal(Card::West),
            Self::Ordinal(Ord::NorthWest)
        ]
    }
}

/// Directions that move along the axis of 3D space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction3D {
    North,
    East,
    South,
    West,
    Up,
    Down
}

impl<T> Directional<Point3D<T>> for Direction3D
    where T: Zero + One + Neg<Output=T>
{
    /// Used to calculate the offset created by one step in a direction.
    fn direction_vector(self) -> Point3D<T> {
        match self {
            Self::North => Point3D(T::zero(), -T::one(), T::zero()),
            Self::East => Point3D(T::one(), T::zero(), T::zero()),
            Self::South => Point3D(T::zero(), T::one(), T::zero()),
            Self::West => Point3D(-T::one(), T::zero(), T::zero()),
            Self::Up => Point3D(T::zero(), T::zero(), T::one()),
            Self::Down => Point3D(T::zero(), T::zero(), -T::one())
        }
    }
}

impl Direction3D {
    #[must_use]
    pub const fn all() -> [Self; 6] {
        [Self::North, Self::East, Self::South, Self::West, Self::Up, Self::Down]
    }
}

/// Directions that move to neighbouring tiles on a 2D hex grid.
/// ```text
///   \ N  /
/// NW +--+ NE
///   /    \
/// -+      +-
///   \    /
/// SW +--+ SE
///   / S  \
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexDirection {
    North,
    South,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest
}

impl<T> Directional<Point3D<T>> for HexDirection
    where T: Zero + One + Neg<Output=T>
{
    /// Used to calculate the offset created by one step in a direction.
    fn direction_vector(self) -> Point3D<T> {
        match self {
            Self::North => Point3D(T::zero(), -T::one(), T::one()),
            Self::NorthEast => Point3D(T::one(), -T::one(), T::zero()),
            Self::SouthEast => Point3D(T::one(), T::zero(), -T::one()),
            Self::South => Point3D(T::zero(), T::one(), -T::one()),
            Self::SouthWest => Point3D(-T::one(), T::one(), T::zero()),
            Self::NorthWest => Point3D(-T::one(), T::zero(), T::one())
        }
    }
}

impl HexDirection {
    #[must_use]
    pub const fn all() -> [Self; 6] {
        [
            Self::North, Self::NorthEast, Self::SouthEast,
            Self::South, Self::SouthWest, Self::NorthWest
        ]
    }

    /// Parse a [`HexDirection`] from a string.
    /// Valid representations are: NE, SE, S, N, NW, and SW.
    pub fn parse(input: &str) -> TextParserResult<Self> {
        alt((
            value(Self::NorthEast, tag("ne").or(tag("NE"))),
            value(Self::NorthWest, tag("nw").or(tag("NW"))),
            value(Self::SouthEast, tag("se").or(tag("SE"))),
            value(Self::SouthWest, tag("sw").or(tag("SW"))),
            value(Self::North, one_of("Nn")),
            value(Self::South, one_of("Ss"))
        )).parse(input)
    }
}