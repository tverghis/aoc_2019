use crate::wire::Point;

/// Lines can only be vertical or horizontal, since the direction of movement is always at
/// 90-degree angles.
#[derive(Debug, PartialEq)]
enum LineOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }

    fn slope(&self) -> f32 {
        (self.p2.y - self.p1.y) as f32 / (self.p2.x - self.p1.x) as f32
    }

    fn are_parallel(lhs: &Line, rhs: &Line) -> bool {
        lhs.orientation() == rhs.orientation()
    }

    fn orientation(&self) -> LineOrientation {
        if self.slope().is_infinite() {
            return LineOrientation::Vertical;
        }

        return LineOrientation::Horizontal;
    }

    fn contains_point(&self, p: Point) -> bool {
        let min_x = std::cmp::min(self.p1.x, self.p2.x);
        let min_y = std::cmp::min(self.p1.y, self.p2.y);
        let max_x = std::cmp::max(self.p1.x, self.p2.x);
        let max_y = std::cmp::max(self.p1.y, self.p2.y);

        match self.orientation() {
            LineOrientation::Horizontal => (min_x <= p.x) && (p.x <= max_x),
            LineOrientation::Vertical => (min_y <= p.y) && (p.y <= max_y),
        }
    }

    pub fn intersection_point(lhs: &Line, rhs: &Line) -> Option<Point> {
        if Line::are_parallel(lhs, rhs) {
            return None;
        }

        // One line is vertical, and the other is horizontal.
        // The point of intersection is at `(x_vert, y_horiz)`.
        let int_point = match lhs.orientation() {
            LineOrientation::Vertical => Point::new(lhs.p1.x, rhs.p1.y),
            LineOrientation::Horizontal => Point::new(rhs.p1.x, lhs.p1.y),
        };

        // However, we need to check that the point actually lies on the segments.
        if !lhs.contains_point(int_point) || !rhs.contains_point(int_point) {
            return None;
        }

        Some(int_point)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn intersection_point() {
        let l1 = Line::new(Point::new(1, 5), Point::new(10, 5));
        let l2 = Line::new(Point::new(5, 1), Point::new(5, 10));

        assert_eq!(Line::intersection_point(&l1, &l2), Some(Point::new(5, 5)));
    }

    #[test]
    fn intersection_parallel() {
        let l1 = Line::new(Point::new(0, 0), Point::new(0, 10));
        let l2 = Line::new(Point::new(5, 0), Point::new(5, 10));

        assert_eq!(Line::intersection_point(&l1, &l2), None);
    }
}
