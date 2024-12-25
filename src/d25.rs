fn map_pin(pins: &Vec<Vec<char>>) -> Vec<i32> {
    let mut v = vec![-1; 5];
    for l in pins {
        for (i, &e) in l.iter().enumerate() {
            if e == '#' {
                v[i] += 1;
            }
        }
    }
    v
}

fn load_data() -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    use std::fs;

    let content = fs::read_to_string("./input/d25_input.txt").expect("could not read file");
    let spl = content
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|e| e.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let locks = spl.iter().filter(|&e| e[0][0] == '#').collect::<Vec<_>>();
    let keys = spl.iter().filter(|&e| e[0][0] == '.').collect::<Vec<_>>();
    (
        locks.iter().map(|&x| map_pin(x)).collect::<Vec<_>>(),
        keys.iter().map(|&x| map_pin(x)).collect::<Vec<_>>(),
    )
}

fn lock_key_fit(lock: &Vec<i32>, key: &Vec<i32>) -> bool {
    lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5)
}

fn count_fitting() -> usize {
    let (locks, keys) = load_data();
    let mut cnt = 0;
    for l in locks.iter() {
        for k in keys.iter() {
            if lock_key_fit(l, k) {
                cnt += 1;
            }
        }
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", count_fitting());
    }

    #[test]
    fn task2() {
        // println!("{}", find_most_bananas());
    }
}
