use core::panic;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");

    let (instructions, mut registers, unparsed_instructions) = parse(input);

    println!("part1: {}", part1(&instructions, &mut registers));

    println!("part2: {:?}", part2(&instructions, unparsed_instructions));
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Adv(u8),
    Bxl(u8),
    Bst(u8),
    Jnz(u8),
    Bxc,
    Out(u8),
    Bdv(u8),
    Cdv(u8),
}

#[derive(Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

fn parse(input: &str) -> (Vec<Operation>, Registers, &str) {
    let mut registers = [0, 0, 0];

    for (i, line) in input.lines().take(3).enumerate() {
        if let Some(value) = line
            .split_whitespace()
            .last()
            .map(|c| c.parse().expect("Failed to parse \"{c}\""))
        {
            registers[i] = value;
        }
    }
    let [a, b, c] = registers;

    let unparsed_instructions = input.lines().last().unwrap().split_once(' ').unwrap().1;

    (
        parse_instructions(input),
        Registers { a, b, c },
        unparsed_instructions,
    )
}

fn parse_instructions(input: &str) -> Vec<Operation> {
    let mut instructions = Vec::new();
    if let Some(line) = input.lines().last() {
        for (opcode, operand) in line
            .split_whitespace()
            .last()
            .unwrap()
            .split(',')
            .map(|c| c.parse().expect("Failed to parse \"{c}\""))
            .tuples()
        {
            instructions.push(match opcode {
                0 => Operation::Adv(operand),
                1 => Operation::Bxl(operand),
                2 => Operation::Bst(operand),
                3 => Operation::Jnz(operand),
                4 => Operation::Bxc,
                5 => Operation::Out(operand),
                6 => Operation::Bdv(operand),
                7 => Operation::Cdv(operand),
                _ => panic!("Unexpected opcode: \"{opcode}\""),
            });
        }
    }
    instructions
}

fn combo_operand(operand: u8, registers: &Registers) -> u64 {
    match operand {
        0..=3 => operand as u64,
        4 => registers.a,
        5 => registers.b,
        6 => registers.c,
        7 => todo!(),
        _ => panic!("Unexpected operand: \"{operand}\""),
    }
}

fn part1(instructions: &[Operation], registers: &mut Registers) -> String {
    let mut ip = 0;
    let mut output;
    let mut results = String::new();

    while ip < instructions.len() {
        (ip, output) = instructions[ip].execute(registers, ip);

        if let Some(value) = output {
            results.push_str(&value.to_string());

            results.push(',');
        }
    }

    results.truncate(results.len() - 1);
    results
}
impl Operation {
    fn adv(operand: u8, registers: &Registers) -> u64 {
        let a = registers.a / (1 << combo_operand(operand, registers));
        let b = registers.a >> (combo_operand(operand, registers));
        assert_eq!(a, b, "{a}, {b}");
        b
    }

    fn execute(&self, registers: &mut Registers, mut ip: usize) -> (usize, Option<u64>) {
        ip += 1;

        let mut output = None;
        match *self {
            Operation::Adv(operand) => registers.a = Operation::adv(operand, registers),
            Operation::Bxl(operand) => registers.b ^= operand as u64,
            Operation::Bst(operand) => registers.b = combo_operand(operand, registers) % 8,
            Operation::Jnz(operand) => {
                if registers.a != 0 {
                    ip = operand as usize
                }
            }
            Operation::Bxc => registers.b ^= registers.c,
            Operation::Out(operand) => output = Some(combo_operand(operand, registers) % 8),
            Operation::Bdv(operand) => registers.b = Operation::adv(operand, registers),

            Operation::Cdv(operand) => registers.c = Operation::adv(operand, registers),
        };
        (ip, output)
    }
}

fn part2(instructions: &[Operation], unparsed_instructions: &str) -> Option<u64> {
    let target: Vec<u8> = unparsed_instructions
        .split(',')
        .map(|c| c.parse().unwrap())
        .rev()
        .collect();

    let solutions = reverse(instructions, &target, 0);

    for a in solutions.into_iter().flat_map(|a| [a, (a << 3)]) {
        let mut registers = Registers { a, b: 0, c: 0 };
        let output = part1(instructions, &mut registers);

        let parse_instructions = parse_instructions(output.as_str());
        if parse_instructions.as_slice() == instructions {
            return Some(a);
        }
    }

    None
}

fn reverse(instructions: &[Operation], target: &[u8], a: u64) -> Vec<u64> {
    if target.is_empty() {
        return vec![a];
    }

    let mut solutions = Vec::new();

    for bits in 0..=0b111 {
        let possible_a = (a << 3) + bits;
        let mut registers = Registers {
            a: possible_a,
            b: 0,
            c: 0,
        };

        for instruction in instructions {
            if let Operation::Adv(_) = instruction {
                continue;
            }

            let (_, output) = instruction.execute(&mut registers, 0);

            if let Some(value) = output {
                if value as u8 == target[0] {
                    solutions.append(&mut reverse(instructions, &target[1..], possible_a));
                }
            }
        }
    }

    solutions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");
        let (instructions, mut registers, _) = parse(input);
        let output = part1(&instructions, &mut registers);
        assert_eq!(output, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part1_ex3() {
        let input = include_str!("./example3.txt");
        let (instructions, mut registers, _) = parse(input);
        let output = part1(&instructions, &mut registers);
        assert_eq!(output, "0,3,5,4,3,0");
    }

    // #[ignore = "reason"]
    #[test]
    fn test_part1_ex4() {
        let input = include_str!("./example4.txt");
        let (instructions, mut registers, _) = parse(input);
        let output = part1(&instructions, &mut registers);
        assert_eq!(output, "1,5,0,3,5,4,3,0");
    }

    // #[ignore]
    #[test]
    fn test_part2_ex2() {
        let input = include_str!("./example3.txt");
        let (instructions, _, unparsed_intructions) = parse(input);
        let output = part2(&instructions, unparsed_intructions);
        assert_eq!(output, Some(117440));
    }

    // #[ignore = "reason"]
    #[test]
    fn test_part2_ex4() {
        let input = include_str!("./example4.txt");
        let (instructions, _, unparsed_intructions) = parse(input);
        let output = part2(&instructions, unparsed_intructions);
        assert_eq!(output, Some(7516488));
    }
}
