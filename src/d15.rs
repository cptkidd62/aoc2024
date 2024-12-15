#[derive(Debug, Clone, Copy)]
enum WState {
    Wall,
    Box,
    Empty,
}
#[derive(Debug, Clone, Copy)]
enum WState2 {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}
#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

struct Warehouse {
    width: usize,
    height: usize,
    map: Vec<Vec<WState>>,
    robot_x: isize,
    robot_y: isize,
    directions: Vec<Dir>,
}

impl Warehouse {
    fn attempt_to_move_box(
        self: &mut Self,
        x: isize,
        y: isize,
        off_x: isize,
        off_y: isize,
    ) -> bool {
        use WState::*;
        match self.map[(x + off_x) as usize][(y + off_y) as usize] {
            Wall => false,
            Empty => {
                self.map[(x + off_x) as usize][(y + off_y) as usize] = Box;
                self.map[x as usize][y as usize] = Empty;
                true
            }
            Box => {
                if self.attempt_to_move_box(x + off_x, y + off_y, off_x, off_y) {
                    self.map[(x + off_x) as usize][(y + off_y) as usize] = Box;
                    self.map[x as usize][y as usize] = Empty;
                    true
                } else {
                    false
                }
            }
        }
    }
    fn move_robot(self: &mut Self, dir: Dir) {
        use Dir::*;
        use WState::*;
        let (off_x, off_y) = match dir {
            Up => (-1, 0),
            Down => (1, 0),
            Right => (0, 1),
            Left => (0, -1),
        };
        match self.map[(self.robot_x + off_x) as usize][(self.robot_y + off_y) as usize] {
            Wall => (),
            Empty => {
                self.robot_x += off_x;
                self.robot_y += off_y;
            }
            Box => {
                if self.attempt_to_move_box(
                    self.robot_x + off_x,
                    self.robot_y + off_y,
                    off_x,
                    off_y,
                ) {
                    self.robot_x += off_x;
                    self.robot_y += off_y;
                }
            }
        }
    }
    fn coordinate_sum(self: &Self) -> usize {
        use WState::*;
        self.map.iter().enumerate().fold(0, |a, (i, row)| {
            a + row.iter().enumerate().fold(0, |b, (j, s)| {
                b + match s {
                    Box => i * 100 + j,
                    _ => 0,
                }
            })
        })
    }
}

fn load_data() -> Warehouse {
    use std::fs;
    use Dir::*;
    use WState::*;

    let content = fs::read_to_string("./input/d15_input.txt").expect("could not read file");
    let splitted = content.split("\n\n").collect::<Vec<_>>();
    let lines = splitted[0].split_whitespace().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len();
    let mut map = vec![vec![Empty; width]; height];
    let mut robot_x = 0;
    let mut robot_y = 0;
    for (i, &l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => map[i][j] = Wall,
                'O' => map[i][j] = Box,
                '@' => {
                    robot_x = i as isize;
                    robot_y = j as isize;
                }
                _ => (),
            }
        }
    }

    let directions = splitted[1]
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '^' => Up,
            '>' => Right,
            'v' => Down,
            '<' => Left,
            _ => panic!("how?!"),
        })
        .collect::<Vec<_>>();

    Warehouse {
        width,
        height,
        map,
        robot_x,
        robot_y,
        directions,
    }
}

struct Warehouse2 {
    width: usize,
    height: usize,
    map: Vec<Vec<WState2>>,
    robot_x: isize,
    robot_y: isize,
    directions: Vec<Dir>,
}

