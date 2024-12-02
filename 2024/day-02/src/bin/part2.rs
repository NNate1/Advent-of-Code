fn main() {
    let input = include_str!("./input.txt");
    let output = part2_v2(input);
    dbg!(output);
}

#[allow(dead_code)]
fn part2(input: &str) -> String {
    let mut safe_count = 0;
    for line in input.lines() {
        // }
        let levels = line.split_whitespace();

        let levels: Vec<i32> = levels.map(|x| x.parse().unwrap()).collect();

        println!("\n{levels:?}");
        for ignore in 0..levels.len() {
            // let it = levels.iter();
            // let sub_levels: Vec<&i32> =
            //     it.clone().take(ignore).chain(it.skip(ignore + 1)).collect();

            let mut sub_levels = levels.clone();
            sub_levels.remove(ignore);

            let increasing = sub_levels[1] > sub_levels[0];
            println!("{sub_levels:?}");
            if sub_levels
                .windows(2)
                .all(|w| (w[1] > w[0]) == increasing && (1..=3).contains(&w[0].abs_diff(w[1])))
            {
                println!("{ignore}");
                safe_count += 1;
                break;
            }
        }
    }

    safe_count.to_string()
}

fn part2_v2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .filter(check_safety)
        .count()
}

fn check_safety(levels: &Vec<i32>) -> bool {
    // line: &str) -> bool {
    // let levels: Vec<i32> = line
    //     .split_whitespace()
    //     .map(|x| x.parse().unwrap())
    //     .collect();

    println!("\n{levels:?}");
    for ignore in 0..levels.len() {
        // let it = levels.iter();
        // let sub_levels: Vec<&i32> =
        //     it.clone().take(ignore).chain(it.skip(ignore + 1)).collect();

        let mut sub_levels = levels.clone();
        sub_levels.remove(ignore);
        let increasing = sub_levels[1] > sub_levels[0];

        println!("{sub_levels:?}");

        if sub_levels
            .windows(2)
            .all(|w| (w[1] > w[0]) == increasing && (1..=3).contains(&w[0].abs_diff(w[1])))
        {
            println!("{ignore}");
            return true;
        }
    }
    false
}
// TEST

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        let input = include_str!("./example.txt");
        let output = part2(input);
        assert_eq!(output, "4");
    }

    #[test]
    fn test_v2() {
        let input = include_str!("./example.txt");
        let output = part2_v2(input);
        assert_eq!(output, 4);
    }
}
