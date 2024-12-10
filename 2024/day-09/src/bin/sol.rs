use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input);

    println!("part1: {}\npart2: {}", output.0, output.1);
}

fn parse(input: &str) -> Vec<i64> {
    let mut disk = Vec::<i64>::new();

    for (i, c) in input.trim().chars().enumerate() {
        let byte = if i % 2 == 0 { (i / 2) as i64 } else { -1 };

        disk.append(&mut vec![
            byte;
            c.to_digit(10).unwrap_or_else(|| panic!(r" Help: |{c}|"))
                as usize
        ]);
    }

    disk
}

fn solve(input: &str) -> (i64, i64) {
    let disk = parse(input);

    (part1(disk.clone()), part2(disk))
}

fn checksum(disk: &[i64]) -> i64 {
    disk.iter()
        .enumerate()
        .filter(|(_, x)| **x != -1)
        .map(|(i, &x)| x * i as i64)
        .sum()
}

fn part1(mut disk: Vec<i64>) -> i64 {
    let mut left = 0;
    let mut right = disk.len() - 1;

    loop {
        while disk[left] != -1 && left < right {
            left += 1
        }

        while disk[right] == -1 && left < right {
            right -= 1
        }

        if left >= right {
            break;
        }

        disk.swap(left, right);

        left += 1;
        right -= 1;
    }

    checksum(&disk)
}

fn part2(mut disk: Vec<i64>) -> i64 {
    let left = 0;
    let mut right = disk.len() - 1;

    while disk[right] == -1 && left < right {
        right -= 1
    }

    let mut value = disk[right];

    loop {
        while disk[right] != value && left < right {
            right -= 1
        }

        let end = right;

        while disk[right] == value && left < right {
            right -= 1
        }

        if left >= right {
            break;
        }

        let start = right + 1;

        for (idx, window) in disk
            .windows(end - start + 1)
            .enumerate()
            .take(start - (end - start))
        {
            if window.iter().all(|&x| x == -1) {
                for x in disk.iter_mut().skip(idx).take(end - start + 1) {
                    *x = value;
                }

                for x in disk.iter_mut().skip(start).take(end - start + 1) {
                    *x = -1;
                }

                break;
            }
        }

        if left >= right || value == 0 {
            break;
        }
        value -= 1;
    }

    checksum(&disk)
}

#[allow(dead_code)]
fn print_disk(disk: &[i64]) {
    println!(
        "Rearraged:\n{}\n",
        disk.iter()
            .map(|&x| {
                if x == -1 {
                    String::from(".")
                } else {
                    x.to_string()
                }
            })
            .join("")
    );
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./example.txt");
        let output = solve(input);
        assert_eq!(output.0, 1928);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./example.txt");
        let output = solve(input);
        assert_eq!(output.1, 2858);
    }

    #[test]
    fn test_edge1() {
        let input = include_str!("./edge_case1.txt");
        let output = solve(input);
        assert_eq!(output.1, 169);
    }

    #[test]
    fn test_edge2() {
        let input = include_str!("./edge_case2.txt");
        let output = solve(input);
        assert_eq!(output.1, 6204);
    }
}
