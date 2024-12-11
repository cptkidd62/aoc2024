use std::collections::HashMap;

fn load_data() -> Vec<u128> {
    use std::fs;

    let content = fs::read_to_string("./input/d11_input.txt").expect("could not read file");
    content
        .split_whitespace()
        .map(|n| n.parse::<u128>().unwrap())
        .collect::<Vec<_>>()
}

fn count_digits(n: u128) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn blink_n_times(stones: Vec<u128>, n: usize) -> usize {
    let mut stones = stones.clone();
    let mut new = vec![];
    for _ in 0..n {
        for &s in stones.iter() {
            if s == 0 {
                new.push(1);
            } else if count_digits(s) % 2 == 0 {
                let d = 10u128.pow(count_digits(s) / 2);
                new.push(s / d);
                new.push(s % d);
            } else {
                new.push(s * 2024);
            }
        }
        stones = new.clone();
        new = vec![];
    }
    stones.len()
}

fn blink_n_times_distinct(stones: Vec<u128>, n: usize) -> usize {
    let mut map = HashMap::new();
    stones
        .iter()
        .for_each(|&s| *map.entry(s).or_insert(0usize) += 1);
    let mut new = HashMap::new();
    for _ in 0..n {
        for (&s, &t) in map.iter() {
            if s == 0 {
                *new.entry(1).or_insert(0usize) += t;
            } else if count_digits(s) % 2 == 0 {
                let d = 10u128.pow(count_digits(s) / 2);
                *new.entry(s / d).or_insert(0usize) += t;
                *new.entry(s % d).or_insert(0usize) += t;
            } else {
                *new.entry(s * 2024).or_insert(0usize) += t;
            }
        }
        map = new.clone();
        new.clear();
    }
    map.iter().fold(0, |a, (_, &v)| a + v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", blink_n_times(load_data(), 25));
    }

    #[test]
    fn task2() {
        println!("{}", blink_n_times_distinct(load_data(), 75));
    }
}
