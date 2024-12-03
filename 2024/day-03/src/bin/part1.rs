use regex::Regex;
fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
        .sum()
    // let mut result = 0;
    //
    // for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
    //     result += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    // }
    //
    // result.to_string()
}

// TEST

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        let input = include_str!("./example1.txt");
        let output = part1(input);
        assert_eq!(output, 161);
    }
}
