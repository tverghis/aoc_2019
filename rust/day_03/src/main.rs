mod circuit;

use circuit::Wire;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not open input file");

    let wires: Vec<Wire> = input.lines().map(Wire::from).collect();
    assert_eq!(wires.len(), 2);

    println!("Part 1: {}", part_1(&wires));
    println!("Part 2: {}", part_2(&wires));
}

fn part_1(wires: &[Wire]) -> u32 {
    wires[0]
        .intersection_points(&wires[1])
        .iter()
        .map(|p| p.distance_from_origin())
        .min()
        .unwrap()
}

fn part_2(wires: &[Wire]) -> u32 {
    wires[0]
        .intersection_points(&wires[1])
        .iter()
        .map(|&p| wires[0].distance_to_point(p) + wires[1].distance_to_point(p))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_tests_part_1() {
        let wires = vec![Wire::from("R8,U5,L5,D3"), Wire::from("U7,R6,D4,L4")];

        assert_eq!(part_1(&wires), 6);
    }

    #[test]
    fn sample_tests_part_2() {
        let wires = vec![Wire::from("R8,U5,L5,D3"), Wire::from("U7,R6,D4,L4")];
        assert_eq!(part_2(&wires), 30);

        let wires = vec![
            Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            Wire::from("U62,R66,U55,R34,D71,R55,D58,R83"),
        ];
        assert_eq!(part_2(&wires), 610);

        let wires = vec![
            Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        ];
        assert_eq!(part_2(&wires), 410);
    }
}
