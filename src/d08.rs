use std::collections::{HashMap, HashSet};

struct AntennasMap {
    width: isize,
    height: isize,
    antennas: HashMap<char, Vec<(isize, isize)>>,
}

impl AntennasMap {
    fn count_antinodes(&self) -> usize {
        let mut set = HashSet::new();
        self.antennas.iter().for_each(|(_, v)| {
            for (a, b) in v.iter() {
                for (c, d) in v.iter() {
                    if *a != *c || *b != *d {
                        let diff_x = *c - *a;
                        let diff_y = *d - *b;
                        let (x1, y1) = (*a - diff_x, *b - diff_y);
                        let (x2, y2) = (*c + diff_x, *d + diff_y);
                        if x1 >= 0 && x1 < self.height && y1 >= 0 && y1 < self.width {
                            set.insert((x1, y1));
                        }
                        if x2 >= 0 && x2 < self.height && y2 >= 0 && y2 < self.width {
                            set.insert((x2, y2));
                        }
                    }
                }
            }
        });
        set.len()
    }
    fn count_multiple_antinodes(&self) -> usize {
        let mut set = HashSet::new();
        self.antennas.iter().for_each(|(_, v)| {
            for (a, b) in v.iter() {
                for (c, d) in v.iter() {
                    if *a != *c || *b != *d {
                        set.insert((*a, *b));
                        set.insert((*c, *d));
                        let diff_x = *c - *a;
                        let diff_y = *d - *b;
                        let (mut x1, mut y1) = (*a - diff_x, *b - diff_y);
                        let (mut x2, mut y2) = (*c + diff_x, *d + diff_y);
                        while x1 >= 0 && x1 < self.height && y1 >= 0 && y1 < self.width {
                            set.insert((x1, y1));
                            x1 -= diff_x;
                            y1 -= diff_y;
                        }
                        while x2 >= 0 && x2 < self.height && y2 >= 0 && y2 < self.width {
                            set.insert((x2, y2));
                            x2 += diff_x;
                            y2 += diff_y;
                        }
                    }
                }
            }
        });
        set.len()
    }
}

fn load_data() -> AntennasMap {
    use std::fs;

    let content = fs::read_to_string("./input/d08_input.txt").expect("could not read file");
    let splitted = content.split_whitespace().collect::<Vec<_>>();
    let width = splitted[0].len() as isize;
    let height = splitted.len() as isize;
    let mut antennas = HashMap::new();
    splitted.iter().enumerate().for_each(|(i, &l)| {
        l.chars().enumerate().for_each(|(j, c)| {
            if c.is_alphanumeric() {
                antennas
                    .entry(c)
                    .or_insert(vec![(i as isize, j as isize)])
                    .push((i as isize, j as isize));
            }
        })
    });

    AntennasMap {
        width,
        height,
        antennas,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", load_data().count_antinodes());
    }

    #[test]
    fn task2() {
        println!("{}", load_data().count_multiple_antinodes());
    }
}
