use std::{ops::{self, Add, Range}, collections::HashMap, str::Lines, error::Error};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point2D<T> where T: Add {
    pub x: T,
    pub y: T
}

pub type Coordinate = Point2D<i32>;

impl <'a, T> Point2D<T> where T : Add + 'a {
    pub fn neighbours<Out>(&'a self) -> impl IntoIterator<Item=Out> + 'a
        where &'a Point2D<T> : Add<Point2D<i32>, Output=Out> {

        use Direction::*;
        [
            North, East, South, West,
            NorthEast, NorthWest, SouthEast, SouthWest
        ].into_iter().map(move |direction| self + direction.unit_vector())
    }

    pub fn cardinal_neighbours<Out>(&'a self) -> impl IntoIterator<Item=Out> + 'a
        where &'a Point2D<T> : Add<Point2D<i32>, Output=Out> {

        use CardinalDirection::*;
        [ North, East, South, West ].iter()
            .map(move |direction| self + direction.unit_vector())
    }
}

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

impl_binop_point2d!(+ i32);
impl_binop_point2d!(- i32);

#[derive(Clone)]
pub struct Area {
    pub range_x: Range<i32>,
    pub range_y: Range<i32>
}

impl Area {
    pub fn iter<'a>(self) -> impl Iterator<Item=Point2D<i32>> + 'a {
        self.iter_rows().flatten()
    }

    pub fn iter_rows<'a>(self) -> impl Iterator<Item =impl Iterator<Item = Point2D<i32>>> + 'a {
        self.range_y.into_iter()
            .map(move |y| self.range_x.clone().into_iter()
                .map(move |x| Point2D { x, y }))
    }

    pub fn square(dimensions: usize) -> Area {
        Point2D { x: dimensions, y: dimensions }.into()
    }

    pub fn corners(&self) -> [Point2D<i32>; 4] {
        let x_start = self.range_x.start;
        let x_end = self.range_x.end - 1;
        let y_start = self.range_y.start;
        let y_end = self.range_y.end - 1;

        [
            Point2D { x: x_start, y: y_start },
            Point2D { x: x_end, y: y_start },
            Point2D { x: x_end, y: y_end },
            Point2D { x: x_start, y: y_end }
        ]
    }
}

impl Into<Area> for Point2D<usize> {
    fn into(self) -> Area {
        Area {
            range_x: 0..self.x as i32,
            range_y: 0..self.y as i32
        }
    }
}

impl From<(&Point2D<i32>, &Point2D<i32>)> for Area {
    fn from((top_left, bottom_right): (&Point2D<i32>, &Point2D<i32>)) -> Area {
        Area {
            range_x: top_left.x..bottom_right.x,
            range_y: top_left.y..bottom_right.y
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    pub squares: HashMap<Coordinate, T>
}

impl <T> Grid<T> {
    pub fn value_at(&self, point: &Point2D<i32>) -> Option<&T> {
        self.squares.get(point)
    }

    pub fn iter<'a>(&'a self, area: Area) -> impl Iterator<Item = (Point2D<i32>, Option<&'a T>)> {
        area.iter().map(|point| (point, self.value_at(&point)))
    }

    pub fn iter_rows(&self, area: Area) -> impl Iterator<Item = impl Iterator<Item = Option<&T>>> {
        area.iter_rows()
            .map(|row| row.map(|point| self.value_at(&point)))
    }

    pub fn map<R>(self, mapper: impl Fn(Point2D<i32>, T) -> R) -> Grid<R> {
        let squares = self.squares.into_iter().map(|(key, value)| {
            (key, mapper(key, value))
        }).collect();

        Grid { squares }
    }   

    pub fn from_lines(lines: Lines, mapper: impl Fn(char) -> Result<T, Box<dyn Error>>) -> Result<Grid<T>, Box<dyn Error>> {
        let squares = lines.into_iter()
            .enumerate().flat_map(|(y, line)| {
                let mapper = &mapper;
                line.chars().into_iter().enumerate()
                    .map(move |(x, char)| -> Result<(Point2D<i32>, T), Box<dyn Error>> {
                        Ok((Point2D {x: x as i32, y: y as i32}, mapper(char)?))
                    })
            }).collect::<Result<HashMap<Point2D<i32>, T>, Box<dyn Error>>>()?;

        Ok(Grid { squares })
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