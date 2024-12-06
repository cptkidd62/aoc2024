use std::collections::HashSet;

struct MappedArea {
    width: isize,
    height: isize,
    obstacles: Vec<Vec<bool>>,
    guard_position: Option<(isize, isize)>,
    guard_dir: Direction,
}

fn guard_position_ok((x, y): (isize, isize), w: isize, h: isize) -> bool {
    x >= 0 && x < h && y >= 0 && y < w
}

impl MappedArea {
    fn get_move_trail(&self) -> Vec<(isize, isize)> {
        let mut trail = vec![];
        if self.guard_position.is_some() {
            let (mut x, mut y) = self.guard_position.unwrap();
            let mut dir = self.guard_dir;
            while guard_position_ok((x, y), self.width, self.height) {
                trail.push((x, y));
                let (nx, ny) = match dir {
                    Direction::UP => (x - 1, y),
                    Direction::RIGHT => (x, y + 1),
                    Direction::DOWN => (x + 1, y),
                    Direction::LEFT => (x, y - 1),
                };
                if !guard_position_ok((nx, ny), self.width, self.height)
                    || !self.obstacles[nx as usize][ny as usize]
                {
                    (x, y) = (nx, ny);
                } else {
                    dir = match dir {
                        Direction::UP => Direction::RIGHT,
                        Direction::RIGHT => Direction::DOWN,
                        Direction::DOWN => Direction::LEFT,
                        Direction::LEFT => Direction::UP,
                    };
                }
            }
        }
        trail.sort();
        trail.dedup();
        trail
    }

    fn detect_loop_for_area(&self, a: isize, b: isize) -> bool {
        let mut trail = HashSet::new();
        if self.guard_position.is_some() {
            let (mut x, mut y) = self.guard_position.unwrap();
            let mut dir = self.guard_dir;
            while guard_position_ok((x, y), self.width, self.height) {
                if trail.contains(&(x, y, dir)) {
                    return true;
                }
                trail.insert((x, y, dir));
                let (nx, ny) = match dir {
                    Direction::UP => (x - 1, y),
                    Direction::RIGHT => (x, y + 1),
                    Direction::DOWN => (x + 1, y),
                    Direction::LEFT => (x, y - 1),
                };
                if !guard_position_ok((nx, ny), self.width, self.height)
                    || !(self.obstacles[nx as usize][ny as usize] || nx == a && ny == b)
                {
                    (x, y) = (nx, ny);
                } else {
                    dir = match dir {
                        Direction::UP => Direction::RIGHT,
                        Direction::RIGHT => Direction::DOWN,
                        Direction::DOWN => Direction::LEFT,
                        Direction::LEFT => Direction::UP,
                    };
                }
            }
        }
        false
    }

    fn count_possible_loops(&self) -> i32 {
        (0..self.height).fold(0, |a, i| {
            a + (0..self.width).fold(0, |b, j| {
                b + if self.guard_position.unwrap_or((-1, -1)) != (i, j) {
                    if self.detect_loop_for_area(i, j) {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    UP = 0,
    RIGHT = 1,
    DOWN = 2,
    LEFT = 3,
}

fn load_data() -> MappedArea {
    use std::fs;

    let content = fs::read_to_string("./input/d06_input.txt").expect("could not read file");
    let splitted = content.split_whitespace().collect::<Vec<_>>();
    let width = splitted[0].len() as isize;
    let height = splitted.len() as isize;
    let mut obstacles = vec![vec![false; width as usize]; height as usize];
    let mut guard_position = None;
    let mut guard_dir = Direction::UP;
    splitted.iter().enumerate().for_each(|(i, &l)| {
        l.chars().enumerate().for_each(|(j, c)| match c {
            '#' => obstacles[i][j] = true,
            '^' => {
                guard_position = Some((i as isize, j as isize));
                guard_dir = Direction::UP;
            }
            '>' => {
                guard_position = Some((i as isize, j as isize));
                guard_dir = Direction::RIGHT;
            }
            'v' => {
                guard_position = Some((i as isize, j as isize));
                guard_dir = Direction::DOWN;
            }
            '<' => {
                guard_position = Some((i as isize, j as isize));
                guard_dir = Direction::LEFT;
            }
            _ => (),
        });
    });
    MappedArea {
        width,
        height,
        obstacles,
        guard_position,
        guard_dir,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", load_data().get_move_trail().len());
    }

    #[test]
    fn task2() {
        println!("{}", load_data().count_possible_loops());
    }
}
