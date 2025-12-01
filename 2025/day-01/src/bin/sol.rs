const INITIAL_POSITION: i64 = 50;
const MAX_POSITION: i64 = 100;

fn main() {
    let input1 = include_str!("./input1.txt");

    let codes = parse_input(input1);

    println!("part1: {}", part1(&codes));

    println!("part2: {:?}", part2(&codes));
    println!("part2_brute_force: {:?}", part2_brute_force(&codes));
}

#[derive(Debug)]
struct Code {
    direction: char,
    value: i64,
}

fn parse_input(input: &str) -> Vec<Code> {
    let mut codes: Vec<Code> = Vec::new();

    for (direction, value) in input.trim().lines().map(|x| x.split_at(1)) {
        let code = Code {
            direction: direction.parse().unwrap(),
            value: value
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse: |{value}|")),
        };
        codes.push(code);
    }

    codes
}

fn part1(codes: &[Code]) -> i64 {
    let mut position = INITIAL_POSITION;
    let mut counter = 0;

    for code in codes {
        position = (position
            + match code.direction {
                'L' => -code.value,
                _ => code.value,
            })
        .rem_euclid(MAX_POSITION);

        // println!("Position {position} after move {code:#?}");
        if position == 0 {
            counter += 1
        }
    }

    counter
}

fn part2(codes: &[Code]) -> i64 {
    let mut position = INITIAL_POSITION;
    let mut counter = 0;

    for code in codes {
        let prev_position = position;

        position += match code.direction {
            'L' => -code.value,
            _ => code.value,
        };

        counter += position.div_euclid(MAX_POSITION).abs();

        // Count when the move lands on 0 moving left
        // It is already counted when moving right by the euclidean division
        position = position.rem_euclid(MAX_POSITION);
        if position == 0 && code.direction == 'L' {
            counter += 1;
        }

        // Do not count moving from 0, previous "if" already counted it
        // Only double counts on left movements
        if prev_position == 0 && code.direction == 'L' {
            counter -= 1;
        }

        // println!("Position {position}, counter: {counter} after move {code:?}");
    }

    counter
}

fn part2_brute_force(codes: &[Code]) -> i64 {
    let mut position = INITIAL_POSITION;
    let mut counter: i64 = 0;

    for code in codes {
        let step = match code.direction {
            'L' => -1,
            _ => 1,
        };

        let mut moves = code.value;

        while moves != 0 {
            position = (position + step).rem_euclid(MAX_POSITION);

            if position == 0 {
                counter += 1;
            }
            moves -= 1;
        }

        // println!("Position {position}, counter: {counter} after move {code:?}");
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1() {
        let input = parse_input(include_str!("./example1.txt"));
        let output = part1(&input);
        assert_eq!(output, 3);
    }

    // #[ignore]
    #[test]
    fn test_part2() {
        let input = parse_input(include_str!("./example1.txt"));
        let output = part2(&input);
        assert_eq!(output, 6);
    }

    #[test]
    fn test_part2_custom() {
        let input = parse_input(include_str!("./custom1.txt"));
        let output = part2(&input);
        assert_eq!(output, 10);
    }

    #[test]
    fn test_part2_brute_force() {
        let input = parse_input(include_str!("./example1.txt"));
        let output = part2_brute_force(&input);
        assert_eq!(output, 6);
    }

    #[test]
    fn test_part2_brute_force_custom() {
        let input = parse_input(include_str!("./custom1.txt"));
        let output = part2_brute_force(&input);
        assert_eq!(output, 10);
    }
}
