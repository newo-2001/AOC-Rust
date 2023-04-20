use std::ops::{self, Add};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point2D<T> where T: Add {
    x: T,
    y: T
}

pub type Coordinate = Point2D<usize>;

impl <'a, T> Point2D<T> where T : Add + 'a {
    pub fn neighbours<Out>(&'a self) -> impl Iterator<Item=Out> + 'a
        where &'a Point2D<T> : Add<Point2D<i32>, Output=Out> {

        use Direction::*;
        [
            North, East, South, West,
            NorthEast, NorthWest, SouthEast, SouthWest
        ].iter().map(move |direction| self + direction.unit_vector())
    }

    pub fn cardinal_neighbours<Out>(&'a self) -> impl Iterator<Item=Out> + 'a
        where &'a Point2D<T> : Add<Point2D<i32>, Output=Out> {

        use CardinalDirection::*;
        [ North, East, South, West ].iter()
            .map(move |direction| self + direction.unit_vector())
    }
}

impl_op_ex_commutative!(+ |left: &Point2D<usize>, right: &Point2D<i32>| -> Option<Point2D<usize>> {
    let x = left.x as i32 + right.x;
    let y = left.y as i32 + right.y;

    if x < 0 || y < 0 { None }
    else { Some(Point2D { x: x as usize, y: y as usize}) }
});

macro_rules! impl_binop_point2d {
    ($op:tt $T:ty) => {
        impl_op_ex!($op |left: &Point2D<$T>, right: &Point2D<$T>| -> Point2D<$T> {
            Point2D {
                x: left.x $op right.x,
                y: left.y $op right.y
            }
        });
    }
}

impl_binop_point2d!(+ usize);
impl_binop_point2d!(- usize);

impl_binop_point2d!(+ i32);
impl_binop_point2d!(- i32);

pub trait Grid<T> {
    fn value_at<'a>(&'a self, index: &Point2D<usize>) -> Option<&'a T>;
    fn iter_2d<'a>(&'a self) -> Box<dyn Iterator<Item = &T> + 'a>;
}

impl <T> Grid<T> for Vec<Vec<T>> {
    fn value_at(&self, point: &Point2D<usize>) -> Option<&T> {
        self.get(point.y)
            .map_or(None, |row| Some(&row[point.x]))
    }

    fn iter_2d<'a>(&'a self) -> Box<dyn Iterator<Item = &T> + 'a> {
        Box::new(self.iter().flat_map(|row| row.iter()))
    }
}

#[derive(Copy, Clone)]
pub enum CardinalDirection {
    North, East, South, West
}

impl CardinalDirection {
    pub fn unit_vector(&self) -> Point2D<i32> {
        use CardinalDirection::*;

        match self {
            North => Point2D { x: 0, y: 1 },
            East  => Point2D { x: 1, y: 0 },
            South => Point2D { x: 0, y: -1 },
            West  => Point2D { x: -1, y: 0 }
        }
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    North, East, South, West,
    NorthEast, SouthEast, SouthWest, NorthWest
}

impl Direction {
    pub fn unit_vector(&self) -> Point2D<i32> {
        use Direction::*;

        match self {
            North => CardinalDirection::North.unit_vector(),
            East => CardinalDirection::East.unit_vector(),
            South => CardinalDirection::South.unit_vector(),
            West => CardinalDirection::West.unit_vector(),
            NorthEast => North.unit_vector() + East.unit_vector(),
            NorthWest => North.unit_vector() + West.unit_vector(),
            SouthEast => South.unit_vector() + East.unit_vector(),
            SouthWest => South.unit_vector() + West.unit_vector()
        }
    }
}