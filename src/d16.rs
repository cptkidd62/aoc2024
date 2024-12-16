use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct State {
    deer: (usize, usize),
    visited: HashSet<(usize, usize)>,
    score: usize,
    direction: Dir,
}

impl State {
    fn is_valid_move(self: &Self, n_loc: (usize, usize)) -> bool {
        !self.visited.contains(&n_loc)
    }
    fn do_move(self: &Self, n_loc: (usize, usize)) -> Self {
        let mut n_visited = self.visited.clone();
        n_visited.insert(n_loc);
        let mut score_inc = 1;
        let mut n_dir = self.direction;
        if n_loc.0 > self.deer.0 && (n_dir == Dir::Left || n_dir == Dir::Right) {
            n_dir = Dir::Down;
            score_inc += 1000;
        }
        if n_loc.0 < self.deer.0 && (n_dir == Dir::Left || n_dir == Dir::Right) {
            n_dir = Dir::Up;
            score_inc += 1000;
        }
        if n_loc.1 > self.deer.1 && (n_dir == Dir::Up || n_dir == Dir::Down) {
            n_dir = Dir::Right;
            score_inc += 1000;
        }
        if n_loc.1 < self.deer.1 && (n_dir == Dir::Up || n_dir == Dir::Down) {
            n_dir = Dir::Left;
            score_inc += 1000;
        }
        Self {
            deer: n_loc,
            visited: n_visited,
            score: self.score + score_inc,
            direction: n_dir,
        }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    transitions: HashMap<(usize, usize), Vec<(usize, usize)>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    fn is_in_end(self: &Self, deer: (usize, usize)) -> bool {
        deer == self.end
    }

    fn find_min(self: &Self, state: State) -> usize {
        if self.is_in_end(state.deer) {
            state.score
        } else {
            let transitions = self.transitions.get(&state.deer).unwrap().clone();
            let possible_moves = transitions
                .iter()
                .filter(|&m| state.is_valid_move(*m))
                .collect::<Vec<_>>();
            if possible_moves.len() == 0 {
                usize::MAX
            } else {
                possible_moves
                    .iter()
                    .map(|&m| self.find_min(state.do_move(*m)))
                    .min()
                    .unwrap()
            }
        }
    }

    fn generate_start_state(self: &Self) -> State {
        let mut n_vis = HashSet::new();
        n_vis.insert(self.start);
        State {
            deer: self.start,
            visited: n_vis,
            score: 0,
            direction: Dir::Right,
        }
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
            // println!("{:?}", maze);
            maze.find_min(maze.generate_start_state())
        });
    }

    #[test]
    fn task2() {
        // println!("{}", calculate_price(map_area_new(code_area(load_data()))));
    }
}
