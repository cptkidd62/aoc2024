use std::clone;

fn load_data() -> Vec<Vec<i32>> {
    use std::fs;

    let mut data = vec![];
    fs::read_to_string("./input/d02_input.txt")
        .expect("could not read file")
        .split('\n')
        .for_each(|l| {
            data.push(
                l.split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect(),
            )
        });
    data
}

fn is_safe(v: &Vec<i32>) -> bool {
    if v.len() < 2 || v[0] == v[1] {
        return false;
    }
    let asc = v[0] < v[1];
    for i in 0..(v.len() - 1) {
        if v[i] == v[i + 1] || (v[i] < v[i + 1]) != asc || v[i].abs_diff(v[i + 1]) > 3 {
            return false;
        }
    }
    return true;
}

fn count_safe() -> usize {
    let data = load_data();
    data.iter()
        .fold(0, |a, l| if is_safe(&l) { a + 1 } else { a })
}

fn count_safe_skip() -> usize {
    let data = load_data();
    data.iter().fold(0, |a, l| {
        if l.iter().enumerate().any(|(i, _)| {
            let mut x = l.clone();
            x.remove(i);
            is_safe(&x)
        }) {
            a + 1
        } else {
            a
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", count_safe());
    }

    #[test]
    fn task2() {
        println!("{}", count_safe_skip());
    }
}
