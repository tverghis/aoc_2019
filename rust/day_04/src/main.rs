fn main() {
    println!("Part 1: {}", part_1(353096..=843212));
}

fn part_1(range: std::ops::RangeInclusive<u32>) -> usize {
    range.filter(|&n| is_valid_password(n)).count()
}

fn is_valid_password(n: u32) -> bool {
    has_two_adjacent_digits(n) && has_increasing_digits(n)
}

fn has_two_adjacent_digits(n: u32) -> bool {
    let mut n = n;

    for _ in 0..5 {
        if n % 10 == (n % 100) / 10 {
            return true;
        }

        n = n / 10;
    }

    false
}

fn has_increasing_digits(n: u32) -> bool {
    let mut n = n;

    for _ in 0..5 {
        if n % 10 < (n % 100) / 10 {
            return false;
        }

        n = n / 10;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adjacent_digits() {
        assert!(has_two_adjacent_digits(111111));
        assert!(has_two_adjacent_digits(223450));
        assert!(has_two_adjacent_digits(133456));
        assert!(!has_two_adjacent_digits(123789));
    }

    #[test]
    fn test_increasing_digits() {
        assert!(has_increasing_digits(111123));
        assert!(has_increasing_digits(135679));
        assert!(!has_increasing_digits(223450));
    }

    #[test]
    fn test_valid_password() {
        assert!(is_valid_password(111111));
        assert!(!is_valid_password(223450));
        assert!(!is_valid_password(123789));
    }
}
