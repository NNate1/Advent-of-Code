use std::collections::{HashSet, VecDeque};

use good_lp::{Expression, ProblemVariables, Solution, SolverModel, default_solver, variable};

fn main() {
    let input = include_str!("input.txt");
    let machines = parse_input(input);
    let (part1, part2) = (part1(&machines), part2(&machines));
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u64>,
}

fn parse_input(input: &str) -> Vec<Machine> {
    // input: [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

    let mut machines = Vec::new();

    for line in input.lines() {
        // 1. Separate the Lights string (e.g., "[.##......]") from the rest
        let (lights_str, buttons_and_joltage) = line.split_once("]").unwrap();

        // 2. Parse Lights: Skip '[' and map '#' to true, '.' to false
        let lights = lights_str.chars().skip(1).map(|c| c == '#').collect();

        // 3. Separate Buttons string from Joltage string
        let (buttons_str, joltage_str) = buttons_and_joltage.split_once("{").unwrap();

        // 4. Parse Buttons: Trims parentheses and parses indices.
        let buttons = buttons_str
            .split_whitespace()
            .map(|button| {
                // Trim surrounding parentheses and parse comma-separated indices
                button
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>() // Explicit type hint
            })
            .collect();

        // 5. Parse Joltage: Trims braces and parses target values.
        let joltage = joltage_str
            .trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        machines.push(Machine {
            lights,
            buttons,
            joltage,
        });
    }

    machines
}

fn part1(machines: &[Machine]) -> u64 {
    machines.iter().map(button_presses_bfs).sum()
}

fn button_presses_bfs(machine: &Machine) -> u64 {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // Start state: All lights off, 0 presses.
    queue.push_back((vec![false; machine.lights.len()], 0));

    while let Some((lights, presses)) = queue.pop_front() {
        for button in machine.buttons.iter() {
            let mut new_lights = lights.clone();

            for &light in button {
                new_lights[light] ^= true;
            }

            if new_lights == machine.lights {
                return presses + 1;
            }
            if visited.insert(new_lights.clone()) {
                queue.push_back((new_lights, presses + 1));
            }
        }
    }

    panic!("No solution found for part1")
}

fn part2(machines: &[Machine]) -> u64 {
    machines
        .iter()
        // .map(button_presses_joltage_bfs_too_slow)
        .map(button_presses_integer_programming)
        .sum()
}

fn button_presses_integer_programming(machine: &Machine) -> u64 {
    let target_joltage = &machine.joltage;
    let num_buttons = machine.buttons.len();

    // Calculate maximum number of presses of each button
    let button_max_presses: Vec<u64> = machine
        .buttons
        .iter()
        .map(|button| {
            button
                .iter()
                .map(|&light| target_joltage[light])
                .min()
                .unwrap_or(0)
        })
        .collect();

    // Objetive is to minimize the sum of each button press
    let mut total_presses_objective = Expression::with_capacity(num_buttons);

    let mut vars = ProblemVariables::new();
    let mut button_vars = Vec::new();

    // Define number of button presses as variables
    for (i, &max_presses) in button_max_presses.iter().enumerate() {
        let button_var = vars.add(
            variable()
                .min(0.)
                .max(max_presses as f64)
                .integer()
                .name(format!("button_{}", i)),
        );

        button_vars.push(button_var);

        // Add button presses var to objetive expression
        total_presses_objective += button_var;
    }

    // Minimize number of presses
    let mut problem = vars
        .minimise(&total_presses_objective)
        .using(default_solver);

    // Add constraints: joltage must be equal to target joltage
    for (jolt_idx, &joltage) in target_joltage.iter().enumerate() {
        let mut constraint_lhs = Expression::with_capacity(num_buttons);

        // Sum all buttons that affect joltage of 'jolt_idx'
        for (button_idx, button_lights) in machine.buttons.iter().enumerate() {
            if button_lights.contains(&jolt_idx) {
                constraint_lhs += button_vars[button_idx];
            }
        }

        problem.add_constraint(
            constraint_lhs
                .eq(joltage as f64)
                .set_name(format!("joltage_constraint_{}", jolt_idx)),
        );
    }

    let solution = problem.solve();

    match solution {
        Ok(sol) => sol.eval(&total_presses_objective) as u64,
        Err(e) => {
            eprintln!("Solver error: {:?}", e);
            panic!("Solver failed to find a solution.");
        }
    }
}

#[allow(dead_code)]
fn button_presses_joltage_bfs_too_slow(machine: &Machine) -> u64 {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((vec![0; machine.joltage.len()], 0));

    while let Some((joltage, presses)) = queue.pop_front() {
        for button in machine.buttons.iter() {
            let mut new_joltage = joltage.clone();
            for &light in button {
                new_joltage[light] += 1;
                if new_joltage[light] > machine.joltage[light] {
                    continue;
                }
            }

            if new_joltage == machine.joltage {
                return presses + 1;
            }

            if seen.insert(new_joltage.clone()) {
                queue.push_back((new_joltage, presses + 1));
            }
        }
    }

    panic!()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("example.txt");
        let machines = parse_input(input);
        assert_eq!(7, part1(&machines));
    }

    #[test]
    fn part2_test() {
        let input = include_str!("example.txt");
        let machines = parse_input(input);
        assert_eq!(33, part2(&machines));
    }
}
