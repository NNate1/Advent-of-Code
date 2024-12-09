fn main() {
    let input = include_str!("./input.txt");
    let output = part2(parse(input));
    dbg!(output);
}

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part2(lines: Vec<&str>) -> i32 {
    let mut count = 0;

    for (y, line) in lines.iter().enumerate().take(lines.len() - 1).skip(1) {
        for (x, letter) in line.chars().enumerate().take(line.len() - 1).skip(1) {
            if letter == 'A' {
                let diag1: [char; 2] = [
                    lines[y - 1].chars().nth(x - 1).unwrap(),
                    lines[y + 1].chars().nth(x + 1).unwrap(),
                ];

                let diag2: [char; 2] = [
                    lines[y - 1].chars().nth(x + 1).unwrap(),
                    lines[y + 1].chars().nth(x - 1).unwrap(),
                ];

                if diag1.contains(&'S')
                    && diag1.contains(&'M')
                    && diag2.contains(&'M')
                    && diag2.contains(&'S')
                {
                    count += 1
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
        let output = part2(parse(input));
        assert_eq!(output, 9);
    }
}
