use itertools::Itertools;
fn main() {
    let input = include_str!("input.txt");
    // let input = include_str!("example.txt");
    let coordinates = parse_input(input);
    println!("{coordinates:?}");
    println!("part 1: {}", part1(&coordinates));
}

fn parse_input(input : &str) -> Vec<(i64, i64)> {

    input.lines().map(
    |line| { let (x, y) = line.split_once(',').unwrap();
    return ( x.parse().unwrap()
    , y.parse().unwrap())
        }).collect()
}


fn part1(coordinates : &[(i64,i64)]) -> i64 {
    coordinates.iter()
        .cartesian_product(coordinates.iter())
        .map(|(&a, &b) | area(a,b))
        .max()
        .unwrap()


}

fn area(p1 : (i64, i64), p2 : (i64,i64)) -> i64 {

    
    let result =(1 + (p1.0 - p2.0).abs() )*
    (1 +(p1.1 - p2.1).abs());

    // println!("{p1:?} {p2:?} results in {result}");
    result
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
fn part1_test() {

    let input = include_str!("example.txt");
    let coordinates = parse_input(input);
    assert_eq!(50, part1(&coordinates));
    }
}
