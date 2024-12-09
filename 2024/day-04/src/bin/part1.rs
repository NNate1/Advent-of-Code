fn main() {
    let input = include_str!("./input.txt");
    let output = part1(parse(input));
    dbg!(output);
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

const WORD: &str = "XMAS";

fn part1(lines: Vec<&str>) -> i32 {
    let mut count = 0;

    for (y, line) in lines.iter().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter == 'X' {
                'dir: for dir in DIRECTIONS {
                    if (0..lines.len() as i32).contains(&(dir.0 * 3 + y as i32))
                        && (0..line.len() as i32).contains(&(dir.1 * 3 + x as i32))
                    {
                        for i in 1..WORD.len() {
                            if lines[(dir.0 * i as i32 + y as i32) as usize]
                                .chars()
                                .nth((dir.1 * i as i32 + x as i32) as usize)
                                .unwrap()
                                != WORD.chars().nth(i).unwrap()
                            {
                                continue 'dir;
                            }
                        }
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

// TEST

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1() {
        let input = include_str!("./example1.txt");
        let output = part1(parse(input));
        assert_eq!(output, 18);
    }

    #[test]
    fn test_ex2() {
        let input = include_str!("./example2.txt");
        let output = part1(parse(input));
        assert_eq!(output, 18);
    }

    #[test]
    fn horizontal() {
        let input = "......XMASSAMXMS...";
        let output = part1(parse(input));
        assert_eq!(output, 2);
    }

    #[test]
    fn vertical() {
        let input = "X
M
A
S
A
M
X";
        let output = part1(parse(input));
        assert_eq!(output, 2);
    }
}
