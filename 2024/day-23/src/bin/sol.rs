use ::std::collections::HashMap;

use ::std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");

    let connections = parse(input);

    println!("part1: {}", part1(&connections));

    println!("part2: {:?}", part2(&connections));
}

fn parse(input: &str) -> HashMap<String, HashSet<String>> {
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();

    for (a, b) in input.lines().filter_map(|line| line.split_once('-')) {
        connections
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());

        connections
            .entry(b.to_string())
            .or_default()
            .insert(a.to_string());
    }

    connections
}

fn part1(connections: &HashMap<String, HashSet<String>>) -> usize {
    let mut counter = 0;

    for (source, destinations) in connections {
        for destination in destinations {
            // counter += destinations.intersection(&connections[destination]).count() - 1;

            let c = destinations.intersection(&connections[destination]);
            if source.starts_with("t") || destination.starts_with("t") {
                counter += c.count()
            } else {
                counter += c.filter(|s| s.starts_with("t")).count();
            }
        }
    }

    counter / 6
}

fn part2(connections: &HashMap<String, HashSet<String>>) -> String {
    let mut lans = Vec::new();
    let mut visited = Vec::new();
    for (source, destinations) in connections {
        visited.push(source.to_string());
        let targets: Vec<String> = destinations.iter().map(|s| s.to_string()).collect();
        dfs(
            connections,
            &vec![source.to_string()],
            &targets,
            &mut visited,
            &mut lans,
        );
    }

    // println!("{:#?}", lans);

    let mut sol = lans.into_iter().max_by_key(|lan| lan.len()).unwrap();
    sol.sort();

    sol.join(",")
}

// struct Node {
//
//
// }

fn dfs(
    connections: &HashMap<String, HashSet<String>>,
    lan: &Vec<String>,
    targets: &Vec<String>,
    visited: &mut Vec<String>,
    lans: &mut Vec<Vec<String>>,
) {
    if targets.is_empty() {
        lans.push(lan.clone());
        return;
    }

    for target in targets {
        let new_targets = targets
            .iter()
            .filter(|&s| connections[target].contains(s))
            .filter(|s| !visited.contains(s))
            .cloned()
            .collect();

        let mut new_lan = lan.clone();
        new_lan.push(target.to_string());
        visited.push(target.to_string());
        dfs(connections, &new_lan, &new_targets, visited, lans)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1_ex1() {
        let input = parse(include_str!("./example1.txt"));
        let output = part1(&input);
        assert_eq!(output, 7);
    }

    // #[ignore]
    #[test]
    fn test_part2_ex1() {
        let input = parse(include_str!("./example1.txt"));
        let output = part2(&input);
        assert_eq!(output, "co,de,ka,ta");
    }
}