impl Warehouse2 {
    fn is_box_movable(self: &Self, x: isize, y: isize, off_x: isize, off_y: isize) -> bool {
        use WState2::*;
        match (off_x, off_y) {
            (_, 0) => {
                match (
                    self.map[(x + off_x) as usize][y as usize],
                    self.map[(x + off_x) as usize][(y + 1) as usize],
                ) {
                    (Wall, _) | (_, Wall) => false,
                    (Empty, Empty) => true,
                    (BoxLeft, BoxRight) => self.is_box_movable(x + off_x, y, off_x, off_y),
                    (BoxRight, Empty) => self.is_box_movable(x + off_x, y - 1, off_x, off_y),
                    (BoxRight, BoxLeft) => {
                        self.is_box_movable(x + off_x, y - 1, off_x, off_y)
                            && self.is_box_movable(x + off_x, y + 1, off_x, off_y)
                    }
                    (Empty, BoxLeft) => self.is_box_movable(x + off_x, y + 1, off_x, off_y),
                    _ => panic!("oops"),
                }
            }
            (0, -1) => match self.map[(x) as usize][(y - 1) as usize] {
                Wall => false,
                Empty => true,
                BoxLeft => panic!("wrong box"),
                BoxRight => self.is_box_movable(x, y - 2, off_x, off_y),
            },
            (0, 1) => match self.map[(x) as usize][(y + 2) as usize] {
                Wall => false,
                Empty => true,
                BoxLeft => self.is_box_movable(x, y + 2, off_x, off_y),
                BoxRight => panic!("wrong box"),
            },
            _ => panic!("gone wrong"),
        }
    }
    fn move_box(self: &mut Self, x: isize, y: isize, off_x: isize, off_y: isize) {
        use WState2::*;
        match (off_x, off_y) {
            (_, 0) => {
                match (
                    self.map[(x + off_x) as usize][y as usize],
                    self.map[(x + off_x) as usize][(y + 1) as usize],
                ) {
                    (Empty, Empty) => {
                        self.map[(x + off_x) as usize][y as usize] = BoxLeft;
                        self.map[(x + off_x) as usize][(y + 1) as usize] = BoxRight;
                        self.map[x as usize][y as usize] = Empty;
                        self.map[x as usize][(y + 1) as usize] = Empty;
                    }
                    (BoxLeft, BoxRight) => {
                        self.move_box(x + off_x, y, off_x, off_y);
                        self.map[(x + off_x) as usize][y as usize] = BoxLeft;
                        self.map[(x + off_x) as usize][(y + 1) as usize] = BoxRight;
                        self.map[x as usize][y as usize] = Empty;
                        self.map[x as usize][(y + 1) as usize] = Empty;
                    }
                    (BoxRight, Empty) => {
                        self.move_box(x + off_x, y - 1, off_x, off_y);
                        self.map[(x + off_x) as usize][y as usize] = BoxLeft;
                        self.map[(x + off_x) as usize][(y + 1) as usize] = BoxRight;
                        self.map[x as usize][y as usize] = Empty;
                        self.map[x as usize][(y + 1) as usize] = Empty;
                    }
                    (BoxRight, BoxLeft) => {
                        self.move_box(x + off_x, y - 1, off_x, off_y);
                        self.move_box(x + off_x, y + 1, off_x, off_y);
                        self.map[(x + off_x) as usize][y as usize] = BoxLeft;
                        self.map[(x + off_x) as usize][(y + 1) as usize] = BoxRight;
                        self.map[x as usize][y as usize] = Empty;
                        self.map[x as usize][(y + 1) as usize] = Empty;
                    }
                    (Empty, BoxLeft) => {
                        self.move_box(x + off_x, y + 1, off_x, off_y);
                        self.map[(x + off_x) as usize][y as usize] = BoxLeft;
                        self.map[(x + off_x) as usize][(y + 1) as usize] = BoxRight;
                        self.map[x as usize][y as usize] = Empty;
                        self.map[x as usize][(y + 1) as usize] = Empty;
                    }
                    _ => panic!("oops2"),
                }
            }
            (0, -1) => match self.map[x as usize][(y - 1) as usize] {
                Wall => (),
                Empty => {
                    self.map[x as usize][(y - 1) as usize] = BoxLeft;
                    self.map[x as usize][y as usize] = BoxRight;
                    self.map[x as usize][(y + 1) as usize] = Empty;
                }
                BoxLeft => panic!("aaa"),
                BoxRight => {
                    self.move_box(x, y - 2, off_x, off_y);
                    self.map[x as usize][(y - 1) as usize] = BoxLeft;
                    self.map[x as usize][y as usize] = BoxRight;
                    self.map[x as usize][(y + 1) as usize] = Empty;
                }
            },
            (0, 1) => match self.map[x as usize][(y + 2) as usize] {
                Wall => (),
                Empty => {
                    self.map[x as usize][(y + 1) as usize] = BoxLeft;
                    self.map[x as usize][(y + 2) as usize] = BoxRight;
                    self.map[x as usize][y as usize] = Empty;
                }
                BoxLeft => {
                    self.move_box(x, y + 2, off_x, off_y);
                    self.map[x as usize][(y + 1) as usize] = BoxLeft;
                    self.map[x as usize][(y + 2) as usize] = BoxRight;
                    self.map[x as usize][y as usize] = Empty;
                }
                BoxRight => panic!("aaa"),
            },
            _ => panic!("gone wrong2"),
        }
    }
    fn move_robot(self: &mut Self, dir: Dir) {
        use Dir::*;
        use WState2::*;
        let (off_x, off_y) = match dir {
            Up => (-1, 0),
            Down => (1, 0),
            Right => (0, 1),
            Left => (0, -1),
        };
        match self.map[(self.robot_x + off_x) as usize][(self.robot_y + off_y) as usize] {
            Wall => (),
            Empty => {
                self.robot_x += off_x;
                self.robot_y += off_y;
            }
            BoxLeft => {
                if self.is_box_movable(self.robot_x + off_x, self.robot_y + off_y, off_x, off_y) {
                    self.move_box(self.robot_x + off_x, self.robot_y + off_y, off_x, off_y);
                    self.robot_x += off_x;
                    self.robot_y += off_y;
                }
            }
            BoxRight => {
                if self.is_box_movable(self.robot_x + off_x, self.robot_y + off_y - 1, off_x, off_y)
                {
                    self.move_box(self.robot_x + off_x, self.robot_y + off_y - 1, off_x, off_y);
                    self.robot_x += off_x;
                    self.robot_y += off_y;
                }
            }
        }
    }
    fn coordinate_sum(self: &Self) -> usize {
        use WState2::*;
        self.map.iter().enumerate().fold(0, |a, (i, row)| {
            a + row.iter().enumerate().fold(0, |b, (j, s)| {
                b + match s {
                    BoxLeft => i * 100 + j,
                    _ => 0,
                }
            })
        })
    }
}

