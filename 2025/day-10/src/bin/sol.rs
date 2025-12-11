// use itertools::Itertools;

fn main() {
    // let input = include_str!("input.txt");
    let input = include_str!("example.txt");
    let machines = parse_input(input);
    println!("{machines:?}");
    // println!("part 1: {}", part1(&coordinates));
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i32>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    for line in input.lines() {
        let (lights_str, buttons_and_joltage) = line.split_once("]").unwrap();

        let lights = lights_str.chars().skip(1).map(|c| c == '#').collect();

        let (buttons_str, joltage_str) = buttons_and_joltage.split_once("{").unwrap();

        let buttons = buttons_str
            .split_whitespace()
            .map(|button| {
                let mut char_iter = button.chars();
                char_iter.next();
                char_iter.next_back();

                char_iter
                    .as_str()
                    .split(",")
                    .map(|button| button.parse().unwrap())
                    .collect()
            })
            .collect();

        let mut char_iter = joltage_str.chars();
        char_iter.next_back();

        let joltage = char_iter
            .as_str()
            .split(",")
            .map(|joltage| joltage.parse().unwrap())
            .collect();

        machines.push(Machine {
            lights,
            buttons,
            joltage,
        });
    }

    machines

    // input
    //     .lines()
    //     .map(|line| {
    //         let (x, y) = line.split_once(',').unwrap();
    //         return (x.parse().unwrap(), y.parse().unwrap());
    //     })
    //     .collect()
}

// fn part1(coordinates: &[(i64, i64)]) -> i64 {
//     coordinates
//         .iter()
//         .cartesian_product(coordinates.iter())
//         .map(|(&a, &b)| area(a, b))
//         .max()
//         .unwrap()
// }
//
// fn area(p1: (i64, i64), p2: (i64, i64)) -> i64 {
//     let result = (1 + (p1.0 - p2.0).abs()) * (1 + (p1.1 - p2.1).abs());
//
//     // println!("{p1:?} {p2:?} results in {result}");
//     result
// }
//
// #[cfg(test)]
// mod test {
//
//     use super::*;
//
//     #[test]
//     fn part1_test() {
//         let input = include_str!("example.txt");
//         let coordinates = parse_input(input);
//         assert_eq!(50, part1(&coordinates));
//     }
// }
