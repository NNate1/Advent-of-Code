use std::collections::HashMap;

use rusttype::Vector;

fn main() {
    let input = include_str!("./input.txt");

    dbg!(part1(&parse(input)));

    dbg!(part2(&parse(input)));
}

fn parse(input: &str) -> Vec<&str> {
    input
        .lines()
        // .map(|s| s.strip_suffix("A").unwrap())
        .collect()
}

fn create_number_pad() -> HashMap<char, Vector<i32>> {
    let mut number_pad = HashMap::new();

    number_pad.insert('7', Vector { x: 0, y: 0 });
    number_pad.insert('8', Vector { x: 1, y: 0 });
    number_pad.insert('9', Vector { x: 2, y: 0 });
    number_pad.insert('4', Vector { x: 0, y: 1 });
    number_pad.insert('5', Vector { x: 1, y: 1 });
    number_pad.insert('6', Vector { x: 2, y: 1 });
    number_pad.insert('1', Vector { x: 0, y: 2 });
    number_pad.insert('2', Vector { x: 1, y: 2 });
    number_pad.insert('3', Vector { x: 2, y: 2 });

    number_pad.insert(' ', Vector { x: 0, y: 3 });
    number_pad.insert('0', Vector { x: 1, y: 3 });
    number_pad.insert('A', Vector { x: 2, y: 3 });

    number_pad
}

fn create_directional_pad() -> HashMap<char, Vector<i32>> {
    let mut directional_pad = HashMap::new();

    directional_pad.insert(' ', Vector { x: 0, y: 0 });
    directional_pad.insert('^', Vector { x: 1, y: 0 });
    directional_pad.insert('A', Vector { x: 2, y: 0 });
    directional_pad.insert('<', Vector { x: 0, y: 1 });
    directional_pad.insert('v', Vector { x: 1, y: 1 });
    directional_pad.insert('>', Vector { x: 2, y: 1 });

    directional_pad
}

fn part1(codes: &Vec<&str>) -> usize {
    solve(codes, 3)
}

fn part2(codes: &Vec<&str>) -> usize {
    solve(codes, 26)

    // dfs(code, keypad, directional_pad, memo, depth)
}

fn dfs(
    code: &str,
    keypad: &HashMap<char, Vector<i32>>,
    directional_pad: &HashMap<char, Vector<i32>>,
    memo: &mut HashMap<(String, usize), usize>,
    depth: usize,
) -> usize {
    if depth == 0 {
        return code.len();
    }

    let mut current = keypad[&'A'];
    let mut length = 0;

    if memo.contains_key(&(code.to_string(), depth)) {
        *memo.entry(("ola".to_string(), 0)).or_default() += 1;
    }

    if let Some(value) = memo.get(&(code.to_string(), depth)) {
        return *value;
    }

    for digit in code.chars() {
        // println!("{} {:?}", digit, keypad);
        let dest = keypad[&digit];
        let movement = dest - current;

        let horizontal_movement = if movement.x > 0 {
            ">".repeat(movement.x.unsigned_abs() as usize)
        } else {
            "<".repeat(movement.x.unsigned_abs() as usize)
        };

        let vertical_movement = if movement.y > 0 {
            "v".repeat(movement.y.unsigned_abs() as usize)
        } else {
            "^".repeat(movement.y.unsigned_abs() as usize)
        };

        let mut sequence = String::new();

        if keypad[&' '] == current + (Vector { y: 0, ..movement }) {
            sequence.push_str(&vertical_movement);
            sequence.push_str(&horizontal_movement);
        } else if keypad[&' '] == current + (Vector { x: 0, ..movement }) || movement.x < 0 {
            sequence.push_str(&horizontal_movement);
            sequence.push_str(&vertical_movement);
        } else {
            sequence.push_str(&vertical_movement);
            sequence.push_str(&horizontal_movement);
        }

        sequence.push('A');

        // println!("Depth: {} Sequence: {}", depth, sequence);
        length += dfs(&sequence, directional_pad, directional_pad, memo, depth - 1);

        current = dest;
    }
    memo.insert((code.to_string(), depth), length);
    length
}

fn solve(codes: &Vec<&str>, robots: usize) -> usize {
    let number_pad = create_number_pad();
    let directional_pad = create_directional_pad();

    let mut sol = 0;
    let mut memo = HashMap::new();

    for code in codes {
        let length = dfs(code, &number_pad, &directional_pad, &mut memo, robots);

        println!("{} : {}", code, length);
        sol += length * code.strip_suffix("A").unwrap().parse::<usize>().unwrap();
    }

    sol
}

fn button_presses(code: &str, start: Vector<i32>, keypad: &HashMap<char, Vector<i32>>) -> String {
    let mut sequence = String::new();

    let mut current = start;
    for digit in code.chars() {
        let dest = keypad[&digit];
        let movement = dest - current;

        let horizontal_movement = if movement.x > 0 {
            ">".repeat(movement.x.unsigned_abs() as usize)
        } else {
            "<".repeat(movement.x.unsigned_abs() as usize)
        };

        let vertical_movement = if movement.y > 0 {
            "v".repeat(movement.y.unsigned_abs() as usize)
        } else {
            "^".repeat(movement.y.unsigned_abs() as usize)
        };

        if keypad[&' '] == current + (Vector { y: 0, ..movement }) {
            sequence.push_str(&vertical_movement);
            sequence.push_str(&horizontal_movement);
        } else if keypad[&' '] == current + (Vector { x: 0, ..movement }) || movement.x < 0 {
            sequence.push_str(&horizontal_movement);
            sequence.push_str(&vertical_movement);
        } else {
            sequence.push_str(&vertical_movement);
            sequence.push_str(&horizontal_movement);
        }

        sequence.push('A');

        current = dest;
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1_ex0() {
        let input = include_str!("./example0.txt");
        let output = part1(&parse(input));
        assert_eq!(output, 68 * 29);
        // panic!()
    }

    // #[ignore]
    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");
        let output = part1(&parse(input));
        assert_eq!(output, 126384);
    }
}
