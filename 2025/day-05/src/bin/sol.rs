use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let (mut fresh_ranges, ingredients) = parse_input(input);
    println!("part 1: {}", part1(&fresh_ranges, &ingredients));
    println!("part 2: {}", part2(&mut fresh_ranges));
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let (fresh_ranges, ingredients) = input.split_once("\n\n").unwrap();

    (
        fresh_ranges
            .lines()
            .map(|line| {
                let (left, right) = line.split_once("-").unwrap();
                (left.parse().unwrap(), right.parse().unwrap())
            })
            .collect(),
        ingredients
            .lines()
            .map(|line| line.parse().unwrap())
            .collect(),
    )
}

fn is_fresh(fresh_ranges: &[(i64, i64)], ingredient: i64) -> bool {
    fresh_ranges
        .iter()
        .any(|&(left, right)| left <= ingredient && ingredient <= right)
}

fn part1(fresh_ranges: &[(i64, i64)], ingredients: &[i64]) -> usize {
    ingredients
        .iter()
        .filter(|&&ingredient| is_fresh(fresh_ranges, ingredient))
        .count()
}

#[allow(dead_code)]
fn part2_naive_core_dump_out_of_memory(fresh_ranges: &[(i64, i64)]) -> usize {
    let mut fresh_ingredients: HashSet<i64> = HashSet::new();

    fresh_ranges.iter().for_each(|&(left, right)| {
        fresh_ingredients.extend((left..=right).collect::<HashSet<i64>>())
    });

    fresh_ingredients.len()
}

fn part2(fresh_ranges: &mut [(i64, i64)]) -> i64 {
    fresh_ranges.sort_by_key(|&(left, _)| left);

    let mut prev_right = 0;

    let mut counter = 0;

    for &(left, right) in fresh_ranges.iter() {
        counter += 0.max(1 + right - left.max(prev_right + 1));
        prev_right = right.max(prev_right);
    }

    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("example.txt");
        let (fresh_ranges, ingredients) = parse_input(input);
        assert_eq!(3, part1(&fresh_ranges, &ingredients));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("example.txt");
        let (mut fresh_ranges, _) = parse_input(input);
        assert_eq!(14, part2(&mut fresh_ranges));
    }
}
