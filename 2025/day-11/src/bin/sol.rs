use std::collections::HashMap;

const START_PART_1: &str = "you";
const END: &str = "out";

const START_PART_2: &str = "svr";
const CHECKPOINT_1: &str = "dac";
const CHECKPOINT_2: &str = "fft";

fn main() {
    let input = include_str!("input.txt");
    let adjacency_matrix = parse_input(input);

    println!("part 1: {}", part1(&adjacency_matrix));
    println!("part 2: {}", part2(&adjacency_matrix));
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (source, destinations) = line.split_once(':').unwrap();
            (source, destinations.split_whitespace().collect())
        })
        .collect()
}

fn part1(adjacency_matrix: &HashMap<&str, Vec<&str>>) -> u64 {
    dfs_number_of_paths(adjacency_matrix, END, START_PART_1, &mut HashMap::new())
}

fn part2(adjacency_matrix: &HashMap<&str, Vec<&str>>) -> u64 {
    let path_1 = dfs_number_of_paths(
        adjacency_matrix,
        CHECKPOINT_1,
        START_PART_2,
        &mut HashMap::new(),
    ) * dfs_number_of_paths(
        adjacency_matrix,
        CHECKPOINT_2,
        CHECKPOINT_1,
        &mut HashMap::new(),
    ) * dfs_number_of_paths(adjacency_matrix, END, CHECKPOINT_2, &mut HashMap::new());

    let path_2 = dfs_number_of_paths(
        adjacency_matrix,
        CHECKPOINT_2,
        START_PART_2,
        &mut HashMap::new(),
    ) * dfs_number_of_paths(
        adjacency_matrix,
        CHECKPOINT_1,
        CHECKPOINT_2,
        &mut HashMap::new(),
    ) * dfs_number_of_paths(adjacency_matrix, END, CHECKPOINT_1, &mut HashMap::new());

    path_1 + path_2
}

fn dfs_number_of_paths<'a>(
    adjacency_matrix: &HashMap<&str, Vec<&'a str>>,
    end: &str,
    node: &'a str,
    paths: &mut HashMap<&'a str, u64>,
) -> u64 {
    if node == end {
        return 1;
    }

    if let Some(&number_of_paths) = paths.get(node) {
        return number_of_paths;
    }

    let mut number_of_paths = 0;

    if let Some(edges) = adjacency_matrix.get(node) {
        for &edge in edges {
            number_of_paths += dfs_number_of_paths(adjacency_matrix, end, edge, paths);
        }
        paths.insert(node, number_of_paths);
        number_of_paths
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("example1.txt");
        let adjacency_matrix = parse_input(input);
        assert_eq!(5, part1(&adjacency_matrix));
    }

    #[test]
    fn part2_test() {
        let input = include_str!("example2.txt");
        let adjacency_matrix = parse_input(input);
        assert_eq!(2, part2(&adjacency_matrix));
    }
}
