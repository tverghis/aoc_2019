use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read from file");

    let sum = input.lines().fold(0, |acc, x| {
        acc + total_fuel_mass(mass_to_fuel(
            x.parse::<i32>().expect("Could not convert input to i32"),
        ))
    });
    dbg!(sum);
}

fn mass_to_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn total_fuel_mass(fuel_mass: i32) -> i32 {
    if fuel_mass <= 0 {
        return 0;
    }

    fuel_mass + total_fuel_mass(mass_to_fuel(fuel_mass))
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

    #[test]
    fn test_total_fuel_mass() {
        assert_eq!(total_fuel_mass(2), 2 + 0);
        assert_eq!(total_fuel_mass(654), 654 + 216 + 70 + 21 + 5);
        assert_eq!(
            total_fuel_mass(33583),
            33583 + 11192 + 3728 + 1240 + 411 + 135 + 43 + 12 + 2
        );
    }
}
