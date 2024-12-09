use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut order = HashMap::<u32, HashSet<u32>>::new();

    let mut iter = input.lines();

    for line in iter.by_ref() {
        let mut values = line.split('|');

        match (values.next(), values.next()) {
            (Some(a), Some(b)) => order
                .entry(a.parse().unwrap())
                .or_default()
                .insert(b.parse().unwrap()),
            (_, _) => break,
        };
    }

    let mut sum = 0;

    for line in iter {
        let pages: Vec<u32> = line.split(',').map(|s| s.parse().unwrap()).collect();

        if pages.iter().enumerate().all(|(i, page)| {
            order
                .get(page)
                .map_or_else(|| true, |x| x.iter().all(|x| !pages[0..i].contains(x)))
        }) {
            sum += pages[pages.len() / 2];
        }
    }

    sum
}

// TEST
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = include_str!("./example1.txt");
        let output = part1(input);
        assert_eq!(output, 143);
    }

    #[test]
    fn test_ex_gui() {
        let input = include_str!("./exgui.txt");
        let output = part1(input);
        assert_eq!(output, 27);
    }
}
