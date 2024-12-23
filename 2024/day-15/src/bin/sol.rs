use core::panic;
use std::fmt::Display;

use rusttype::Vector;

fn main() {
    let input = include_str!("./input.txt");

    let (warehouse, moves, robot) = parse(input);

    println!("part1: {}", part1(warehouse.clone(), &moves, robot));
    println!("part2: {}", part2(&warehouse, &moves, robot));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,
    Wall,
    Box,
    Robot,
    LeftBox,
    RightBox,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => ".",
                Cell::Wall => "#",
                Cell::Box => "O",
                Cell::Robot => "@",
                Cell::LeftBox => "[",
                Cell::RightBox => "]",
            }
        )
    }
}

fn parse(input: &str) -> (Vec<Vec<Cell>>, Vec<Vector<i32>>, Vector<i32>) {
    let mut warehouse = Vec::new();
    let mut moves = Vec::new();

    let mut robot = Vector::default();
    let mut lines = input.lines();

    for (y, line) in lines.by_ref().enumerate() {
        let mut row = Vec::new();
        if line.is_empty() {
            break;
        }

        for (x, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                'O' => Cell::Box,
                '@' => {
                    robot = Vector {
                        x: x as i32,
                        y: y as i32,
                    };
                    Cell::Robot
                }
                _ => panic!("Unexpected character: \'{c}\'"),
            });
        }

        warehouse.push(row);
    }

    for line in lines {
        for c in line.chars() {
            moves.push(match c {
                '>' => Vector { x: 1, y: 0 },
                'v' => Vector { x: 0, y: 1 },
                '<' => Vector { x: -1, y: 0 },
                '^' => Vector { x: 0, y: -1 },
                _ => panic!("Unexpected character: \'{c}\'"),
            });
        }
    }

    (warehouse, moves, robot)
}

fn widen(warehouse: &Vec<Vec<Cell>>, mut robot: Vector<i32>) -> (Vec<Vec<Cell>>, Vector<i32>) {
    let mut wide_warehouse = Vec::new();

    for row in warehouse {
        let mut wide_row = Vec::new();
        for c in row {
            wide_row.extend_from_slice(
                &(match c {
                    Cell::Box => [Cell::LeftBox, Cell::RightBox],
                    Cell::Robot => [Cell::Robot, Cell::Empty],
                    &c => [c; 2],
                }),
            );
        }
        wide_warehouse.push(wide_row);
    }

    robot.x *= 2;
    (wide_warehouse, robot)
}

#[allow(dead_code)]
fn print_warehouse(warehouse: &Vec<Vec<Cell>>) {
    for row in warehouse {
        for c in row {
            print!("{:}", c);
        }
        println!();
    }
}

fn part1(mut warehouse: Vec<Vec<Cell>>, moves: &Vec<Vector<i32>>, robot: Vector<i32>) -> usize {
    push_boxes(&mut warehouse, moves, robot);

    sum_coordinates(&warehouse)
}

fn push_boxes(warehouse: &mut Vec<Vec<Cell>>, moves: &Vec<Vector<i32>>, mut robot: Vector<i32>) {
    for &movement in moves {
        let mut box_count = 0;

        let mut dest = robot;

        loop {
            dest = dest + movement;
            let y = dest.y as usize;

            let x = dest.x as usize;

            match warehouse[y][x] {
                Cell::Box => {
                    box_count += 1;
                    warehouse[y][x] = Cell::Empty
                }
                Cell::Empty => {
                    dest = dest + movement;
                    break;
                }

                _ => break,
            }
        }

        let back = Vector {
            x: -movement.x,
            y: -movement.y,
        };
        for _ in 0..box_count {
            dest = dest + back;

            let y = dest.y as usize;

            let x = dest.x as usize;
            warehouse[y][x] = Cell::Box;
        }

        let y = robot.y as usize;

        let x = robot.x as usize;
        warehouse[y][x] = Cell::Empty;

        robot = dest + back;

        let y = robot.y as usize;

        let x = robot.x as usize;

        warehouse[y][x] = Cell::Robot;
    }
}
fn sum_coordinates(warehouse: &[Vec<Cell>]) -> usize {
    warehouse
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == Cell::Box || **c == Cell::LeftBox)
                .map(move |(x, _)| 100 * y + x)
        })
        .sum()
}

fn part2(warehouse: &Vec<Vec<Cell>>, moves: &Vec<Vector<i32>>, robot: Vector<i32>) -> usize {
    let (mut wide_warehouse, robot) = widen(warehouse, robot);

    push_wide_boxes(&mut wide_warehouse, moves, robot);

    sum_coordinates(&wide_warehouse)
}

