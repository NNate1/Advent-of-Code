fn main() {
    let input = include_str!("./input.txt");

    let (locks, keys) = parse(input);

    println!("part1: {:?}", part1(locks, keys));
    // println!("part2: {}", output.1);
}

fn parse(input: &str) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut locks = Vec::<[usize; 5]>::new();
    let mut keys = Vec::<[usize; 5]>::new();

    let mut currently_key = false;
    let mut currently_lock = false;
    let mut lock = [5; 5];
    let mut key = [0; 5];

    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            if currently_key {
                keys.push(key);
                key = [0; 5];
            } else if currently_lock {
                locks.push(lock);
                lock = [5; 5];
            }

            currently_key = false;
            currently_lock = false;
            continue;
        } else if !(currently_key || currently_lock) {
            currently_lock = line.starts_with("#");
            currently_key = !currently_lock;
        } else if currently_lock {
            for (j, c) in line.chars().enumerate() {
                if c == '.' {
                    println!("key[{j}] {}, % {}", key[j], (i - 1) % 8);
                    lock[j] = lock[j].min((i - 1) % 8);
                }
            }
        } else if currently_key {
            for (j, c) in line.chars().enumerate() {
                if c == '#' {
                    key[j] = key[j].max(5 - ((i - 1) % 8));
                }
            }
        }
    }

    if currently_key {
        keys.push(key);
    } else if currently_lock {
        locks.push(lock);
    }

    (locks, keys)
}

fn part1(locks: Vec<[usize; 5]>, keys: Vec<[usize; 5]>) -> usize {
    println!("locks: {:?}, keys {:?}", locks, keys);
    let c_tricky = locks
        .iter()
        .zip(keys.iter())
        .filter(|(lock, key)| {
            lock.iter()
                .zip(key.iter())
                .map(|(l, k)| l + k)
                .all(|x| x <= 5)
        })
        .inspect(|(lock, key)| println!("lock: {:?}, key {:?}", lock, key))
        .count();

    let mut count = 0;
    for lock in locks {
        'keys: for key in &keys {
            for i in 0..lock.len() {
                if lock[i] + key[i] > 5 {
                    println!("NOPE: lock: {:?}, key {:?}", lock, key);
                    continue 'keys;
                }
            }

            println!("Match: lock: {:?}, key {:?}", lock, key);
            count += 1;
        }
    }

    println!("for {}, iter {}", count, c_tricky);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = include_str!("./example1.txt");
        let (locks, keys) = parse(input);
        let output = part1(locks, keys);
        assert_eq!(output, 3);
    }
}
