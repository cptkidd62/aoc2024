use std::collections::HashMap;

fn load_data() -> Vec<Vec<char>> {
    use std::fs;

    let content = fs::read_to_string("./input/d12_input.txt").expect("could not read file");
    content
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|&l| l.chars().collect::<Vec<_>>())
        .collect()
}

fn spread(area: &mut Vec<Vec<(char, usize)>>, i: usize, j: usize, c: char, n: usize) {
    let height = area.len();
    let width = area[0].len();
    area[i][j] = (c, n);
    if i > 0 {
        let (c1, n1) = area[i - 1][j];
        if c1 == c && n1 != n {
            spread(area, i - 1, j, c, n);
        }
    }
    if i < height - 1 {
        let (c1, n1) = area[i + 1][j];
        if c1 == c && n1 != n {
            spread(area, i + 1, j, c, n);
        }
    }
    if j > 0 {
        let (c1, n1) = area[i][j - 1];
        if c1 == c && n1 != n {
            spread(area, i, j - 1, c, n);
        }
    }
    if j < width - 1 {
        let (c1, n1) = area[i][j + 1];
        if c1 == c && n1 != n {
            spread(area, i, j + 1, c, n);
        }
    }
}

fn code_area(area: Vec<Vec<char>>) -> Vec<Vec<(char, usize)>> {
    let mut map = HashMap::new();
    let mut new_area = area
        .iter()
        .map(|l| l.iter().map(|&e| (e, 0usize)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = area.len();
    let width = area[0].len();
    for i in 0..height {
        for j in 0..width {
            let (c, n) = new_area[i][j];
            if n == 0 {
                let n1 = map.entry(c).or_insert(0);
                *n1 += 1;
                spread(&mut new_area, i, j, c, *n1);
            }
        }
    }
    new_area
}

fn map_area(area: Vec<Vec<(char, usize)>>) -> HashMap<(char, usize), (usize, usize)> {
    let mut map = HashMap::new();
    let height = area.len();
    let width = area[0].len();
    for (i, l) in area.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            let mut perim: usize = 0;
            if i == 0 || area[i - 1][j] != *c {
                perim += 1;
            }
            if i == height - 1 || area[i + 1][j] != *c {
                perim += 1;
            }
            if j == 0 || area[i][j - 1] != *c {
                perim += 1;
            }
            if j == width - 1 || area[i][j + 1] != *c {
                perim += 1;
            }
            let (a, p) = map.entry(*c).or_insert((0, 0));
            *a += 1;
            *p += perim;
        }
    }
    map
}

fn map_area_new(area: Vec<Vec<(char, usize)>>) -> HashMap<(char, usize), (usize, usize)> {
    let mut map = HashMap::new();
    let height = area.len();
    let width = area[0].len();
    for (i, l) in area.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            let mut edges: usize = 0;
            // count left edge
            if !(i > 0
                && area[i - 1][j] == *c
                && (j == 0 || area[i][j - 1] != *c && area[i - 1][j - 1] != *c)
                || j > 0 && area[i][j - 1] == *c)
            {
                edges += 1;
            }
            // count top edge
            if !(j > 0
                && area[i][j - 1] == *c
                && (i == 0 || area[i - 1][j] != *c && area[i - 1][j - 1] != *c)
                || i > 0 && area[i - 1][j] == *c)
            {
                edges += 1;
            }
            // count right edge
            if !(i > 0
                && area[i - 1][j] == *c
                && (j == width - 1 || area[i][j + 1] != *c && area[i - 1][j + 1] != *c)
                || j < width - 1 && area[i][j + 1] == *c)
            {
                edges += 1;
            }
            // count bottom edge
            if !(j > 0
                && area[i][j - 1] == *c
                && (i == height - 1 || area[i + 1][j] != *c && area[i + 1][j - 1] != *c)
                || i < height - 1 && area[i + 1][j] == *c)
            {
                edges += 1;
            }
            let (a, p) = map.entry(*c).or_insert((0, 0));
            *a += 1;
            *p += edges;
        }
    }
    map
}

fn calculate_price(map: HashMap<(char, usize), (usize, usize)>) -> usize {
    map.iter().fold(0, |a, (_, &(r, p))| a + r * p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", calculate_price(map_area(code_area(load_data()))));
    }

    #[test]
    fn task2() {
        println!("{}", calculate_price(map_area_new(code_area(load_data()))));
    }
}
