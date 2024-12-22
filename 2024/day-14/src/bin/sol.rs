use ::std::collections::HashMap;

use ::std::collections::HashSet;
use regex::Regex;
use rusttype::Vector;
use std::fs;
use std::path::Path;

fn main() {
    let input = include_str!("./input.txt");

    let guards = parse(input);

    println!("part1: {}", part1(guards.clone(), 101, 103));
    part2(guards, 101, 103);
    println!("part2: search Tree/* files");
}

fn parse(input: &str) -> HashMap<Vector<i32>, Vec<Vector<i32>>> {
    let mut guards: HashMap<Vector<i32>, Vec<Vector<i32>>> = HashMap::new();

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    for (_, [px, py, vx, vy]) in re.captures_iter(input).map(|c| c.extract()) {
        let position = Vector {
            x: px.parse().unwrap(),
            y: py.parse().unwrap(),
        };
        let velocity = Vector {
            x: vx.parse().unwrap(),
            y: vy.parse().unwrap(),
        };

        guards.entry(position).or_default().push(velocity);
    }

    guards
}

fn part1(guards: HashMap<Vector<i32>, Vec<Vector<i32>>>, width: i32, height: i32) -> usize {
    let destinations = patrol(guards, width, height, 100);

    quadrants(destinations, width, height)
}

fn patrol(
    mut guards: HashMap<Vector<i32>, Vec<Vector<i32>>>,
    width: i32,
    height: i32,
    iterations: i32,
) -> HashMap<Vector<i32>, Vec<Vector<i32>>> {
    for _ in 0..iterations {
        let mut new_guards: HashMap<Vector<i32>, Vec<Vector<i32>>> = HashMap::new();

        for (pos, velocities) in guards {
            for vel in velocities {
                let mut destination = vel + pos;
                destination.x = (destination.x + width) % width;
                destination.y = (destination.y + height) % height;

                new_guards.entry(destination).or_default().push(vel);
            }
        }

        guards = new_guards;
    }

    guards
}

fn quadrants(guards: HashMap<Vector<i32>, Vec<Vector<i32>>>, width: i32, height: i32) -> usize {
    let mut quadrants = [[0, 0], [0, 0]];

    let mut quadrant_guards = [
        [HashSet::new(), HashSet::new()],
        [HashSet::new(), HashSet::new()],
    ];
    for (pos, velocities) in guards {
        let x = if pos.x > width / 2 {
            1
        } else if pos.x < width / 2 {
            0
        } else {
            continue;
        };

        let y = if pos.y > height / 2 {
            1
        } else if pos.y < height / 2 {
            0
        } else {
            continue;
        };

        assert!(pos.x != width / 2, "X: {pos:?}");

        assert!(pos.y != height / 2, "Y: {pos:?}");
        quadrants[y][x] += velocities.len();
        quadrant_guards[y][x].insert(pos);
    }

    quadrants.iter().flatten().product::<usize>()
}

fn part2(guards: HashMap<Vector<i32>, Vec<Vector<i32>>>, width: i32, height: i32) {
    let path = Path::new("Trees");

    if !path.exists() {
        fs::create_dir_all(path).expect("Unable to create directory \"Trees\"");
    }

    tree(guards, width, height, 10000)
}
fn tree(
    mut guards: HashMap<Vector<i32>, Vec<Vector<i32>>>,
    width: i32,
    height: i32,
    _iterations: i32,
) {
    // save every guard layout as a png and manually look through the files
    for i in 0..=7055 {
        // for _i in 0..iterations {

        if could_be_tree(&guards) && i == 7055 {
            print_tree(&guards, width, height, i);
        }

        let mut new_guards: HashMap<Vector<i32>, Vec<Vector<i32>>> = HashMap::new();
        for (pos, velocities) in guards {
            for vel in velocities {
                let mut destination = vel + pos;
                destination.x = (destination.x + width) % width;
                destination.y = (destination.y + height) % height;
                new_guards.entry(destination).or_default().push(vel);
            }
        }

        guards = new_guards;
    }
}

use image::{Rgb, RgbImage};

fn print_tree(
    guards: &HashMap<Vector<i32>, Vec<Vector<i32>>>,
    width: i32,
    height: i32,
    iteration: i32,
) {
    let mut img = RgbImage::new(width as u32, height as u32);

    let background = Rgb([255, 255, 255]);
    for pixel in img.pixels_mut() {
        *pixel = background;
    }

    let guard_color = Rgb([0, 0, 0]);

    for (&guard, _) in guards.iter() {
        if guard.x >= 0 && guard.x < width && guard.y >= 0 && guard.y < height {
            img.put_pixel(guard.x as u32, guard.y as u32, guard_color);
        }
    }

    let filename = format!("Trees/tree_iteration_{}.png", iteration);
    match img.save(&filename) {
        Ok(_) => (), // println!("Image saved as {}", filename),
        Err(e) => panic!("Failed to save image: {}", e),
    }
}

fn could_be_tree(guards: &HashMap<Vector<i32>, Vec<Vector<i32>>>) -> bool {
    for pos in guards.keys() {
        if guards.contains_key(&(*pos + Vector { x: 0, y: 1 }))
            && guards.contains_key(&(*pos + Vector { x: 0, y: -1 }))
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[ignore]
    #[test]
    fn test_part1_ex1() {
        let input = parse(include_str!("./example1.txt"));
        let output = part1(input, 11, 7);
        assert_eq!(output, 12);
    }
}
