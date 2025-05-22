use itertools::Itertools;
use rusttype::Vector;

fn main() {
    // let input = include_str!("./input.txt");
    let input = include_str!("./example2.txt");

    let maze = parse(input);

    println!("part1: {}", part1(&maze));

    println!("part2: {}", part2(&maze));
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut maze = Vec::new();

    for line in input.lines() {
        maze.push(Vec::from(line))
    }

    maze
}

fn solve(maze: &[Vec<u8>], cheat_limit: usize) -> usize {
    let mut start = Vector::<i32>::default();
    let mut end = Vector::<i32>::default();
    for (y, row) in maze.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == b'S' {
                start = Vector {
                    x: x as i32,
                    y: y as i32,
                }
            } else if c == b'E' {
                end = Vector {
                    x: x as i32,
                    y: y as i32,
                }
            }
        }
    }

    let distances = all_distances(maze, end);

    let fastest = distances[start.y as usize][start.x as usize].unwrap();
    println!("Fastest path: {}", fastest);

    let shortcuts = scanner(maze, &distances, cheat_limit);

    let mut part1_sol = 0;

    for (&saved, shortcuts) in shortcuts.iter().sorted_by_key(|(saved, _)| **saved) {
        // println!("{} cheats save {} picoseconds", count, fastest - dist);
        println!("{} cheats save {} picoseconds", shortcuts.len(), saved);

        // if fastest - saved >= 100 {
        if saved >= 100 {
            part1_sol += shortcuts.len();
        }
    }

    println!("Hello: {}", part1_sol);
    shortcuts
        .iter()
        .filter(|(&saved, _)| saved >= 100)
        .map(|(_, shortcut)| shortcut.len())
        .sum()
}

fn part1(maze: &[Vec<u8>]) -> usize {
    solve(maze, 2)
}

fn part2(maze: &[Vec<u8>]) -> usize {
    solve(maze, 20)
}

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
struct State {
    score: usize,
    position: Vector<i32>,
    cheats: usize,
    previous: Option<Rc<State>>,
    cheat_start: Option<Vector<i32>>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
        // .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn all_distances(maze: &[Vec<u8>], goal: Vector<i32>) -> Vec<Vec<Option<i32>>> {
    let mut heap = BinaryHeap::new();
    heap.push(Rc::new(State {
        score: 0,
        position: goal,
        cheats: 0,
        previous: None,

        cheat_start: None,
    }));

    let mut visited = vec![vec![None; maze[0].len()]; maze.len()];

    while let Some(state) = heap.pop() {
        let score = state.score;
        let position = state.position;

        if visited[position.y as usize][position.x as usize].is_some() {
            continue;
        }

        visited[position.y as usize][position.x as usize] = Some(score as i32);

        for dir in DIRECTIONS {
            let neighbour = position + dir;
            if maze[neighbour.y as usize][neighbour.x as usize] != b'#'
                && visited[neighbour.y as usize][neighbour.x as usize].is_none()
            {
                let next = State {
                    score: score + 1,
                    position: neighbour,
                    previous: Some(state.clone()),
                    cheats: 0,
                    cheat_start: None,
                };

                heap.push(Rc::new(next));
            }
        }
    }

    visited
}

