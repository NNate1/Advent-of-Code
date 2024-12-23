use rusttype::Vector;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

fn main() {
    let input = include_str!("./input.txt");

    let (memory, initial_bytes, bytes) = parse(input, 71, 1024);

    let output = solve(memory, initial_bytes, &bytes);
    println!("part1: {}", output.0);
    println!("part2: {:?}", output.1);
}

fn parse(
    input: &str,
    bounds: usize,
    initial_bytes: usize,
) -> (Vec<Vec<char>>, usize, Vec<Vector<i32>>) {
    let mut memory = vec![vec!['.'; bounds]; bounds];

    let mut bytes = Vec::new();

    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        bytes.push(Vector {
            y: y.parse().unwrap(),
            x: x.parse().unwrap(),
        });
    }

    for byte in bytes.iter().take(initial_bytes) {
        memory[byte.y as usize][byte.x as usize] = '#';
    }

    // print_memory(&memory);
    (memory, initial_bytes, bytes)
}

#[allow(dead_code)]
fn print_memory(memory: &[Vec<char>]) {
    for line in memory.iter() {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn solve(
    mut maze: Vec<Vec<char>>,
    initial_bytes: usize,
    bytes: &[Vector<i32>],
) -> (usize, (usize, usize)) {
    let (distance, mut path) = bfs(&maze);
    let part1 = distance.unwrap();

    for byte in bytes.iter().skip(initial_bytes) {
        maze[byte.y as usize][byte.x as usize] = '#';

        if path.contains(byte) {
            let (d, p) = bfs(&maze);
            if d.is_none() {
                assert!(p.is_empty());
                return (part1, (byte.x as usize, byte.y as usize));
            }
            path = p;
        }
    }

    panic!();
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    score: usize,
    position: Vector<i32>,
    direction: Vector<i32>,
    previous: Option<Rc<State>>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(maze: &[Vec<char>]) -> (Option<usize>, Vec<Vector<i32>>) {
    let goal = Vector {
        x: maze[0].len() as i32 - 1,

        y: maze.len() as i32 - 1,
    };

    let mut heap = BinaryHeap::new();
    heap.push(Rc::new(State {
        score: 0,
        position: Vector { x: 0, y: 0 },
        direction: Vector { x: 1, y: 0 },
        previous: None,
    }));

    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut best_score = None;
    let mut end_node = None;

    while let Some(state) = heap.pop() {
        let score = state.score;
        let position = state.position;
        let direction = state.direction;

        if position == goal {
            best_score = Some(score.min(best_score.unwrap_or(usize::MAX)));
            end_node = Some(state);
            break;
        }

        if visited[position.y as usize][position.x as usize] {
            continue;
        }

        visited[position.y as usize][position.x as usize] = true;

        for dir in DIRECTIONS {
            let neighbour = position + dir;

            if (0..maze.len() as i32).contains(&neighbour.y)
                && (0..maze[0].len() as i32).contains(&neighbour.x)
                && maze[neighbour.y as usize][neighbour.x as usize] != '#'
                && !visited[neighbour.y as usize][neighbour.x as usize]
            {
                let next = State {
                    score: score + 1,
                    position: neighbour,
                    direction,
                    previous: Some(state.clone()),
                };
                heap.push(Rc::new(next));
            }
        }
    }

    let mut path = Vec::<Vector<i32>>::new();

    if let Some(current) = end_node {
        path.push(current.position);

        let mut current = &current;
        while let Some(node) = &current.previous {
            path.push(node.position);
            current = node;
        }
    }

    (best_score, path)
}

const DIRECTIONS: [Vector<i32>; 4] = [
    Vector { x: 1, y: 0 },
    Vector { y: 1, x: 0 },
    Vector { x: -1, y: 0 },
    Vector { y: -1, x: 0 },
];

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");

        let (memory, initial_bytes, bytes) = parse(input, 7, 12);
        let output = solve(memory, initial_bytes, &bytes);
        assert_eq!(output.0, 22);
    }

    #[test]
    fn test_part2_ex1() {
        let input = include_str!("./example1.txt");

        let (memory, initial_bytes, bytes) = parse(input, 7, 12);
        let output = solve(memory, initial_bytes, &bytes);
        assert_eq!(output.1, (6, 1));
    }
}
