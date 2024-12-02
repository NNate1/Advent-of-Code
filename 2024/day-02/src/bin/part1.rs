fn main() {
    let input = include_str!("./input.txt");
    let output = part1_v2(input);
    dbg!(output);
}

#[allow(dead_code)]
fn part1(input: &str) -> String {
    let mut safe_count = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let increasing = levels[1] > levels[0];
        if levels
            .windows(2)
            .all(|w| (w[1] > w[0]) == increasing && (1..=3).contains(&w[0].abs_diff(w[1])))
        {
            safe_count += 1
        }
    }

    safe_count.to_string()
}

fn part1_v2(input: &str) -> usize {
    input.lines().filter(|line| check_safety(line)).count()
}

fn check_safety(line: &str) -> bool {
    let levels: Vec<i32> = line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let increasing = levels[1] > levels[0];
    levels
        .windows(2)
        .all(|w| (w[1] > w[0]) == increasing && (1..=3).contains(&w[0].abs_diff(w[1])))
}

// TEST

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        let input = include_str!("./example.txt");
        let output = part1(input);
        assert_eq!(output, "2");
    }
    #[test]
    fn test_v2() {
        let input = include_str!("./example.txt");
        let output = part1_v2(input);
        assert_eq!(output, 2);
    }
}