fn cheating_bfs(
    maze: &[Vec<u8>],
    start: Vector<i32>,
    cheatless_distances: &[Vec<Option<i32>>],
    cheat_limit: usize,
    shortest_path: usize,
) -> HashMap<i32, HashSet<(Vector<i32>, Vector<i32>)>> {
    let mut heap = BinaryHeap::new();
    heap.push(Rc::new(State {
        score: 0,
        position: start,
        cheats: 0,
        previous: None,
        cheat_start: None,
    }));

    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];

    let mut cheated_distances: HashMap<i32, HashSet<(Vector<i32>, Vector<i32>)>> = HashMap::new();

    while let Some(state) = heap.pop() {
        let score = state.score;
        let position = state.position;
        let mut cheats = state.cheats;
        let cheat_start = state.cheat_start;

        if score >= shortest_path {
            continue;
        }

        if cheats == cheat_limit {
            if let Some(distance) = cheatless_distances[position.y as usize][position.x as usize] {
                let distance = score as i32 + distance;
                println!("Distance: {} Shortest: {}", distance, shortest_path);
                if distance >= shortest_path as i32 {
                    println!("RUA");
                    continue;
                }

                assert!(shortest_path as i32 - distance > 0);
                cheated_distances
                    .entry(score as i32 + distance)
                    .or_default()
                    .insert((cheat_start.unwrap(), position));
            }
            continue;
        }

        if cheats == 0 {
            if visited[position.y as usize][position.x as usize] {
                continue;
            }
            visited[position.y as usize][position.x as usize] = true;
        }

        for dir in DIRECTIONS {
            let neighbour = position + dir;

            if !(0..maze.len() as i32).contains(&neighbour.y)
                || !(0..maze[0].len() as i32).contains(&neighbour.x)
            {
                continue;
            }

            if maze[neighbour.y as usize][neighbour.x as usize] == b'#'
                || maze[position.y as usize][position.x as usize] == b'#'
            {
                if cheats < cheat_limit {
                    let mut next = State {
                        score: score + 1,
                        position: neighbour,
                        previous: Some(state.clone()),
                        cheats: cheats + 1,
                        cheat_start,
                    };

                    if cheats == 1 {
                        next.cheat_start = Some(state.position);
                    }

                    heap.push(Rc::new(next));
                }
            } else if maze[neighbour.y as usize][neighbour.x as usize] != b'#' {
                if cheats > 0 {
                    cheats += 1;
                }
                let next = State {
                    score: score + 1,
                    position: neighbour,
                    previous: Some(state.clone()),
                    cheats,
                    cheat_start,
                };

                heap.push(Rc::new(next));
            }
        }
    }

    cheated_distances
}

fn scanner(
    maze: &[Vec<u8>],
    // start: Vector<i32>,
    cheatless_distances: &[Vec<Option<i32>>],
    cheat_limit: usize,
    // shortest_path: usize,
) -> HashMap<i32, HashSet<(Vector<usize>, Vector<usize>)>> {
    let mut cheated_distances = HashMap::<i32, HashSet<(Vector<usize>, Vector<usize>)>>::new();

    for (start_y, row) in cheatless_distances.iter().enumerate() {
        for (start_x, cell) in row.iter().enumerate() {
            if let Some(initial_distance) = cell {
                for cheat in 0..=cheat_limit {
                    // let cheat = cheat as i32;

                    let ly = start_y.saturating_sub(cheat);
                    let uy = maze.len().min(start_y + cheat);

                    let lx = start_x.saturating_sub(cheat_limit - cheat);
                    let ux = maze[0].len().min(start_x + cheat_limit - cheat);

                    // println!("cheat:{} x: {}..{} y: {}..{}", cheat, lx, ux, ly, uy);

                    // cheat:0 x: 0..3 y: 1..1
                    // cheat:1 x: 0..2 y: 0..2

                    // for y in ly..uy {
                    for (y, row) in cheatless_distances.iter().enumerate().take(uy + 1).skip(ly) {
                        for (x, distance) in row.iter().enumerate().take(ux + 1).skip(lx) {
                            // println!("{:?} -> {:?} : {:?}", (start_x, start_y), (x, y), distance);

                            if x == start_x && y == start_y {
                                continue;
                            }

                            if let Some(final_distance) = distance {
                                let cheat_cost = x.abs_diff(start_x) + y.abs_diff(start_y);
                                let cheated = initial_distance - final_distance - cheat_cost as i32;
                                // println!("Cheated: {}, start {:?}, end {:?}\n initial d {}, final d {}, cheat_cost {}",cheated, (start_x, start_y), (x,y), initial_distance, final_distance, cheat_cost);
                                let start = Vector {
                                    x: start_x,
                                    y: start_y,
                                };
                                let end = Vector { x, y };
                                cheated_distances
                                    .entry(cheated)
                                    .or_default()
                                    .insert((start, end));
                            }
                        }
                    }
                }
            }
        }
    }

    cheated_distances
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
        let output = part1(&parse(input));
        assert_eq!(output, 0);
    }
}
