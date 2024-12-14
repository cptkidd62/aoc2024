use nom::{
    bytes::complete::tag,
    character::complete::i32,
    sequence::{preceded, tuple},
    IResult,
};

struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn move_inside(self: &mut Self, width: i32, height: i32) {
        self.px = (self.px + width + self.vx) % width;
        self.py = (self.py + height + self.vy) % height;
    }
}

struct Bathroom {
    width: i32,
    height: i32,
    robots: Vec<Robot>,
}

impl Bathroom {
    fn simulate_n(self: &mut Self, n: usize) {
        for _ in 0..n {
            for robot in self.robots.iter_mut() {
                robot.move_inside(self.width, self.height);
            }
        }
    }

    fn calculate_security_factor(self: &Self) -> usize {
        let mx = self.width / 2;
        let my = self.height / 2;
        let (mut lt, mut ld, mut rt, mut rd) = (0usize, 0usize, 0usize, 0usize);
        self.robots.iter().for_each(|robot| {
            if robot.px < mx && robot.py < my {
                lt += 1;
            } else if robot.px < mx && robot.py > my {
                ld += 1;
            } else if robot.px > mx && robot.py < my {
                rt += 1;
            } else if robot.px > mx && robot.py > my {
                rd += 1;
            }
        });
        lt * ld * rt * rd
    }

    fn to_file_pic(self: &Self, fname: &str) {
        let mut v = vec![false; (self.width * self.height) as usize];
        self.robots
            .iter()
            .for_each(|robot| v[(robot.py * self.width + robot.px) as usize] = true);

        let mut data = String::new();
        data.push_str("P3\n");
        data.push_str(&format!("{} {} 255\n", self.width, self.height));
        for e in v {
            let c = if e { 255 } else { 0 };
            data.push_str(&format!("{c} {c} {c}\n"));
        }
        std::fs::write(fname, &data).expect("Unable to write to file")
    }
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (_, (px, py, vx, vy)) = tuple((
        preceded(tag("p="), i32),
        preceded(tag(","), i32),
        preceded(tag(" v="), i32),
        preceded(tag(","), i32),
    ))(input)?;
    Ok(("", Robot { px, py, vx, vy }))
}

fn load_data() -> Vec<Robot> {
    use std::fs;

    let content = fs::read_to_string("./input/d14_input.txt").expect("could not read file");
    content
        .split("\n")
        .map(|l| {
            let (_, rob) = parse_robot(l).unwrap();
            rob
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", {
            let mut bath = Bathroom {
                width: 101,
                height: 103,
                robots: load_data(),
            };
            bath.simulate_n(100);
            bath.calculate_security_factor()
        });
    }

    // 7083 is my solution
    #[test]
    fn task2() {
        let mut bath = Bathroom {
            width: 101,
            height: 103,
            robots: load_data(),
        };
        for i in 0..10000 {
            bath.to_file_pic(format!("./d14pics/{i:06}sec.ppm").as_str());
            bath.simulate_n(1);
        }
    }
}
