fn load_data() -> (Vec<usize>, Vec<usize>) {
    use std::fs;

    let content = fs::read_to_string("./input/d09_input.txt").expect("could not read file");
    let mut files = vec![];
    let mut ranges = vec![];
    let mut sum = 0;
    content.chars().enumerate().for_each(|(i, c)| {
        let n = c.to_digit(10).unwrap() as usize;
        sum += n;
        if i % 2 == 0 {
            files.push(n);
        }
        ranges.push(sum);
    });
    (files, ranges)
}

fn checksum() -> usize {
    let (mut files, ranges) = load_data();
    let total = files.iter().sum::<usize>();
    let mut checksum = 0;
    let mut bucket = 0;
    let mut last = files.len() - 1;
    for i in 0..total {
        while !(i < ranges[bucket]) {
            bucket += 1;
        }
        if bucket % 2 == 0 {
            checksum += i * (bucket / 2);
        } else {
            while !(files[last] > 0) {
                last -= 1;
            }
            files[last] -= 1;
            checksum += i * last;
        }
    }
    checksum
}

fn findfree(ranges: &Vec<usize>, n: usize) -> Option<(usize, usize)> {
    for i in 1..ranges.len() {
        if i % 2 == 1 && ranges[i] - ranges[i - 1] >= n {
            return Some((ranges[i - 1], i - 1));
        }
    }
    None
}

fn checksum2() -> usize {
    let (files, mut ranges) = load_data();
    let mut locations = vec![0];
    for (i, &r) in ranges.iter().enumerate() {
        if i % 2 == 1 {
            locations.push(r);
        }
    }
    for i in (0..files.len()).rev() {
        let nl = findfree(&ranges, files[i]);
        match nl {
            Some((x, b)) => {
                if x < locations[i] {
                    locations[i] = x;
                    ranges[b] += files[i];
                }
            }
            None => (),
        }
    }
    println!("{:?}", locations);
    let mut checksum = 0;
    for (i, &c) in locations.iter().enumerate() {
        checksum += i * (c..(c + files[i])).sum::<usize>();
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", checksum());
    }

    #[test]
    fn task2() {
        println!("{}", checksum2());
    }
}
