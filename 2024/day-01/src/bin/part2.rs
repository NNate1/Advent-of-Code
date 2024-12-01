use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut ids: [HashMap<i32, i32>; 2] = [HashMap::new(), HashMap::new()];

    for line in input.lines() {
        for (i, id) in line.split_whitespace().enumerate() {
            let value = id.parse().unwrap();

            ids[i].insert(value, ids[i].get(&value).unwrap_or(&0) + 1);
        }
    }

    let mut similarity: i32 = 0;

    for (id, &count) in ids[0].iter() {
        similarity += id * count * ids[1].get(id).unwrap_or(&0);
    }

    similarity.to_string()
}

// TEST

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test2.txt");
        let output = part2(input);
        assert_eq!(output, "31");
    }
}
