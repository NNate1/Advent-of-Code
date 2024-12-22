use coin_cbc::raw::Status;
use coin_cbc::{Model, Sense};
use regex::Regex;
use rusttype::Vector;

fn main() {
    let input = include_str!("./input.txt");

    let arcade = parse(input);

    let p1 = part1(&arcade);

    let p2 = part2(&arcade);
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}

fn parse(input: &str) -> Vec<Vec<Vector<f64>>> {
    let re = Regex::new(r"(\d+)").unwrap();

    let mut list = Vec::new();
    let mut machine = Vec::new();
    for line in input.lines() {
        if machine.len() == 3 {
            list.push(machine.clone());

            machine.clear();
        }

        if line.is_empty() {
            continue;
        }
        let mut variable = Vector { x: 0., y: 0. };

        let mut iter = re.captures_iter(line);
        variable.x = iter
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        variable.y = iter
            .next()
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        machine.push(variable);
    }

    if machine.len() == 3 {
        list.push(machine.clone());

        machine.clear();
    }

    list
}

fn part1(list: &[Vec<Vector<f64>>]) -> u128 {
    solve(list, 0.)
}

fn part2(list: &[Vec<Vector<f64>>]) -> u128 {
    solve(list, 10000000000000.)
}

fn solve(list: &[Vec<Vector<f64>>], inc: f64) -> u128 {
    let mut sol = 0;
    let tokens = [3, 1];

    for machine in list {
        let point_a = machine[0];
        let point_b = machine[1];
        let mut target = machine[2];

        target.x += inc;

        target.y += inc;

        if let Some([a, b]) = lp(&target, &point_a, &point_b) {
            let a = a.round(); //as f64;

            let b = b.round(); //as f64;
            let x = a * point_a.x + b * point_b.x == target.x;

            let y = a * point_a.y + b * point_b.y == target.y;

            if x && y {
                sol += a as u128 * tokens[0] + b as u128 * tokens[1];
            }
        }
    }

    sol
}

fn lp(target: &Vector<f64>, var_a: &Vector<f64>, var_b: &Vector<f64>) -> Option<[f64; 2]> {
    let mut model = Model::default();
    model.set_parameter("logLevel", "0");

    let a = model.add_integer();
    let b = model.add_integer();

    // minimize 3*a + b
    model.set_obj_coeff(a, 3.0);
    model.set_obj_coeff(b, 1.0);
    model.set_obj_sense(Sense::Minimize);

    // a >= 0
    model.set_col_lower(a, 0.);
    model.set_col_upper(a, f64::MAX);

    // b >= 0
    model.set_col_lower(b, 0.);
    model.set_col_upper(b, f64::MAX);

    // a*a_x + b*b_x = target.x
    let row = model.add_row();
    model.set_row_equal(row, target.x);
    model.set_weight(row, a, var_a.x);
    model.set_weight(row, b, var_b.x);

    // a*a_y + b*b_y = target.y
    let row = model.add_row();
    model.set_row_equal(row, target.y);
    model.set_weight(row, a, var_a.y);
    model.set_weight(row, b, var_b.y);

    let solution = model.solve();

    assert_eq!(Status::Finished, solution.raw().status());

    if Status::Finished == solution.raw().status() {
        let a = solution.col(a);
        let b = solution.col(b);
        Some([a, b])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1_ex0() {
        let input = parse(include_str!("./example0.txt"));
        let output = part1(&input);
        assert_eq!(output, 8);
    }

    // #[ignore]
    #[test]
    fn test_part1_ex1() {
        let input = parse(include_str!("./example1.txt"));
        let output = part1(&input);
        assert_eq!(output, 280);
    }

    // #[ignore]
    #[test]
    fn test_part1_ex2() {
        let input = parse(include_str!("./example2.txt"));
        let output = part1(&input);
        assert_eq!(output, 480);
    }

    // #[ignore]
    #[test]
    fn test_part2_ex0() {
        let input = parse(include_str!("./example0.txt"));
        let output = part2(&input);
        assert_eq!(output, 7500000000008);
    }
}
