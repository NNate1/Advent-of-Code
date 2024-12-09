use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
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

        if !pages.iter().enumerate().all(|(i, page)| {
            order
                .get(page)
                .map_or_else(|| true, |x| x.iter().all(|x| !pages[0..i].contains(x)))
        }) {
            sum += correct_order(&order, pages)
        }
    }

    sum
}

pub fn correct_order(order: &HashMap<u32, HashSet<u32>>, sequence: Vec<u32>) -> u32 {
    let mut og_set = HashSet::<&u32>::from_iter(sequence.iter());
    let mut page = 0;

    for _ in 0..=(sequence.len() / 2) {
        let mut first = og_set.clone();
        for page in og_set.iter() {
            if let Some(destinations) = order.get(page) {
                for dest in destinations {
                    first.remove(dest);
                }
            }
        }

        assert_eq!(first.len(), 1);
        page = **first.iter().next().unwrap();

        og_set.remove(&page);
    }
    page
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = include_str!("./example1.txt");
        let output = part2(input);
        assert_eq!(output, 123);
    }
}
