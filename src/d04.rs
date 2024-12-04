use std::collections::HashSet;

use regex::Regex;

fn load_data() -> String {
    use std::fs;

    let content = fs::read_to_string("./input/d04_input.txt").expect("could not read file");
    content
}

fn rcd_from_string(s: &String) -> Vec<String> {
    let mut v = vec![];
    let lines = s
        .as_str()
        .split('\n')
        .map(|e| String::from(e))
        .collect::<Vec<_>>();
    lines.iter().for_each(|l| v.push(l.clone()));
    let n = lines[0].len();
    let mut cols: Vec<String> = vec![String::new(); n];
    let mut dl: Vec<String> = vec![String::new(); n + n - 1];
    let mut dr: Vec<String> = vec![String::new(); n + n - 1];
    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            cols[j].push(c);
            dl[j + i].push(c);
            dr[n - 1 + j - i].push(c);
        })
    });
    v.append(&mut cols);
    v.append(&mut dl);
    v.append(&mut dr);
    v
}

fn d_from_string(s: &String) -> (Vec<String>, Vec<String>, usize) {
    let lines = s
        .as_str()
        .split('\n')
        .map(|e| String::from(e))
        .collect::<Vec<_>>();
    let n = lines[0].len();
    let mut dl: Vec<String> = vec![String::new(); n + n - 1];
    let mut dr: Vec<String> = vec![String::new(); n + n - 1];
    lines.iter().enumerate().for_each(|(i, l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            dl[j + i].push(c);
            dr[n - 1 + j - i].push(c);
        })
    });
    (dl, dr, n)
}

fn xmas() -> usize {
    let reg = Regex::new(r"XMAS").unwrap();
    let rreg = Regex::new(r"SAMX").unwrap();
    let data = rcd_from_string(&load_data());
    data.iter().fold(0, |a, l| {
        reg.captures_iter(l).count() + rreg.captures_iter(l).count() + a
    })
}

fn x_mas() -> usize {
    let (dl, dr, n) = d_from_string(&load_data());
    let mut l = HashSet::new();
    let mut r = HashSet::new();
    dr.iter().enumerate().for_each(|(i, d)| {
        d.match_indices("MAS").for_each(|(x, _)| {
            let a = if i >= n { 0 } else { n - i - 1 } + x + 1;
            let b = if i < n { 0 } else { i - n + 1 } + x + 1;
            _ = r.insert((a, b))
        });
        d.match_indices("SAM").for_each(|(x, _)| {
            let a = if i >= n { 0 } else { n - i - 1 } + x + 1;
            let b = if i < n { 0 } else { i - n + 1 } + x + 1;
            _ = r.insert((a, b))
        });
    });
    dl.iter().enumerate().for_each(|(i, d)| {
        d.match_indices("MAS").for_each(|(x, _)| {
            let a = if i < n { 0 } else { i - n + 1 } + x + 1;
            let b = if i >= n { n - 1 } else { i } - x - 1;
            _ = l.insert((a, b))
        });
        d.match_indices("SAM").for_each(|(x, _)| {
            let a = if i < n { 0 } else { i - n + 1 } + x + 1;
            let b = if i >= n { n - 1 } else { i } - x - 1;
            _ = l.insert((a, b))
        });
    });
    l.intersection(&r).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", xmas());
    }

    #[test]
    fn task2() {
        println!("{}", x_mas());
    }
}
