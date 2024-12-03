use regex::Regex;
fn main() {
    let input = include_str!("./input.txt");
    let output = part2_v2(input);
    dbg!(output);
}

#[allow(dead_code)]
fn part2(input: &str) -> String {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let do_re = Regex::new(r"(?:^|do\(\))((?:\n|.)*?)(?:$|don't\(\))").unwrap();
    let mut result: i32 = 0;

    for (_, [valid]) in do_re.captures_iter(input).map(|c| c.extract()) {
        result += mul_re
            .captures_iter(valid)
            .map(|c| c[1].parse::<i32>().unwrap() * c[2].parse::<i32>().unwrap())
            .sum::<i32>();
    }

    result.to_string()
}

fn part2_v2(input: &str) -> String {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut active = true;
    let mut result = 0;
    for capture in re.captures_iter(input) {
        match capture.get(0).unwrap().as_str() {
            "do()" => active = true,
            "don't()" => active = false,
            _ if active => {
                result += capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap()
            }
            _ => {}
        }
    }

    result.to_string()
}

// TEST
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("./example2.txt");
        let output = part2(input);
        assert_eq!(output, "48");
    }

    #[test]
    fn test_v2() {
        let input = include_str!("./example2.txt");
        let output = part2_v2(input);
        assert_eq!(output, "48");
    }
}
