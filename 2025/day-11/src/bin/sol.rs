use std::collections::HashMap;

const START: &str = "you";
const END: &str = "out";

fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("example.txt");
    let adjacency_matrix = parse_input(input);

    println!("part 1: {}", part1(&adjacency_matrix));
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

fn part1(adjacency_matrix: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut paths = HashMap::new();

    paths.insert(END, 1);
    dfs(adjacency_matrix, START, &mut paths);

    *paths.get(START).expect("Uh oh, no path found")
}

fn dfs<'a>(
    adjacency_matrix: &HashMap<&str, Vec<&'a str>>,
    node: &'a str,
    paths: &mut HashMap<&'a str, u32>,
) -> u32 {
    if let Some(&number_of_paths) = paths.get(node) {
        return number_of_paths;
    }

    let mut number_of_paths = 0;
    for &edge in adjacency_matrix.get(node).unwrap() {
        number_of_paths += dfs(adjacency_matrix, edge, paths);
    }
    // paths.insert(String::from(node).as_ref(), number_of_paths);
    paths.insert(node, number_of_paths);
    number_of_paths
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("example.txt");
        let adjacency_matrix = parse_input(input);
        assert_eq!(5, part1(&adjacency_matrix));
    }
}