fn push_wide_boxes(
    warehouse: &mut Vec<Vec<Cell>>,
    moves: &Vec<Vector<i32>>,
    mut robot: Vector<i32>,
) {
    for &movement in moves {
        let mut dest = robot + movement;

        // vertical move, recursive checks
        if movement.y != 0 {
            let y = dest.y as usize;

            let x = dest.x as usize;

            match warehouse[y][x] {
                Cell::LeftBox | Cell::RightBox => {
                    if is_pushable(dest, &movement, warehouse) {
                        push_everything(dest, &movement, warehouse);

                        warehouse[y][x] = Cell::Robot;

                        let y = robot.y as usize;
                        let x = robot.x as usize;
                        warehouse[y][x] = Cell::Empty;

                        robot = dest;
                    }
                }
                Cell::Empty => {
                    warehouse[y][x] = Cell::Robot;

                    let y = robot.y as usize;
                    let x = robot.x as usize;
                    warehouse[y][x] = Cell::Empty;

                    robot = dest;
                }
                _ => {}
            }
        }
        // horizontal move
        else {
            let mut box_count = 0;

            loop {
                let y = dest.y as usize;

                let x = dest.x as usize;

                match warehouse[y][x] {
                    Cell::LeftBox => {
                        box_count += 1;
                    }
                    Cell::Empty => {
                        let mut prev = robot + movement;
                        let mut y = prev.y as usize;
                        let mut x = prev.x as usize;

                        let mut prev_box = warehouse[y][x];

                        let mut current;

                        for _ in 0..(2 * box_count) {
                            current = prev + movement;
                            y = current.y as usize;
                            x = current.x as usize;
                            let tmp = warehouse[y][x];
                            warehouse[y][x] = prev_box;

                            prev = current;

                            prev_box = tmp;
                        }

                        let y = robot.y as usize;
                        let x = robot.x as usize;
                        warehouse[y][x] = Cell::Empty;

                        robot = robot + movement;
                        let y = robot.y as usize;
                        let x = robot.x as usize;
                        warehouse[y][x] = Cell::Robot;
                        break;
                    }

                    Cell::Wall => break,
                    _ => {}
                }

                dest = dest + movement;
            }
        }
    }
}

fn is_pushable(mut bruh: Vector<i32>, movement: &Vector<i32>, warehouse: &Vec<Vec<Cell>>) -> bool {
    let y = bruh.y as usize;

    let x = bruh.x as usize;

    if warehouse[y][x] == Cell::RightBox {
        bruh.x -= 1
    }

    let left = bruh + *movement;
    let left_y = left.y as usize;

    let left_x = left.x as usize;

    let right = Vector {
        x: left.x + 1,
        ..left
    };

    let right_y = right.y as usize;

    let right_x = right.x as usize;

    match (warehouse[left_y][left_x], warehouse[right_y][right_x]) {
        (Cell::Empty, Cell::Empty) => true,
        (Cell::Wall, _) | (_, Cell::Wall) => false,
        (Cell::LeftBox, Cell::RightBox) => is_pushable(left, movement, warehouse),

        (Cell::RightBox, Cell::LeftBox) => {
            is_pushable(left, movement, warehouse) && is_pushable(right, movement, warehouse)
        }
        (Cell::RightBox, _) => is_pushable(left, movement, warehouse),
        (_, Cell::LeftBox) => is_pushable(right, movement, warehouse),
        _ => panic!(
            "Push box unexpected match {} {}",
            warehouse[left_y][left_x], warehouse[right_y][right_x]
        ),
    }
}

fn push_everything(mut bruh: Vector<i32>, movement: &Vector<i32>, warehouse: &mut Vec<Vec<Cell>>) {
    let y = bruh.y as usize;

    let x = bruh.x as usize;

    match warehouse[y][x] {
        Cell::RightBox => bruh.x -= 1,
        Cell::LeftBox => {}
        _ => return,
    }

    let left = bruh + *movement;
    let left_y = left.y as usize;

    let left_x = left.x as usize;

    let right = Vector {
        x: left.x + 1,
        ..left
    };

    let right_y = right.y as usize;

    let right_x = right.x as usize;

    push_everything(left, movement, warehouse);

    push_everything(right, movement, warehouse);

    warehouse[left_y][left_x] = Cell::LeftBox;
    warehouse[right_y][right_x] = Cell::RightBox;

    let left = bruh;
    let left_y = left.y as usize;

    let left_x = left.x as usize;

    let right = Vector {
        x: left.x + 1,
        ..left
    };
    let right_y = right.y as usize;

    let right_x = right.x as usize;

    warehouse[left_y][left_x] = Cell::Empty;
    warehouse[right_y][right_x] = Cell::Empty;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1_ex1() {
        let (warehouse, moves, robot) = parse(include_str!("./example1.txt"));
        let output = part1(warehouse, &moves, robot);
        assert_eq!(output, 2028);
    }

    // #[ignore]
    #[test]
    fn test_part1_ex2() {
        let (warehouse, moves, robot) = parse(include_str!("./example2.txt"));
        let output = part1(warehouse, &moves, robot);
        assert_eq!(output, 10092);
    }

    // #[ignore]
    #[test]
    fn test_part2_ex2() {
        let (warehouse, moves, robot) = parse(include_str!("./example3.txt"));
        let output = part2(&warehouse, &moves, robot);
        assert_eq!(output, 618);
    }

    // #[ignore]
    #[test]
    fn test_part2_ex3() {
        let (warehouse, moves, robot) = parse(include_str!("./example2.txt"));
        let output = part2(&warehouse, &moves, robot);
        assert_eq!(output, 9021);
    }
}
