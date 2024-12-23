use rusttype::Vector;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;

fn main() {
    let input = include_str!("./input.txt");

    let maze = parse(input);

    let output = solve(&maze);
    println!("part1: {}", output.0);
    println!("part2: {}", output.1);
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut maze = Vec::new();

    for line in input.lines() {
        maze.push(Vec::from(line))
    }

    maze
}

fn solve(maze: &[Vec<u8>]) -> (usize, usize) {
    let sol = bfs(maze);
    (sol.0.unwrap(), sol.1)
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

fn bfs(maze: &[Vec<u8>]) -> (Option<usize>, usize) {
    let goal = Vector {
        x: maze[0].len() as i32 - 2,
        y: 1,
    };
    let mut heap = BinaryHeap::new();
    heap.push(Rc::new(State {
        score: 0,
        position: Vector {
            x: 1,
            y: maze.len() as i32 - 2,
        },
        direction: Vector { x: 1, y: 0 },
        previous: None,
    }));

    let mut visited = vec![vec![HashMap::<Vector<i32>, usize>::new(); maze[0].len()]; maze.len()];
    let mut best_score = None;
    let mut end_nodes = Vec::new();

    while let Some(state) = heap.pop() {
        let score = state.score;
        let position = state.position;
        let direction = state.direction;

        let previous = &state.previous;

        if let Some(best_score) = best_score {
            if score > best_score {
                continue;
            }
        }

        if let Some(prev_score) = visited[position.y as usize][position.x as usize].get(&direction)
        {
            if *prev_score < score {
                continue;
            }
        }

        if position == goal {
            best_score = Some(score.min(best_score.unwrap_or(usize::MAX)));
            end_nodes.push(state.clone());
            visited[position.y as usize][position.x as usize].insert(direction, score);
            continue;
        }

        visited[position.y as usize][position.x as usize].insert(direction, score);

        for dir in DIRECTIONS {
            let neighbour = position + dir;

            if maze[neighbour.y as usize][neighbour.x as usize] != b'#'
                && visited[neighbour.y as usize][neighbour.x as usize]
                    .get(&dir)
                    .unwrap_or(&usize::MAX)
                    > &score
            {
                let next = if dir == direction {
                    State {
                        score: score + 1,
                        position: neighbour,
                        direction,
                        previous: Some(state.clone()),
                    }
                } else if dir + direction == Vector::default() {
                    continue;
                } else {
                    State {
                        score: score + 1000,
                        position,
                        direction: dir,

                        previous: previous.clone(),
                    }
                };

                heap.push(Rc::new(next));
            }
        }
    }

    let mut visited_nodes = HashSet::<Vector<i32>>::new();

    for node in end_nodes {
        visited_nodes.insert(node.position);
        let mut current = &node;
        while let Some(node) = &current.previous {
            visited_nodes.insert(node.position);
            current = node;
        }
    }
    (best_score, visited_nodes.len())
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

    #[test]
    fn test_part1_ex0() {
        let input = include_str!("./example0.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 6018);
    }

    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 7036);
    }

    #[test]
    fn test_part1_ex2() {
        let input = include_str!("./example2.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 11048);
    }

    #[test]
    fn test_part2_ex1() {
        let input = include_str!("./example1.txt");
        let output = solve(&parse(input));
        assert_eq!(output.1, 45);
    }

    #[test]
    fn test_part2_ex2() {
        let input = include_str!("./example2.txt");
        let output = solve(&parse(input));
        assert_eq!(output.1, 64);
    }
}
