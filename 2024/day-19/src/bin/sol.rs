use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let (towels, targets) = parse(input);

    println!("part1: {}", part1(&towels, &targets));

    println!("part2: {}", part2(&towels, &targets));
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let towels = input.lines().next().unwrap().split(", ").collect();

    let targets = input.lines().skip(2).collect();

    (towels, targets)
}

fn part1(towels: &[&str], targets: &[&str]) -> usize {
    let mut memo = HashMap::new();
    targets
        .iter()
        .filter(|target| solve_count(towels, target, &mut memo) > 0)
        .count()
}

fn solve_count(towels: &[&str], target: &str, memo: &mut HashMap<String, usize>) -> usize {
    if target.is_empty() {
        return 1;
    }

    if let Some(ways) = memo.get(target) {
        return *ways;
    }

    for towel in towels {
        if let Some(suffix) = target.strip_prefix(towel) {
            let ways = solve_count(towels, suffix, memo);
            *memo.entry(target.to_string()).or_default() += ways;
        }
    }

    *memo.get(target).unwrap_or(&0)
}

fn part2(towels: &[&str], targets: &[&str]) -> usize {
    let mut memo = HashMap::new();
    targets
        .iter()
        .map(|target| solve_count(towels, target, &mut memo))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");
        let (towels, targets) = parse(input);
        let output = part1(&towels, &targets);
        assert_eq!(output, 6);
    }

    #[test]
    fn test_part2_ex1() {
        let input = include_str!("./example1.txt");
        let (towels, targets) = parse(input);
        let output = part2(&towels, &targets);
        assert_eq!(output, 16);
    }
}
