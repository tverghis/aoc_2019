use crate::wire::Direction;
use crate::wire::Line;
use crate::wire::Point;

#[derive(Debug, PartialEq)]
pub struct Wire {
    segments: Vec<Line>,
}

impl Wire {
    pub fn intersection_points(&self, other: &Wire) -> Vec<Point> {
        let mut int_points = Vec::new();
        let origin = Point::new(0, 0);

        for l1 in &self.segments {
            for l2 in &other.segments {
                if let Some(int_pt) = Line::intersection_point(l1, l2) {
                    if int_pt != origin {
                        int_points.push(int_pt);
                    }
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

#[cfg(test)]
mod test {
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
