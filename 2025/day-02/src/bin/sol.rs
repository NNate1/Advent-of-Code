fn main() {
    let input = include_str!("input.txt");
    let ranges = parse_input(input);
    println!("part1: {}", part1(&ranges));
    println!("part1_iter: {}", part1_iter(&ranges));
    println!("part2: {}", part2(&ranges));
    println!("part2_iter: {}", part2_iter(&ranges));
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    let mut ranges = Vec::new();
    for (start, end) in input
        .trim()
        .split(",")
        .map(|range| range.split_once("-").unwrap())
    {
        ranges.push((start.parse().unwrap(), end.parse().unwrap()));
    }
    ranges
}

fn part1(ranges: &[(i64, i64)]) -> i64 {
    let mut count = 0;

    for &(start, end) in ranges {
        for i in start..=end {
            if has_equal_halves(i) {
                count += i;
                // println!("Invalid id detected {i} in range {start}-{end}");
            }
        }
    }
    count
}

fn part1_iter(ranges: &[(i64, i64)]) -> i64 {
    ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&i| has_equal_halves(i))
        .sum()
}

fn has_equal_halves(original_number: i64) -> bool {
    let mut digit_count = 0;
    let mut temp_number = original_number;
    while temp_number != 0 {
        temp_number /= 10;
        digit_count += 1;
    }

    let power_of_ten_half = 10_i64.pow(digit_count / 2);
    let left_half = original_number % power_of_ten_half;
    let right_half = original_number / power_of_ten_half;
    digit_count % 2 == 0 && left_half == right_half
}

fn part2(ranges: &[(i64, i64)]) -> i64 {
    let mut count = 0;

    for &(start, end) in ranges {
        for i in start..=end {
            if is_repeated_sequence(i) {
                count += i;
                // println!("Repeated sequence detected {i} in range {start}-{end}");
            }
        }
    }
    count
}

fn part2_iter(ranges: &[(i64, i64)]) -> i64 {
    ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&i| is_repeated_sequence(i))
        .sum()
}

fn is_repeated_sequence(number: i64) -> bool {
    let number_str = number.to_string();

    let length = number_str.len();
    for sequence_length in (1..=(length / 2)).filter(|i| length % i == 0) {
        let sequence = &number_str[..sequence_length];

        if number_str.eq(&sequence.repeat(length / sequence_length)) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("example.txt");
        let ranges = parse_input(input);
        assert_eq!(1227775554, part1(&ranges));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("example.txt");
        let ranges = parse_input(input);
        assert_eq!(4174379265, part2(&ranges));
    }
}
