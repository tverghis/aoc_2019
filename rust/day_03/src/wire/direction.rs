#[derive(Debug, PartialEq)]
pub enum Direction {
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

#[cfg(test)]
mod test {
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
}
