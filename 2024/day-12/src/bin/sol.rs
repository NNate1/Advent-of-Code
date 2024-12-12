fn main() {
    let input = include_str!("./input.txt");

    let farm = &parse(input);

    let output = solve(farm);

    println!("part1: {}\npart2: {}", output.0, output.1);
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut farm = Vec::new();

    for line in input.lines() {
        farm.push(Vec::from(line))
    }

    farm
}

fn solve(farm: &[Vec<u8>]) -> (usize, usize) {
    let mut visited = vec![vec![false; farm[0].len()]; farm.len()];

    let mut price_part1 = 0;
    let mut price_part2 = 0;

    for (y, row) in farm.iter().enumerate() {
        for (x, _field) in row.iter().enumerate() {
            if !visited[y][x] {
                let mut region = vec![vec![Vec::new(); farm[0].len()]; farm.len()];

                let area = dfs(farm, x, y, &mut visited, &mut region);

                let (perimeter, sides) = get_perimeter_and_sides(&region);

                price_part1 += area * perimeter;

                price_part2 += area * sides;
            }
        }
    }

    (price_part1, price_part2)
}

// Sweep lines/columns to obtain number of sides and perimeters of a region
fn get_perimeter_and_sides(region: &Vec<Vec<Vec<(i32, i32)>>>) -> (usize, usize) {
    let mut sides = 0;
    let mut perimeter = 0;

    for dir in [(0, 1), (0, -1)] {
        for row in region {
            let mut prev = false;
            for edges in row {
                let current = edges.contains(&dir);

                perimeter += if current { 1 } else { 0 };

                sides += if current && !prev { 1 } else { 0 };

                prev = current;
            }
        }
    }

    for dir in [(1, 0), (-1, 0)] {
        for x in 0..region[0].len() {
            let mut prev = false;
            for row in region {
                let current = row[x].contains(&dir);

                perimeter += if current { 1 } else { 0 };

                sides += if current && !prev { 1 } else { 0 };

                prev = current;
            }
        }
    }

    (perimeter, sides)
}

fn dfs(
    farm: &[Vec<u8>],
    x: usize,
    y: usize,
    visited: &mut Vec<Vec<bool>>,
    region: &mut Vec<Vec<Vec<(i32, i32)>>>,
) -> usize {
    visited[y][x] = true;

    let DirectionsResult(neighbours, edges) = directions(farm, x, y);

    region[y][x] = edges.clone();

    let mut area = 1;

    for (x, y) in neighbours {
        if !visited[y][x] {
            area += dfs(farm, x, y, visited, region);
        }
    }

    area
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

struct DirectionsResult(Vec<(usize, usize)>, Vec<(i32, i32)>);

fn directions(farm: &[Vec<u8>], x: usize, y: usize) -> DirectionsResult {
    let mut neighbours = Vec::new();
    let mut edges = Vec::new();

    for dir in DIRECTIONS {
        let (neighbour_x, neighbour_y) = (x as i32 + dir.0, y as i32 + dir.1);

        if (0..farm[0].len() as i32).contains(&neighbour_x)
            && (0..farm.len() as i32).contains(&neighbour_y)
            && farm[neighbour_y as usize][neighbour_x as usize] == farm[y][x]
        {
            neighbours.push((neighbour_x as usize, neighbour_y as usize));
        } else {
            edges.push(dir)
        }
    }

    DirectionsResult(neighbours, edges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 140);
    }

    #[test]
    fn test_part1_ex2() {
        let input = include_str!("./example2.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 772);
    }

    #[test]
    fn test_part1_ex3() {
        let input = include_str!("./example3.txt");
        let output = solve(&parse(input));
        assert_eq!(output.0, 1930);
    }

    #[test]
    fn test_part2_ex1() {
        let input = include_str!("./example1.txt");
        let output = solve(&parse(input));
        assert_eq!(output.1, 80);
    }

    // #[ignore]
    #[test]
    fn test_part2_ex2() {
        let input = include_str!("./example2.txt");
        let output = solve(&parse(input));
        assert_eq!(output.1, 436);
    }

    // #[ignore]
    #[test]
    fn test_part2_ex3() {
        let input = include_str!("./example3.txt");
        let output = solve(&parse(input));
        assert_eq!(output.1, 1206);
    }

    // #[ignore]
    #[test]
    fn test_part2_ex4() {
        let input = include_str!("./example4.txt");
        let output = solve(&parse(input));
        assert_eq!(output.1, 236);
    }

    // #[ignore]
    #[test]
    fn test_part2_ex5() {
        let input = include_str!("./example5.txt");
        let output = solve(&parse(input));
        assert_eq!(output.1, 368);
    }
}
