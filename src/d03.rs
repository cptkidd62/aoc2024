use regex::Regex;

fn load_data() -> String {
    use std::fs;

    let content = fs::read_to_string("./input/d03_input.txt").expect("could not read file");
    content
}

fn filter_multiply() -> usize {
    let reg = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let s = load_data();
    reg.find_iter(s.as_str()).fold(0, |a, m| {
        let r = m
            .as_str()
            .strip_prefix("mul(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split(",")
            .collect::<Vec<_>>();
        a + r.iter().fold(1, |b, n| b * n.parse::<usize>().unwrap())
    })
}

fn filter_multiply_switch() -> usize {
    let reg = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let s = load_data();
    reg.find_iter(s.as_str())
        .fold((0, true), |(a, en), m| {
            match m.as_str().strip_suffix(")").unwrap() {
                "do(" => (a, true),
                "don't(" => (a, false),
                x => {
                    let r = x
                        .strip_prefix("mul(")
                        .unwrap()
                        .split(",")
                        .collect::<Vec<_>>();
                    (
                        if en {
                            a + r.iter().fold(1, |b, n| b * n.parse::<usize>().unwrap())
                        } else {
                            a
                        },
                        en,
                    )
                }
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", filter_multiply());
    }

    #[test]
    fn task2() {
        println!("{}", filter_multiply_switch());
    }
}
