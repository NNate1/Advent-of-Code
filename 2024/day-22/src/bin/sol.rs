use core::panic;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    let secrets = parse(input);

    println!("part1: {}", part1(&secrets));

    println!("part2: {:?}", part2(&secrets));
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|n| n.parse().unwrap_or_else(|_| panic!("Uh oh \"{}\"", n)))
        .collect()
}

const PRUNE: i64 = 16777216;

const FIRST: i64 = 64;
const SECOND: i64 = 32;
const THIRD: i64 = 2048;

fn evolve(mut secret: i64, iter: usize) -> i64 {
    for _ in 1..=iter {
        secret = (secret * FIRST) ^ secret;
        secret %= PRUNE;

        secret = (secret / SECOND) ^ secret;
        secret %= PRUNE;

        secret = (secret * THIRD) ^ secret;
        secret %= PRUNE;
    }
    secret
}

fn part1(secrets: &[i64]) -> i64 {
    secrets
        .iter()
        .map(|&secret| evolve(secret, 2000))
        // .inspect(|secret| println!("{}", secret))
        .sum()
}

fn get_prices(mut secret: i64, iter: usize) -> Vec<(i64, Option<i64>)> {
    let mut prices = Vec::new();
    let mut prev = None;
    for _ in 1..=iter {
        secret = (secret * FIRST) ^ secret;
        secret %= PRUNE;

        secret = (secret / SECOND) ^ secret;
        secret %= PRUNE;

        secret = (secret * THIRD) ^ secret;
        secret %= PRUNE;

        let current_price = secret % 10;

        if let Some(prev_price) = prev {
            prices.push((current_price, Some(current_price - prev_price)))
        } else {
            prices.push((current_price, None));
        }
        prev = Some(current_price);
    }
    prices
}

fn scan_prices(prices: Vec<(i64, Option<i64>)>, possible: &mut HashMap<[i64; 4], i64>) {
    let mut seen = HashSet::<[i64; 4]>::new();

    for ((_, a), (_, b), (_, c), (price, d)) in prices
        .iter()
        .filter(|(_, diff)| diff.is_some())
        .map(|(a, diff)| (a, diff.unwrap()))
        .tuple_windows()
    {
        let seq = [a, b, c, d];
        if seen.insert(seq) {
            *possible.entry(seq).or_default() += *price;
        }
    }
}

fn part2(secrets: &[i64]) -> i64 {
    let mut possible = HashMap::new();
    for &secret in secrets {
        let prices = get_prices(secret, 2000);

        scan_prices(prices, &mut possible);
    }

    *possible.iter().max_by_key(|(_, price)| **price).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");
        let output = part1(&parse(input));
        assert_eq!(output, 37327623);
    }

    #[ignore]
    #[test]
    fn test_part2_ex0() {
        let input = include_str!("./example0.txt");
        let output = part2(&parse(input));
        assert_eq!(output, 6)
    }

    // #[ignore]
    #[test]
    fn test_part2_ex2() {
        let input = include_str!("./example2.txt");
        let output = part2(&parse(input));
        assert_eq!(output, 23);
    }
}
