use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let toilet_rolls = parse_input(input);

    println!("part1: {}", part1(&toilet_rolls));
    println!("part2: {}", part2(toilet_rolls));
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Vec2 {
    x: i64,
    y: i64,
}

fn parse_input(input: &str) -> HashSet<Vec2> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter(|&(_, c)| c == '@')
                .map(move |(x, _)| Vec2 {
                    x: x as i64,
                    y: y as i64,
                })
        })
        .collect()
}

fn is_removable(position: &Vec2, toilet_rolls: &HashSet<Vec2>) -> bool {
    let mut count = 0;

    for x in (position.x - 1)..=(position.x + 1) {
        for y in (position.y - 1)..=(position.y + 1) {
            let neighbour = Vec2 { x, y };

            if toilet_rolls.contains(&neighbour) {
                count += 1;
            }
        }
    }

    count < 5
}

fn part1(toilet_rolls: &HashSet<Vec2>) -> usize {
    toilet_rolls
        .iter()
        .filter(|roll| is_removable(roll, toilet_rolls))
        .count()
}

fn part2(mut toilet_rolls: HashSet<Vec2>) -> usize {
    let mut count = 0;

    loop {
        let initial_size = toilet_rolls.len();

        let remaining_rolls: HashSet<Vec2> = toilet_rolls
            .iter()
            .filter(|roll| !is_removable(roll, &toilet_rolls))
            .cloned()
            .collect();

        let removed_now = initial_size - remaining_rolls.len();

        if removed_now == 0 {
            break;
        }

        count += removed_now;

        toilet_rolls = remaining_rolls;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("example.txt");
        let toilet_rolls = parse_input(input);
        assert_eq!(13, part1(&toilet_rolls));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("example.txt");
        let toilet_rolls = parse_input(input);
        assert_eq!(43, part2(toilet_rolls));
    }
}
