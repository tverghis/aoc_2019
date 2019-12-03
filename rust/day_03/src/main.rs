use std::fs;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn manhattan_distance(lhs: Point, rhs: Point) -> i32 {
        (lhs.x - rhs.x).abs() + (lhs.y - rhs.y).abs()
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

#[derive(Debug, PartialEq)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }

    fn slope(&self) -> f32 {
        (self.p2.y - self.p1.y) as f32 / (self.p2.x - self.p1.x) as f32
    }

    fn are_parallel(lhs: &Line, rhs: &Line) -> bool {
        (lhs.slope() - rhs.slope()).abs() < std::f32::EPSILON // Clippy doesn't like exact float cmp
    }

    fn intersection_point(lhs: &Line, rhs: &Line) -> Option<Point> {
        if Line::are_parallel(lhs, rhs) {
            return None;
        }

        unimplemented!();
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up(i32),
    Right(i32),
    Down(i32),
    Left(i32),
}

impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        let d = input.bytes().nth(0).unwrap();
        let m: i32 = input[1..].parse().unwrap();

        match d {
            b'U' => Direction::Up(m),
            b'R' => Direction::Right(m),
            b'D' => Direction::Down(m),
            b'L' => Direction::Left(m),
            _ => panic!(format!("Unknown direction {}", d)),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Wire {
    segments: Vec<Line>,
}

impl Wire {
    fn intersection_points(&self, other: &Wire) -> Vec<Point> {
        let mut int_points = Vec::new();

        for l1 in &self.segments {
            for l2 in &other.segments {
                if let Some(int_pt) = Line::intersection_point(l1, l2) {
                    int_points.push(int_pt);
                }
            }
        }

        return int_points;
    }
}

impl From<&str> for Wire {
    fn from(input: &str) -> Self {
        let mut last_point = Point::new(0, 0);

        let segments = input
            .split(',')
            .map(Direction::from)
            .map(|d| {
                let next_point = last_point + d;
                let l = Line::new(last_point, next_point);
                last_point = next_point;

                l
            })
            .collect();

        Wire { segments }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not open input file");

    let wires: Vec<Wire> = input.lines().map(Wire::from).collect();
    assert_eq!(wires.len(), 2);

    let intersection_distances: Vec<i32> = wires[0]
        .intersection_points(&wires[1])
        .iter()
        .map(|&p| Point::manhattan_distance(p, Point::new(0, 0)))
        .collect();

    dbg!(intersection_distances);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn direction_from() {
        assert_eq!(Direction::from("U12"), Direction::Up(12));
        assert_eq!(Direction::from("R12"), Direction::Right(12));
        assert_eq!(Direction::from("D12"), Direction::Down(12));
        assert_eq!(Direction::from("L12"), Direction::Left(12));
    }

    #[test]
    #[should_panic]
    fn direction_from_invalid_direction() {
        let _ = Direction::from("F32");
    }

    #[test]
    #[should_panic]
    fn direction_from_no_magnitude() {
        let _ = Direction::from("R");
    }

    #[test]
    #[should_panic]
    fn direction_from_invalid_magnitude() {
        let _ = Direction::from("R5HELLO");
    }

    #[test]
    fn point_add() {
        let origin = Point::new(0, 0);

        assert_eq!(origin + Direction::Up(1), Point::new(0, 1));
        assert_eq!(origin + Direction::Right(1), Point::new(1, 0));
        assert_eq!(origin + Direction::Down(1), Point::new(0, -1));
        assert_eq!(origin + Direction::Left(1), Point::new(-1, 0));
    }

    #[test]
    fn wire_from() {
        let input = "R10,D3,L10,U3";

        assert_eq!(
            Wire::from(input),
            Wire {
                segments: vec![
                    Line::new(Point::new(0, 0), Point::new(10, 0)),
                    Line::new(Point::new(10, 0), Point::new(10, -3)),
                    Line::new(Point::new(10, -3), Point::new(0, -3)),
                    Line::new(Point::new(0, -3), Point::new(0, 0)),
                ]
            }
        );
    }
}
