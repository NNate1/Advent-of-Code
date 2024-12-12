use rusttype::{Point, Vector};

fn main() {
    let input = include_str!("./input.txt");
    let output = solve(&parse(input));

    println!("part1: {}\npart2: {}", output.0, output.1);
}

const DIRECTIONS: [Vector<i32>; 4] = [
    Vector { x: 1, y: 0 },
    Vector { x: 0, y: 1 },
    Vector { x: -1, y: 0 },
    Vector { x: 0, y: -1 },
];

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut map = Vec::<Vec<i32>>::new();

    for line in input.lines() {
        map.push(
            line.chars()
                .map(|c| {
                    if c == '.' {
                        -1
                    } else {
                        c.to_digit(10).unwrap() as i32
                        // .unwrap_or_else(|| panic!("Error parsing char \'{c}\' to digit."))
                    }
                })
                .collect(),
        )
    }
    map
}

fn solve(map: &Vec<Vec<i32>>) -> (usize, i32) {
    let mut score = 0;
    let mut rating = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, &height) in row.iter().enumerate().filter(|(_, &height)| height == 0) {
            let visited = find_trailheads(map, height, x as i32, y as i32);

            score += get_score(map, &visited);

            rating += get_rating(map, &visited);
        }
    }

    (score, rating)
}

fn find_trailheads(map: &Vec<Vec<i32>>, height: i32, x: i32, y: i32) -> Vec<Vec<i32>> {
    let mut visited = vec![vec![0; map[0].len()]; map.len()];

    dfs(map, Point { x, y }, height, &mut visited);

    visited
}

fn get_score(map: &[Vec<i32>], visited: &[Vec<i32>]) -> usize {
    visited
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, &reached)| reached != 0 && map[y][*x] == 9)
                .count()
        })
        .sum()
}

fn get_rating(map: &[Vec<i32>], visited: &[Vec<i32>]) -> i32 {
    visited
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, &reached)| reached != 0 && map[y][*x] == 9)
                .map(|(_, &reached)| reached)
                .sum::<i32>()
        })
        .sum()
}

fn dfs(map: &Vec<Vec<i32>>, position: Point<i32>, height: i32, visited: &mut Vec<Vec<i32>>) {
    visited[position.y as usize][position.x as usize] += 1;

    for dir in DIRECTIONS {
        let next = position + dir;

        if (0..map.len()).contains(&(next.y as usize))
            && (0..map[0].len()).contains(&(next.x as usize))
            && map[next.y as usize][next.x as usize] == height + 1
        {
            dfs(map, next, map[next.y as usize][next.x as usize], visited)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 2);
    }

    #[test]
    fn test_part1_ex2() {
        let input = include_str!("./example2.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 4);
    }
    #[test]
    fn test_part1_ex3() {
        let input = include_str!("./example3.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 3);
    }
    #[test]
    fn test_part1_ex4() {
        let input = include_str!("./example4.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 36);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./example4.txt");
        let output = solve(&parse(input));
        assert_eq!(output.1, 81);
    }
}