fn load_data2() -> Warehouse2 {
    use std::fs;
    use Dir::*;
    use WState2::*;

    let content = fs::read_to_string("./input/d15_input.txt").expect("could not read file");
    let splitted = content.split("\n\n").collect::<Vec<_>>();
    let lines = splitted[0].split_whitespace().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines[0].len() * 2;
    let mut map = vec![vec![Empty; width]; height];
    let mut robot_x = 0;
    let mut robot_y = 0;
    for (i, &l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => {
                    map[i][j * 2] = Wall;
                    map[i][j * 2 + 1] = Wall;
                }
                'O' => {
                    map[i][j * 2] = BoxLeft;
                    map[i][j * 2 + 1] = BoxRight;
                }
                '@' => {
                    robot_x = i as isize;
                    robot_y = (j * 2) as isize;
                }
                _ => (),
            }
        }
    }

    let directions = splitted[1]
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| match c {
            '^' => Up,
            '>' => Right,
            'v' => Down,
            '<' => Left,
            _ => panic!("how?!"),
        })
        .collect::<Vec<_>>();

    Warehouse2 {
        width,
        height,
        map,
        robot_x,
        robot_y,
        directions,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", {
            let mut wh = load_data();
            for dir in wh.directions.clone() {
                wh.move_robot(dir);
            }
            wh.coordinate_sum()
        });
    }

    #[test]
    fn task2() {
        println!("{}", {
            let mut wh = load_data2();
            for dir in wh.directions.clone() {
                wh.move_robot(dir);
            }
            wh.coordinate_sum()
        });
    }
}
