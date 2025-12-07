use std::{
    collections::HashMap,
    ops::{Add, Mul},
};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let (values, operators) = parse_input(input);
    println!("part 1: {}", part1(&values, &operators));

    let (digit_map, _) = parse_input_part2(input);
    println!("part 2: {}", part2(&digit_map, &operators));
}

fn parse_input(input: &str) -> (Vec<Vec<i64>>, Vec<String>) {
    let operators = input
        .lines()
        .last()
        .map(|line| line.split_whitespace().map(str::to_string).collect_vec())
        .unwrap();

    let line_count = input.lines().count();

    let horizontal_values = input
        .lines()
        .take(line_count - 1)
        .map(|line| line.split_whitespace().map(|w| w.parse::<i64>().unwrap()));

    let mut vertical_values = vec![vec![0; line_count - 1]; operators.len()];

    for (row_idx, row) in horizontal_values.enumerate() {
        for (column_idx, value) in row.enumerate() {
            vertical_values[column_idx][row_idx] = value;
        }
    }

    (vertical_values, operators)
}

fn do_operation(values: &[i64], operator: &str) -> i64 {
    values
        .iter()
        .copied()
        .reduce({
            match operator {
                "+" => i64::add,
                "*" => i64::mul,
                _ => panic!(),
            }
        })
        .unwrap()
}

fn part1(values: &[Vec<i64>], operators: &Vec<String>) -> i64 {
    values
        .iter()
        .zip(operators)
        .map(|(values, operator)| do_operation(values, operator))
        .sum()
}

fn parse_input_part2(input: &str) -> (HashMap<(usize, usize), u32>, Vec<String>) {
    let operators = input
        .lines()
        .last()
        .map(|line| line.split_whitespace().map(str::to_string).collect_vec())
        .unwrap();

    let digit_map: HashMap<(usize, usize), u32> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_digit())
                .map(move |(x, c)| ((y, x), c.to_digit(10).unwrap()))
        })
        .collect();

    (digit_map, operators)
}

fn part2(digit_map: &HashMap<(usize, usize), u32>, operators: &[String]) -> i64 {
    let max_row = digit_map.keys().max_by_key(|(y, _)| y).unwrap().0;
    let max_column = digit_map.keys().max_by_key(|(_, x)| x).unwrap().1;

    let mut total = 0;

    let mut numbers = Vec::new();

    let mut operator_iter = operators.iter().rev();
    for x in (0..=max_column).rev() {
        let mut number = 0;

        let mut empty_column = true;

        for y in 0..=max_row {
            if let Some(digit) = digit_map.get(&(y, x)) {
                number = number * 10 + *digit as i64;
                empty_column = false;
            }
        }

        if !empty_column {
            numbers.push(number);
        }

        if empty_column || x == 0 {
            total += do_operation(&numbers, operator_iter.next().unwrap());
            numbers.clear();
        }
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("example.txt");
        let (values, operators) = parse_input(input);
        assert_eq!(4277556, part1(&values, &operators));
    }

    #[test]
    fn part2_test() {
        let input = include_str!("example.txt");
        let (digit_map, operators) = parse_input_part2(input);
        assert_eq!(3263827, part2(&digit_map, &operators));
    }
}
