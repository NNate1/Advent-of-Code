use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");

    let stones = parse(input);

    let output = (part1(stones.clone()), part2(stones));

    println!("part1: {}\npart2: {}", output.0, output.1);
}

fn parse(input: &str) -> Vec<u128> {
    input
        .split_whitespace()
        .map(|stone| {
            stone
                .parse()
                .unwrap_or_else(|_| panic!("Error parsing \'{stone}\' to u128"))
        })
        .collect()
}

fn part1(initial_stones: Vec<u128>) -> usize {
    // solve_simulate(initial_stones, BLINK_COUNT_PART_1)
    solve_optimized(initial_stones, BLINK_COUNT_PART_1)
}

fn part2(initial_stones: Vec<u128>) -> usize {
    // solve(initial_stones, BLINK_COUNT_PART_2)
    solve_optimized(initial_stones, BLINK_COUNT_PART_2)
}
const BLINK_COUNT_PART_1: usize = 25;

const BLINK_COUNT_PART_2: usize = 75;

fn digits(num: u128) -> Option<(u128, u128)> {
    let num_as_string = num.to_string();

    if num_as_string.len() % 2 == 0 {
        let left: u128 = num_as_string[0..num_as_string.len() / 2].parse().unwrap();
        let right: u128 = num_as_string[num_as_string.len() / 2..].parse().unwrap();

        Some((left, right))
    } else {
        None
    }
}

#[allow(dead_code)]
// Executes the complete algorithm using a vector
fn solve_simulate(initial_stones: Vec<u128>, blinks: usize) -> usize {
    let mut stones = (
        initial_stones.clone(),
        Vec::with_capacity(initial_stones.len()),
    );

    let mut seen = HashSet::new();

    for iter in 0..blinks {
        let (prev, new) = if iter % 2 == 0 {
            (&mut stones.0, &mut stones.1)
        } else {
            (&mut stones.1, &mut stones.0)
        };

        for &stone in prev.iter() {
            seen.insert(stone);

            let (new_stone, extra) = blink(stone);
            new.push(new_stone);
            if let Some(value) = extra {
                new.push(value);
            }
        }
        prev.clear();
    }

    if blinks % 2 == 0 { stones.0 } else { stones.1 }.len()
}

// Stores the number of each stone using a hashtable
// Executes the blink operation once per iteration for each unique stone
fn solve_optimized(initial_stones: Vec<u128>, total_blinks: usize) -> usize {
    let mut stones = HashMap::<(u128, usize), usize>::new();

    for stone in initial_stones {
        *stones.entry((stone, 0)).or_default() += 1;
    }

    let mut seen = HashSet::new();

    for _ in 0..total_blinks {
        let mut new_stones = HashMap::<(u128, usize), usize>::new();

        for ((stone, blinks), count) in stones {
            seen.insert(stone);

            let (new_stone, extra) = blink(stone);
            *new_stones.entry((new_stone, blinks + 1)).or_default() += count;
            if let Some(value) = extra {
                *new_stones.entry((value, blinks + 1)).or_default() += count;
            }
        }

        stones = new_stones;
    }

    stones.values().sum()
}

fn blink(stone: u128) -> (u128, Option<u128>) {
    match (stone, digits(stone)) {
        (0, _) => (1, None),
        (_, Some((left, right))) => (left, Some(right)),
        (_, _) => (2024 * stone, None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");
        let output = part1(parse(input));
        assert_eq!(output, 55312);
    }
}
