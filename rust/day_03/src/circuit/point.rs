use crate::circuit::Direction;
use std::convert::TryInto;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn manhattan_distance(lhs: Point, rhs: Point) -> u32 {
        ((lhs.x - rhs.x).abs() + (lhs.y - rhs.y).abs())
            .try_into()
            .unwrap()
    }

    pub fn distance_from_origin(self) -> u32 {
        Point::manhattan_distance(self, Point::new(0, 0))
    }
}

#[allow(clippy::suspicious_arithmetic_impl)] // Clippy doesn't like subtraction in an `Add` impl, but it's ok here
impl ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up(m) => Point::new(self.x, self.y + m),
            Direction::Right(m) => Point::new(self.x + m, self.y),
            Direction::Down(m) => Point::new(self.x, self.y - m),
            Direction::Left(m) => Point::new(self.x - m, self.y),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn point_add() {
        let origin = Point::new(0, 0);

        assert_eq!(origin + Direction::Up(1), Point::new(0, 1));
        assert_eq!(origin + Direction::Right(1), Point::new(1, 0));
        assert_eq!(origin + Direction::Down(1), Point::new(0, -1));
        assert_eq!(origin + Direction::Left(1), Point::new(-1, 0));
    }
}
