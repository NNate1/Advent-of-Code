use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Guard {
    x: i32,
    y: i32,
    dir: Dir,
}

impl Guard {
    fn walk(&mut self) {
        self.x += self.dir.x;
        self.y += self.dir.y;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Dir {
    x: i32,
    y: i32,
}

impl Dir {
    fn next(&mut self) {
        match (self.y, self.x) {
            (-1, 0) => {
                self.y = 0;
                self.x = 1
            }
            (0, 1) => {
                self.y = 1;
                self.x = 0
            }
            (1, 0) => {
                self.y = 0;
                self.x = -1
            }
            (0, -1) => {
                self.y = -1;
                self.x = 0
            }
            _ => panic!("Uh oh {self:?}"),
        }
    }
}

fn parse(input: &str) -> (i32, i32, Vec<Vec<char>>, Guard) {
    let mut guard = Guard {
        dir: Dir { x: 0, y: -1 },
        x: 0,
        y: 0,
    };

    let mut board = Vec::new();
    for (y, line) in input.lines().enumerate() {
        board.push(
            line.char_indices()
                .map(|(x, c)| {
                    if c == '^' {
                        guard.x = x as i32;
                        guard.y = y as i32;
                    }
                    c
                })
                .collect_vec(),
        );
    }

    (board[0].len() as i32, board.len() as i32, board, guard)
}

fn part2(input: &str) -> u32 {
    let (width, height, mut board, mut guard) = parse(input);

    let mut sol = 0;
    println!("Got input");

    let mut attempts = vec![false; (height * width) as usize];
    loop {
        let next = (guard.x + guard.dir.x, guard.y + guard.dir.y);

        if !(0..width).contains(&next.0) || !(0..height).contains(&next.1) {
            break;
        }

        if board[next.1 as usize][next.0 as usize] == '#' {
            guard.dir.next();
        } else {
            if !attempts[(next.0 + next.1 * width) as usize] {
                attempts[(next.0 + next.1 * width) as usize] = true;
                board[next.1 as usize][next.0 as usize] = '#';
                sol += if has_loop(width, height, &board, guard) {
                    1
                } else {
                    0
                };

                board[next.1 as usize][next.0 as usize] = '.';
            }
            guard.walk()
        }
    }

    sol
}

fn has_loop(width: i32, height: i32, board: &[Vec<char>], initial_guard: Guard) -> bool {
    let mut iterations = 0;
    let mut new_visited = vec![false; (height * width * 4) as usize];

    let mut guard = initial_guard;
    loop {
        let d = match (guard.dir.x, guard.dir.y) {
            (0, 1) => 0,
            (0, -1) => 1,
            (-1, 0) => 2,
            (1, 0) => 3,
            _ => panic!(),
        };

        if new_visited[((guard.x + guard.y * width) * 4 + d) as usize] {
            break true;
        }
        new_visited[((guard.x + guard.y * width) * 4 + d) as usize] = true;

        iterations += 1;
        if iterations >= width * height * 4 {
            panic!();
            // break true;
        };

        let next = (guard.x + guard.dir.x, guard.y + guard.dir.y);

        if !(0..width).contains(&next.0) || !(0..height).contains(&next.1) {
            break false;
        }

        if board[next.1 as usize][next.0 as usize] == '#' {
            guard.dir.next();
        } else {
            guard.walk()
        }
    }
}

// TEST
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = include_str!("./example1.txt");
        let output = part2(input);
        assert_eq!(output, 6);
    }
}
