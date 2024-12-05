fn load_data() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    use std::fs;

    let content = fs::read_to_string("./input/d05_input.txt").expect("could not read file");
    let splitted = content.as_str().split("\n\n").collect::<Vec<_>>();

    let pairs = splitted[0]
        .split("\n")
        .map(|l| {
            let t = l.split("|").collect::<Vec<_>>();
            (t[0].parse::<u32>().unwrap(), t[1].parse::<u32>().unwrap())
        })
        .collect::<Vec<(_, _)>>();

    let input = splitted[1]
        .split("\n")
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (pairs, input)
}

fn middle_pages() -> u32 {
    let (order, data) = load_data();
    let filtered = data
        .iter()
        .filter(|&l| {
            order.iter().all(|(a, b)| {
                match (l.iter().position(|r| r == a), l.iter().position(|r| r == b)) {
                    (Some(x), Some(y)) => x < y,
                    _ => true,
                }
            })
        })
        .collect::<Vec<_>>();
    filtered
        .iter()
        .fold(0, |a, &l| l.iter().nth(l.len() / 2).unwrap() + a)
}

fn fixed_middle_pages() -> u32 {
    let (order, data) = load_data();
    let filtered = data
        .iter()
        .filter(|&l| {
            !order.iter().all(|(a, b)| {
                match (l.iter().position(|r| r == a), l.iter().position(|r| r == b)) {
                    (Some(x), Some(y)) => x < y,
                    _ => true,
                }
            })
        })
        .collect::<Vec<_>>();
    let mut nf = filtered.iter().map(|&l| l.clone()).collect::<Vec<_>>();
    nf.iter_mut().for_each(|l| {
        l.sort_by(
            |a, b| match order.iter().position(|(x, y)| x == a && y == b) {
                Some(_) => std::cmp::Ordering::Less,
                None => match order.iter().position(|(y, x)| x == a && y == b) {
                    Some(_) => std::cmp::Ordering::Greater,
                    None => std::cmp::Ordering::Less,
                },
            },
        )
    });
    nf.iter()
        .fold(0, |a, l| l.iter().nth(l.len() / 2).unwrap() + a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", middle_pages());
    }

    #[test]
    fn task2() {
        println!("{}", fixed_middle_pages());
    }
}
