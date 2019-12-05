/// Ugly solutions, but I tried to do this without converting to Strings.

#[allow(clippy::unreadable_literal)]

fn main() {
    println!("Part 1: {}", solution(353096..=843212));
}

fn solution(range: std::ops::RangeInclusive<u32>) -> usize {
    range.filter(|&n| is_valid_password(n)).count()
}

fn is_valid_password(n: u32) -> bool {
    has_two_adjacent_digits(n) && has_increasing_digits(n)
}

fn has_two_adjacent_digits(n: u32) -> bool {
    let mut n = n;
    let mut last_matched_digit = None;
    let mut is_valid = false;

    for _ in 0..5 {
        if n % 10 == (n % 100) / 10 {
            if let Some(p) = last_matched_digit {
                is_valid = p != (n % 10);
            } else {
                is_valid = true;
            }

            last_matched_digit = Some(n % 10);
        } else {
            if is_valid {
                return true;
            }
            last_matched_digit = None;
        }

        n /= 10;
    }

    is_valid
}

fn has_increasing_digits(n: u32) -> bool {
    let mut n = n;

    for _ in 0..5 {
        if n % 10 < (n % 100) / 10 {
            return false;
        }

        n /= 10;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adjacent_digits() {
        assert!(has_two_adjacent_digits(223450));
        assert!(has_two_adjacent_digits(133456));
        assert!(has_two_adjacent_digits(112233));
        assert!(has_two_adjacent_digits(111122));
        assert!(!has_two_adjacent_digits(123444));
        assert!(!has_two_adjacent_digits(123789));
        assert!(!has_two_adjacent_digits(111111));
    }

    #[test]
    fn test_increasing_digits() {
        assert!(has_increasing_digits(111123));
        assert!(has_increasing_digits(135679));
        assert!(!has_increasing_digits(223450));
    }

    #[test]
    fn test_valid_password() {
        assert!(!is_valid_password(111111));
        assert!(!is_valid_password(223450));
        assert!(!is_valid_password(123789));
    }
}
