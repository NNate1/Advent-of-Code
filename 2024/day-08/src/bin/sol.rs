use itertools::Itertools;
use rusttype::Point;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input);

    println!("part1: {}\npart2: {}", output.0, output.1);
}

struct ParseResult(i32, i32, HashMap<char, Vec<Point<i32>>>);

fn parse(input: &str) -> ParseResult {
    let mut antennas = HashMap::<char, Vec<Point<i32>>>::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            antennas.entry(c).or_default().push(Point {
                x: x as i32,
                y: y as i32,
            });
        }
    }

    ParseResult(
        input.lines().next().unwrap().chars().count() as i32,
        input.lines().count() as i32,
        antennas,
    )
}

fn solve(input: &str) -> (usize, usize) {
    let ParseResult(width, height, antennas) = parse(input);

    let mut antinodes_part1 = HashSet::<Point<i32>>::new();

    let mut antinodes_part2 = HashSet::<Point<i32>>::new();

    for (_, towers) in antennas {
        for (&tower1, &tower2) in towers.iter().tuple_combinations() {
            for antinode in get_antinodes_part1(width, height, tower1, tower2) {
                antinodes_part1.insert(antinode);
            }

            for antinode in get_antinodes_part2(width, height, tower1, tower2) {
                antinodes_part2.insert(antinode);
            }
        }
    }
    (antinodes_part1.len(), antinodes_part2.len())
}

fn get_antinodes_part1(
    width: i32,
    height: i32,
    tower1: Point<i32>,
    tower2: Point<i32>,
) -> Vec<Point<i32>> {
    let distance = tower1 - tower2;

    let antinodes = [tower1 + distance, tower2 - distance];

    antinodes
        .into_iter()
        .filter(|antinode| (0..width).contains(&antinode.x) && (0..height).contains(&antinode.y))
        .collect_vec()
}

fn get_antinodes_part2(
    width: i32,
    height: i32,
    tower1: Point<i32>,
    tower2: Point<i32>,
) -> Vec<Point<i32>> {
    let mut antinodes = Vec::new();

    for d in [tower1 - tower2, tower2 - tower1] {
        let mut pos = tower1;
        while (0..width).contains(&pos.x) && (0..height).contains(&pos.y) {
            antinodes.push(pos);
            pos = pos + d;
        }
    }

    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./example.txt");
        let output = solve(input);
        assert_eq!(output.0, 14);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./example.txt");
        let output = solve(input);
        assert_eq!(output.1, 34);
    }
}
