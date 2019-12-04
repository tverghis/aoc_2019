mod wire;

use std::fs;
use wire::Wire;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not open input file");

    let wires: Vec<Wire> = input.lines().map(Wire::from).collect();
    assert_eq!(wires.len(), 2);

    println!("Part 1: {}", part_1(&wires));
}

fn part_1(wires: &[Wire]) -> i32 {
    wires[0]
        .intersection_points(&wires[1])
        .iter()
        .map(|p| p.distance_from_origin())
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_tests() {
        let wire_1 = Wire::from("R8,U5,L5,D3");
        let wire_2 = Wire::from("U7,R6,D4,L4");

        assert_eq!(wire_1.segments.len(), 4);
        assert_eq!(wire_2.segments.len(), 4);

        let wires = vec![wire_1, wire_2];

        assert_eq!(part_1(&wires), 6);
    }
}
