use std::collections::{HashMap, HashSet};

struct Trails {
    width: usize,
    height: usize,
    map: Vec<Vec<usize>>,
    possible_moves: HashMap<(usize, usize), Vec<(usize, usize)>>,
    heads: Vec<(usize, usize)>,
}

impl Trails {
    fn get_to_nine(self: &Trails, x: usize, y: usize) -> HashSet<(usize, usize)> {
        if self.map[x][y] == 9 {
            return HashSet::from([(x, y)]);
        }
        let mut n = HashSet::new();
        match self.possible_moves.get(&(x, y)) {
            Some(v) => {
                for (i, j) in v {
                    n.extend(self.get_to_nine(*i, *j));
                }
            }
            None => (),
        }
        n
    }

    fn count_score(self: &Trails) -> usize {
        self.heads
            .iter()
            .fold(0, |a, &(i, j)| a + self.get_to_nine(i, j).len())
    }

    fn get_to_nine_new(self: &Trails, x: usize, y: usize) -> usize {
        if self.map[x][y] == 9 {
            return 1;
        }
        match self.possible_moves.get(&(x, y)) {
            Some(v) => v
                .iter()
                .fold(0, |a, &(i, j)| a + self.get_to_nine_new(i, j)),
            None => 0,
        }
    }

    fn count_score_new(self: &Trails) -> usize {
        self.heads
            .iter()
            .fold(0, |a, &(i, j)| a + self.get_to_nine_new(i, j))
    }
}

fn load_data() -> Trails {
    use std::fs;

    let content = fs::read_to_string("./input/d10_input.txt").expect("could not read file");
    let lines = content.split_whitespace().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len();
    let map = lines
        .iter()
        .map(|&l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut possible_moves = HashMap::new();
    let mut heads = vec![];
    map.iter().enumerate().for_each(|(i, l)| {
        l.iter().enumerate().for_each(|(j, &e)| {
            if e == 0 {
                heads.push((i, j));
            }
            if i > 0 && map[i - 1][j] == e + 1 {
                possible_moves
                    .entry((i, j))
                    .or_insert(vec![])
                    .push((i - 1, j));
            }
            if i < height - 1 && map[i + 1][j] == e + 1 {
                possible_moves
                    .entry((i, j))
                    .or_insert(vec![])
                    .push((i + 1, j));
            }
            if j > 0 && map[i][j - 1] == e + 1 {
                possible_moves
                    .entry((i, j))
                    .or_insert(vec![])
                    .push((i, j - 1));
            }
            if j < width - 1 && map[i][j + 1] == e + 1 {
                possible_moves
                    .entry((i, j))
                    .or_insert(vec![])
                    .push((i, j + 1));
            }
        })
    });
    Trails {
        width,
        height,
        map,
        possible_moves,
        heads,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", load_data().count_score());
    }

    #[test]
    fn task2() {
        println!("{}", load_data().count_score_new());
    }
}
