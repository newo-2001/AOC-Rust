use ahash::HashSet;
use aoc_lib::{geometry::Point3D, parsing::{ParseError, parse_lines, TextParser, Parsable}, iteration::ExtraIter};
use aoc_runner_api::SolverResult;
use nom::{character::complete::char, sequence::separated_pair};

#[derive(PartialEq, Eq)]
struct Brick(Vec<Point3D<u32>>);

impl Brick {
    fn positions(&self) -> impl Iterator<Item=Point3D<u32>> + '_ {
        self.0.iter().copied()
    }

    fn positions_mut(&mut self) -> impl Iterator<Item=&mut Point3D<u32>> {
        self.0.iter_mut()
    }

    fn contains(&self, position: Point3D<u32>) -> bool {
        self.0.contains(&position)
    }

    fn is_supported(&self, grid: &HashSet<Point3D<u32>>) -> bool {
        self.positions()
            .any(|Point3D(x, y, z)| {
                let below = Point3D(x, y, z - 1);
                (z == 1) || (grid.contains(&below) && !self.contains(below))
            })
    }

    fn drop(&mut self, grid: &mut HashSet<Point3D<u32>>) {
        while !self.is_supported(grid) {
            for point in self.positions_mut() {
                grid.remove(point);

                let Point3D(_, _, z) = point;
                *z -= 1;

                grid.insert(*point);
            }
        }
    }
}

impl FromIterator<Point3D<u32>> for Brick {
    fn from_iter<I: IntoIterator<Item = Point3D<u32>>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Brick {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let (Point3D(x1, y1, z1), Point3D(x2, y2, z2)) = separated_pair(
                Point3D::<u32>::parse,
                char('~'),
                Point3D::<u32>::parse
            ).run(input)?;
        
        Ok((x1..=x2).flat_map(|x| {
            (y1..=y2).flat_map(move |y| {
                (z1..=z2).map(move |z| Point3D(x, y, z))
            })
        }).collect())
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut bricks = parse_lines(Brick::parse, input)?;
    let mut grid: HashSet<Point3D<u32>> = bricks.iter()    
        .flat_map(Brick::positions)
        .collect();

    while !bricks.iter().all(|brick| brick.is_supported(&grid)) {
        for brick in &mut bricks {
            brick.drop(&mut grid);
        }
    }

    let disintegratable = bricks.iter().count_where(|brick| {
        for pos in brick.positions() {
            grid.remove(&pos);
        }
        
        let stable = bricks.iter().all(|other| other.is_supported(&grid));

        for pos in brick.positions() {
            grid.insert(pos);
        }

        stable
    });

    Ok(Box::new(disintegratable))
}