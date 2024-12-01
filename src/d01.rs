use std::collections::{hash_map, HashMap};

fn load_data(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    use std::fs;

    let content = fs::read_to_string("./input/d01_input.txt").expect("could not read file");
    content
        .split_whitespace()
        .collect::<Vec<_>>()
        .chunks(2)
        .for_each(|c| {
            v1.push(c[0].parse().unwrap());
            v2.push(c[1].parse().unwrap())
        });
}

fn difference() -> i32 {
    let mut data1 = vec![];
    let mut data2 = vec![];
    load_data(&mut data1, &mut data2);
    data1.sort_unstable();
    data2.sort_unstable();
    data1
        .iter()
        .zip(data2.iter())
        .fold(0, |a, (x, y)| a + (x - y).abs())
}

fn similarity() -> i32 {
    let mut data1 = vec![];
    let mut data2 = vec![];
    load_data(&mut data1, &mut data2);
    let mut count: HashMap<i32, i32> = HashMap::new();
    for e in data2 {
        *count.entry(e).or_default() += 1;
    }
    data1.iter().fold(0, |a, x| a + *count.entry(*x).or_default() * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", difference());
    }

    #[test]
    fn task2() {
        println!("{}", similarity());
    }
}
