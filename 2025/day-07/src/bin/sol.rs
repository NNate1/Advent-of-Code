use std::{
    collections::{HashMap, HashSet},
    fmt::{self},
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let (start, splitters) = parse_input(input);
    println!("part 1: {}", part1(start, &splitters));
    println!("part 2: {}", part2(start, &splitters));
}

fn parse_input(input: &str) -> (Position, HashSet<Position>) {
    let start = input
        .lines()
        .take(1)
        .flat_map(str::chars)
        .enumerate()
        .find(|&(_x, c)| c == 'S')
        .map(|(x, _c)| Position { x: x as i64, y: 0 })
        .unwrap();

    let splitters = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_x, c)| c == '^')
                .map(move |(x, _)| Position {
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect();

    (start, splitters)
}

fn part1(start: Position, splitters: &HashSet<Position>) -> i64 {
    let max_rows = splitters.iter().map(|p| p.y).max().unwrap();
    let mut visited = HashSet::<Position>::new();

    part1_recursive(splitters, max_rows, start, &mut visited)
}

fn part1_recursive(
    splitters: &HashSet<Position>,
    max_rows: i64,
    mut position: Position,          // `mut` is necessary for the while loop
    visited: &mut HashSet<Position>, // Shared mutable state
) -> i64 {
    while position.y <= max_rows && !visited.contains(&position) {
        visited.insert(position);

        if splitters.contains(&position) {
            let left_pos = Position {
                x: position.x - 1,
                y: position.y,
            };
            let right_pos = Position {
                x: position.x + 1,
                y: position.y,
            };

            let left_splits = part1_recursive(splitters, max_rows, left_pos, visited);
            let right_splits = part1_recursive(splitters, max_rows, right_pos, visited);

            return 1 + left_splits + right_splits;
        }

        position.y += 1;
    }

    0
}

fn part2(start: Position, splitters: &HashSet<Position>) -> i64 {
    let max_rows = splitters.iter().map(|p| p.y).max().unwrap();
    let mut visited = HashMap::new();

    part2_recursive(splitters, max_rows, start, &mut visited)
}

fn part2_recursive(
    splitters: &HashSet<Position>,
    max_rows: i64,
    mut position: Position,
    memo: &mut HashMap<Position, i64>,
) -> i64 {
    while position.y <= max_rows {
        if splitters.contains(&position) {
            if let Some(&timeline_counter) = memo.get(&position) {
                return timeline_counter;
            }

            let left_pos = Position {
                x: position.x - 1,
                y: position.y,
            };
            let right_pos = Position {
                x: position.x + 1,
                y: position.y,
            };

            let left_timelines = part2_recursive(splitters, max_rows, left_pos, memo);
            let right_timelines = part2_recursive(splitters, max_rows, right_pos, memo);

            let result = left_timelines + right_timelines;

            memo.insert(position, result);
            return result;
        }
        position.y += 1;
    }

    1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("example.txt");
        let (start, splitters) = parse_input(input);
        assert_eq!(21, part1(start, &splitters));
    }

    #[test]
    fn part2_test() {
        let input = include_str!("example.txt");
        let (start, splitters) = parse_input(input);
        assert_eq!(40, part2(start, &splitters));
    }
}
