use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    // let output = part1(input);
    println!("part1: {}\npart2: {}", part1(input), part2(input));
}

#[derive(Debug)]
enum Operator {
    Sum,
    Multiplication,
    Concatenation,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Sum => write!(f, "+"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Concatenation => write!(f, "||"),
        }
    }
}
impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Sum => a + b,
            Operator::Multiplication => a * b,
            // Operator::Concatenation => b + (a << (64 - b.leading_zeros())),
            Operator::Concatenation => {
                let mut pad = 1;
                while pad <= b {
                    pad *= 10;
                }
                a * pad + b
            }
        }
    }
}

fn part1(input: &str) -> i64 {
    do_it(input, &[Operator::Sum, Operator::Multiplication])
}

fn part2(input: &str) -> i64 {
    do_it(
        input,
        &[
            Operator::Sum,
            Operator::Multiplication,
            Operator::Concatenation,
        ],
    )
}

fn valid(equation: &Vec<i64>, operators: &[Operator]) -> bool {
    let target = equation[0];

    let iter = operators.iter();
    for permutation in itertools::repeat_n(iter, equation.len() - 2).multi_cartesian_product() {
        let mut result = equation[1];

        // print!("{result} ");
        for (op, &value) in permutation.iter().zip(equation.iter().skip(2)) {
            result = op.apply(result, value);
            // print!("{op} {value}");
        }

        // println!("\nresults: {result}");

        if target == result {
            return true;
        }
    }
    false
}

fn do_it(input: &str, operators: &[Operator]) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split([' ', ':'])
                .filter(|x| !x.is_empty())
                .map(|x| x.parse().unwrap_or_else(|_| panic!("{} !!! {}", line, x)))
                .collect()
        })
        .filter(|equation| valid(equation, &operators))
        .map(|equation| equation[0])
        .sum()
}

// TEST
#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1() {
        let input = include_str!("./example.txt");
        let output = part1(input);
        assert_eq!(output, 3749);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./example.txt");
        let output = part2(input);
        assert_eq!(output, 11387);
    }

    #[test]
    fn test_mini_part1() {
        let input = include_str!("./example_mini.txt");
        let output = part1(input);
        assert_eq!(output, 292);
    }
}
