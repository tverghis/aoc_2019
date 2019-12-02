use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read from file");

    let sum = input.lines().fold(0, |acc, x| {
        acc + mass_to_fuel(x.parse::<i32>().expect("Could not convert input to i32"))
    });
    dbg!(sum);
}

fn mass_to_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_to_fuel() {
        assert_eq!(mass_to_fuel(12), 2);
        assert_eq!(mass_to_fuel(14), 2);
        assert_eq!(mass_to_fuel(1969), 654);
        assert_eq!(mass_to_fuel(100756), 33583);
    }
}
