use std::collections::HashMap;

fn load_data() -> Vec<u128> {
    use std::fs;

    let content = fs::read_to_string("./input/d22_input.txt").expect("could not read file");
    content
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<_>>()
}

fn mix(x: u128, y: u128) -> u128 {
    x ^ y
}

fn prune(x: u128) -> u128 {
    x % 16777216
}

fn get_next_secret(x: u128) -> u128 {
    let mut r = prune(mix(x, x * 64));
    r = prune(mix(r, r / 32));
    r = prune(mix(r, r * 2048));
    r
}

fn get_nth_next_secret(x: u128, n: usize) -> u128 {
    let mut r = x;
    for _ in 0..n {
        r = get_next_secret(r);
    }
    r
}

fn get_n_next_secrets(x: u128, n: usize) -> Vec<u128> {
    let mut r = x;
    let mut v = vec![x];
    for _ in 0..n {
        r = get_next_secret(r);
        v.push(r);
    }
    v
}

fn find_most_bananas() -> i64 {
    let v = load_data();
    let prices = v
        .iter()
        .map(|&x| {
            get_n_next_secrets(x, 2000)
                .iter()
                .map(|&y| (y % 10) as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let changes = prices
        .iter()
        .map(|v| v.windows(2).map(|s| s[1] - s[0]).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut maps = vec![HashMap::new(); prices.len()];
    for (i, buyer) in changes.iter().enumerate() {
        for (j, seq) in buyer.windows(4).enumerate() {
            let _ = maps[i].entry(seq).or_insert(prices[i][j + 4]);
        }
    }
    let mut map = HashMap::new();
    for m in maps {
        for (k, v) in m {
            let e = map.entry(k).or_insert(0);
            *e = *e + v;
        }
    }
    map.iter().map(|(_, &v)| v).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!(
            "{}",
            load_data()
                .iter()
                .map(|&x| get_nth_next_secret(x, 2000))
                .sum::<u128>()
        );
    }

    #[test]
    fn task2() {
        println!("{}", find_most_bananas());
    }
}
