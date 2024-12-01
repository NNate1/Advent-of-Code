use std::iter::zip;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // let a = Vec::with_capacity(input.lines())
    let mut ids: [Vec<i32>; 2] = [Vec::new(), Vec::new()];

    for line in input.lines() {
        // for (i, id) in line.split_whitespace().enumerate() {
        //     ids[i].push((*id).parse().unwrap());
        // }
        let mut values = line.split_whitespace();

        ids[0].push(values.next().unwrap().parse().unwrap());
        ids[1].push(values.next().unwrap().parse().unwrap());
    }

    ids[0].sort();
    ids[1].sort();

    let mut diff = 0;

    for (&id1, &id2) in zip(ids[0].iter(), ids[1].iter()) {
        diff += id1.abs_diff(id2);
    }

    diff.to_string()
}

// TEST

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test1.txt");
        let output = part1(input);
        assert_eq!(output, "11");
    }
}
