use std::{collections::HashMap, u128};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Maze {
    transitions: HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn calculate_trans_costs(
    start_v: &(usize, usize),
    end_v: &(usize, usize),
    map: &HashMap<Dir, u128>,
) -> HashMap<Dir, u128> {
    use Dir::*;
    let mut nmap = HashMap::new();
    let &up_c = map.get(&Up).unwrap();
    let &right_c = map.get(&Right).unwrap();
    let &down_c = map.get(&Down).unwrap();
    let &left_c = map.get(&Left).unwrap();
    // up dir
    if start_v.0 < end_v.0 {
        // down
        nmap.insert(
            Up,
            (up_c + 2000 + 1 + 2000)
                .min(right_c + 1000 + 1 + 2000)
                .min(down_c + 1 + 2000)
                .min(left_c + 1000 + 1 + 2000),
        );
    } else if start_v.0 > end_v.0 {
        // up
        nmap.insert(
            Up,
            (up_c + 1)
                .min(right_c + 1000 + 1)
                .min(down_c + 2000 + 1)
                .min(left_c + 1000 + 1),
        );
    } else if start_v.1 < end_v.1 {
        // right
        nmap.insert(
            Up,
            (up_c + 1000 + 1 + 1000)
                .min(right_c + 1 + 1000)
                .min(down_c + 1000 + 1 + 1000)
                .min(left_c + 2000 + 1 + 1000),
        );
    } else if start_v.1 > end_v.1 {
        // left
        nmap.insert(
            Up,
            (up_c + 1000 + 1 + 1000)
                .min(right_c + 2000 + 1 + 1000)
                .min(down_c + 1000 + 1 + 1000)
                .min(left_c + 1 + 1000),
        );
    }
    // right dir
    if start_v.0 < end_v.0 {
        // down
        nmap.insert(
            Right,
            (up_c + 2000 + 1 + 1000)
                .min(right_c + 1000 + 1 + 1000)
                .min(down_c + 1 + 1000)
                .min(left_c + 1000 + 1 + 1000),
        );
    } else if start_v.0 > end_v.0 {
        // up
        nmap.insert(
            Right,
            (up_c + 1 + 1000)
                .min(right_c + 1000 + 1 + 1000)
                .min(down_c + 2000 + 1 + 1000)
                .min(left_c + 1000 + 1 + 1000),
        );
    } else if start_v.1 < end_v.1 {
        // right
        nmap.insert(
            Right,
            (up_c + 1000 + 1)
                .min(right_c + 1)
                .min(down_c + 1000 + 1)
                .min(left_c + 2000 + 1),
        );
    } else if start_v.1 > end_v.1 {
        // left
        nmap.insert(
            Right,
            (up_c + 1000 + 1 + 2000)
                .min(right_c + 2000 + 1 + 2000)
                .min(down_c + 1000 + 1 + 2000)
                .min(left_c + 1 + 2000),
        );
    }
    // down dir
    if start_v.0 < end_v.0 {
        // down
        nmap.insert(
            Down,
            (up_c + 2000 + 1)
                .min(right_c + 1000 + 1)
                .min(down_c + 1)
                .min(left_c + 1000 + 1),
        );
    } else if start_v.0 > end_v.0 {
        // up
        nmap.insert(
            Down,
            (up_c + 1 + 2000)
                .min(right_c + 1000 + 1 + 2000)
                .min(down_c + 2000 + 1 + 2000)
                .min(left_c + 1000 + 1 + 2000),
        );
    } else if start_v.1 < end_v.1 {
        // right
        nmap.insert(
            Down,
            (up_c + 1000 + 1 + 1000)
                .min(right_c + 1 + 1000)
                .min(down_c + 1000 + 1 + 1000)
                .min(left_c + 2000 + 1 + 1000),
        );
    } else if start_v.1 > end_v.1 {
        // left
        nmap.insert(
            Down,
            (up_c + 1000 + 1 + 1000)
                .min(right_c + 2000 + 1 + 1000)
                .min(down_c + 1000 + 1 + 1000)
                .min(left_c + 1 + 1000),
        );
    }
    // left dir
    if start_v.0 < end_v.0 {
        // down
        nmap.insert(
            Left,
            (up_c + 2000 + 1 + 1000)
                .min(right_c + 1000 + 1 + 1000)
                .min(down_c + 1 + 1000)
                .min(left_c + 1000 + 1 + 1000),
        );
    } else if start_v.0 > end_v.0 {
        // up
        nmap.insert(
            Left,
            (up_c + 1 + 1000)
                .min(right_c + 1000 + 1 + 1000)
                .min(down_c + 2000 + 1 + 1000)
                .min(left_c + 1000 + 1 + 1000),
        );
    } else if start_v.1 < end_v.1 {
        // right
        nmap.insert(
            Left,
            (up_c + 1000 + 1 + 2000)
                .min(right_c + 1 + 2000)
                .min(down_c + 1000 + 1 + 2000)
                .min(left_c + 2000 + 1 + 2000),
        );
    } else if start_v.1 > end_v.1 {
        // left
        nmap.insert(
            Left,
            (up_c + 1000 + 1)
                .min(right_c + 2000 + 1)
                .min(down_c + 1000 + 1)
                .min(left_c + 1),
        );
    }

    nmap
}

impl Maze {
    fn generate_all_costs(self: &Self) -> HashMap<(usize, usize), HashMap<Dir, u128>> {
        use Dir::*;
        let mut map = HashMap::new();
        let mut q = HashMap::new();
        for &k in self.transitions.keys() {
            let mut hmap = HashMap::new();
            if k == self.start {
                hmap.insert(Up, 1000);
                hmap.insert(Right, 0);
                hmap.insert(Down, 1000);
                hmap.insert(Left, 1000);
                q.insert(k, 0);
            } else {
                hmap.insert(Up, u128::MAX - 3000);
                hmap.insert(Right, u128::MAX - 3000);
                hmap.insert(Down, u128::MAX - 3000);
                hmap.insert(Left, u128::MAX - 3000);
                q.insert(k, u128::MAX - 3000);
            }
            map.insert(k, hmap);
        }
        while q.len() > 0 {
            let u = q
                .iter()
                .fold(
                    ((0, 0), u128::MAX),
                    |(av, ae), (&v, &e)| {
                        if e < ae {
                            (v, e)
                        } else {
                            (av, ae)
                        }
                    },
                )
                .0;
            q.remove(&u);

            for v in self.transitions.get(&u).unwrap() {
                if q.contains_key(v) {
                    let alt = calculate_trans_costs(&u, v, map.get(&u).unwrap());
                    let ent = map.entry(*v).or_default();
                    let mut nc = u128::MAX;
                    let up_ent = ent.entry(Up).or_default();
                    *up_ent = up_ent.clone().min(*alt.get(&Up).unwrap());
                    nc = nc.min(*up_ent);
                    let right_ent = ent.entry(Right).or_default();
                    *right_ent = right_ent.clone().min(*alt.get(&Right).unwrap());
                    nc = nc.min(*right_ent);
                    let down_ent = ent.entry(Down).or_default();
                    *down_ent = down_ent.clone().min(*alt.get(&Down).unwrap());
                    nc = nc.min(*down_ent);
                    let left_ent = ent.entry(Left).or_default();
                    *left_ent = left_ent.clone().min(*alt.get(&Left).unwrap());
                    nc = nc.min(*left_ent);
                    let q_ent = q.entry(*v).or_default();
                    *q_ent = q_ent.clone().min(nc);
                }
            }
        }
        map
    }

    fn min_cost_to_end(self: &Self) -> u128 {
        let map = self.generate_all_costs();
        *map.get(&self.end).unwrap().values().min().unwrap()
    }
}

fn load_data() -> Maze {
    use std::fs;

    let content = fs::read_to_string("./input/d16_input.txt").expect("could not read file");
    let board = content
        .split_whitespace()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut transitions = HashMap::new();
    for (i, l) in board.iter().enumerate() {
        for (j, &c) in l.iter().enumerate() {
            match c {
                'S' => {
                    start = (i, j);
                    if board[i - 1][j] == '.' || board[i - 1][j] == 'S' || board[i - 1][j] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i - 1, j));
                    }
                    if board[i + 1][j] == '.' || board[i + 1][j] == 'S' || board[i + 1][j] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i + 1, j));
                    }
                    if board[i][j - 1] == '.' || board[i][j - 1] == 'S' || board[i][j - 1] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i, j - 1));
                    }
                    if board[i][j + 1] == '.' || board[i][j + 1] == 'S' || board[i][j + 1] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i, j + 1));
                    }
                }
                'E' => {
                    end = (i, j);
                    if board[i - 1][j] == '.' || board[i - 1][j] == 'S' || board[i - 1][j] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i - 1, j));
                    }
                    if board[i + 1][j] == '.' || board[i + 1][j] == 'S' || board[i + 1][j] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i + 1, j));
                    }
                    if board[i][j - 1] == '.' || board[i][j - 1] == 'S' || board[i][j - 1] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i, j - 1));
                    }
                    if board[i][j + 1] == '.' || board[i][j + 1] == 'S' || board[i][j + 1] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i, j + 1));
                    }
                }
                '.' => {
                    if board[i - 1][j] == '.' || board[i - 1][j] == 'S' || board[i - 1][j] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i - 1, j));
                    }
                    if board[i + 1][j] == '.' || board[i + 1][j] == 'S' || board[i + 1][j] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i + 1, j));
                    }
                    if board[i][j - 1] == '.' || board[i][j - 1] == 'S' || board[i][j - 1] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i, j - 1));
                    }
                    if board[i][j + 1] == '.' || board[i][j + 1] == 'S' || board[i][j + 1] == 'E' {
                        transitions.entry((i, j)).or_insert(vec![]).push((i, j + 1));
                    }
                }
                _ => (),
            }
        }
    }
    Maze {
        transitions,
        start,
        end,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", {
            let maze = load_data();
            maze.min_cost_to_end()
        });
    }

    #[test]
    fn task2() {
        // println!("{}", calculate_price(map_area_new(code_area(load_data()))));
    }
}
