use itertools::Itertools;
use std::collections::HashMap;
use vector3::{self, Vector3};

const LARGEST_GRAPHS: usize = 3;
const INPUT_CONNECTIONS: usize = 1000;

#[allow(dead_code)]
const EXAMPLE_CONNECTIONS: usize = 10;

fn main() {
    let input = include_str!("input.txt");
    let junction_boxes = parse_input(input);
    let solution = solve_day(&junction_boxes, INPUT_CONNECTIONS);
    println!("part 1 : {}", solution.0);
    println!("part 2 : {}", solution.1);
}

fn parse_input(input: &str) -> Vec<Vector3> {
    input
        .lines()
        .filter_map(|line| {
            line.split(',')
                .filter_map(|coordinate| coordinate.parse().ok())
                .collect_tuple::<(f64, f64, f64)>()
                .map(Vector3::from)
        })
        .collect()
}

fn create_adjacency_matrix(junction_boxes: &[Vector3]) -> Vec<Vec<f64>> {
    let n = junction_boxes.len();
    let mut distances = vec![vec![0.0; n]; n];

    junction_boxes
        .iter()
        .enumerate()
        .cartesian_product(junction_boxes.iter().enumerate())
        .filter(|((i, _), (j, _))| i > j)
        .for_each(|((i, &pos_i), (j, &pos_j))| {
            let dist = pos_i.distance_to(pos_j).abs();
            distances[i][j] = dist;
            distances[j][i] = dist;
        });

    distances
}

fn solve_day(junction_boxes: &[Vector3], connection_checkpoint: usize) -> (usize, usize) {
    let adjacency_matrix = create_adjacency_matrix(junction_boxes);

    let sorted_edges = adjacency_matrix
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(move |&(j, _)| i < j)
                .map(move |(j, &distance)| (i, j, distance))
        })
        .sorted_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap())
        .collect_vec();

    kruskal_minimum_spanning_tree(
        junction_boxes.len(),
        junction_boxes,
        &sorted_edges,
        connection_checkpoint,
    )
}

fn kruskal_minimum_spanning_tree(
    num_nodes: usize,
    junction_boxes: &[Vector3],
    sorted_edges: &[(usize, usize, f64)],
    connection_checkpoint: usize,
) -> (usize, usize) {
    let mut union_find = UnionFind::new(num_nodes);

    let mut edges_added_counter = 0;

    let mut part1_sol = 0;
    for (connection_idx, &(i, j, _distance)) in sorted_edges.iter().enumerate() {
        if connection_idx == connection_checkpoint {
            part1_sol = top_tree_sizes(&mut union_find, LARGEST_GRAPHS);
        }

        if union_find.union(i, j) {
            edges_added_counter += 1;

            if edges_added_counter == num_nodes - 1 {
                let part2_sol = (junction_boxes[i].x * junction_boxes[j].x) as usize;
                return (part1_sol, part2_sol);
            }
        }
    }

    panic!();
}

fn top_tree_sizes(union_find: &mut UnionFind, number_of_trees: usize) -> usize {
    let mut set_sizes = HashMap::<usize, usize>::new();
    (0..union_find.len()).for_each(|node| {
        let root = union_find.find(node);
        *set_sizes.entry(root).or_default() = union_find.size_of(root)
    });

    set_sizes
        .values()
        .sorted()
        .rev()
        .take(number_of_trees)
        .copied()
        .reduce(|acc, element| acc * element)
        .unwrap_or(0)
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    pub fn find(&mut self, node: usize) -> usize {
        if self.parent[node] == node {
            node
        } else {
            let root = self.find(self.parent[node]);
            self.parent[node] = root;
            root
        }
    }

    pub fn union(&mut self, left_node: usize, right_node: usize) -> bool {
        let root_left = self.find(left_node);
        let root_right = self.find(right_node);

        if root_left != root_right {
            // Union by Size: Attach the smaller tree to the root of the larger one.
            if self.size[root_left] < self.size[root_right] {
                self.parent[root_left] = root_right;
                self.size[root_right] += self.size[root_left];
            } else {
                self.parent[root_right] = root_left;
                self.size[root_left] += self.size[root_right];
            }
            true
        } else {
            false
        }
    }

    pub fn size_of(&mut self, node: usize) -> usize {
        let root = self.find(node);
        self.size[root]
    }

    pub fn len(&self) -> usize {
        self.parent.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// Naive union find implementation
#[allow(dead_code)]
fn find(ranks: &mut [usize], node: usize) -> usize {
    if ranks[node] == node {
        node
    } else {
        find(ranks, ranks[node])
    }
}

#[allow(dead_code)]
fn union(ranks: &mut [usize], left_node: usize, right_node: usize) {
    let left_rank = find(ranks, left_node);
    let right_rank = find(ranks, right_node);
    ranks[left_rank] = ranks[right_rank];
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("example.txt");
        let junction_boxes = parse_input(input);

        let (solution_part1, _) = solve_day(&junction_boxes, EXAMPLE_CONNECTIONS);
        assert_eq!(40, solution_part1);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("example.txt");
        let junction_boxes = parse_input(input);

        let (_, solution_part2) = solve_day(&junction_boxes, EXAMPLE_CONNECTIONS);
        assert_eq!(25272, solution_part2);
    }
}
