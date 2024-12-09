use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
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

#[derive(Debug)]
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

fn parse(input: &str) -> (i32, i32, HashSet<(i32, i32)>, Guard) {
    let mut obstacles = HashSet::<(i32, i32)>::new();

    let mut guard = Guard {
        dir: Dir { x: 0, y: 0 },
        x: 0,
        y: 0,
    };

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    obstacles.insert((x as i32, y as i32));
                }
                '<' | '>' | 'v' | '^' => {
                    guard.x = x as i32;
                    guard.y = y as i32;
                    match c {
                        '>' => guard.dir.x = 1,
                        '<' => guard.dir.x = -1,
                        'v' => guard.dir.y = 1,
                        '^' => guard.dir.y = -1,
                        _ => panic!("Nao sei dar parse {c}"),
                    };
                }
                _ => {}
            };
        }
    }

    (
        input.lines().next().unwrap().len() as i32,
        input.lines().count() as i32,
        obstacles,
        guard,
    )
}

fn part1(input: &str) -> usize {
    let (bx, by, board, mut guard) = parse(input);

    let mut visited = HashSet::<(i32, i32)>::new();

    loop {
        visited.insert((guard.x, guard.y));
        let mut next = (guard.x + guard.dir.x, guard.y + guard.dir.y);

        while board.contains(&next) {
            guard.dir.next();
            next = (guard.x + guard.dir.x, guard.y + guard.dir.y);
        }

        if !(0..bx).contains(&next.0) || !(0..by).contains(&next.1) {
            return visited.len();
        }

        guard.walk();
    }
}

// TEST
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = include_str!("./example1.txt");
        let output = part1(input);
        assert_eq!(output, 41);
    }
}
