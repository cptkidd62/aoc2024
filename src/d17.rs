use std::ops::Shl;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, u32},
    multi::many0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct State {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    program: Vec<u32>,
    cursor: usize,
    output: Vec<u32>,
}

impl State {
    fn is_done(self: &Self) -> bool {
        self.cursor >= self.program.len() - 1
    }

    fn combo_of_x(self: &Self, x: u32) -> u32 {
        match x {
            0 | 1 | 2 | 3 => x,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("combo panic"),
        }
    }

    fn do_instruction(self: &mut Self) {
        let op = self.program[self.cursor];
        let arg = self.program[self.cursor + 1];
        match op {
            0 => {
                self.reg_a = self.reg_a / (1u32.shl(self.combo_of_x(arg)));
                self.cursor += 2;
            }
            1 => {
                self.reg_b = self.reg_b ^ arg;
                self.cursor += 2;
            }
            2 => {
                self.reg_b = self.combo_of_x(arg) % 8;
                self.cursor += 2;
            }
            3 => {
                if self.reg_a != 0 {
                    self.cursor = arg as usize;
                } else {
                    self.cursor += 2;
                }
            }
            4 => {
                self.reg_b = self.reg_b ^ self.reg_c;
                self.cursor += 2;
            }
            5 => {
                self.output.push(self.combo_of_x(arg) % 8);
                self.cursor += 2;
            }
            6 => {
                self.reg_b = self.reg_a / (1u32.shl(self.combo_of_x(arg)));
                self.cursor += 2;
            }
            7 => {
                self.reg_c = self.reg_a / (1u32.shl(self.combo_of_x(arg)));
                self.cursor += 2;
            }
            _ => panic!("oopsie"),
        }
    }

    fn is_output_fitting(self: &Self) -> bool {
        self.output.len() <= self.program.len()
            && self
                .output
                .iter()
                .zip(self.program.iter())
                .all(|(&x, &y)| x == y)
    }

    fn out_string(self: &Self) -> String {
        self.output
            .iter()
            .map(|&x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn parse_state(input: &str) -> IResult<&str, State> {
    let (_, (reg_a, reg_b, reg_c, program)) = tuple((
        delimited(tag("Register A: "), u32, multispace0),
        delimited(tag("Register B: "), u32, multispace0),
        delimited(tag("Register C: "), u32, multispace0),
        delimited(
            tag("Program:"),
            many0(preceded(alt((multispace1, tag(","))), u32)),
            multispace0,
        ),
    ))(input)?;
    Ok((
        "",
        State {
            reg_a,
            reg_b,
            reg_c,
            program,
            cursor: 0,
            output: vec![],
        },
    ))
}

fn load_data() -> State {
    use std::fs;

    let content = fs::read_to_string("./input/d17_input.txt").expect("could not read file");
    let st = parse_state(&content).unwrap().1;
    println!("{:?}", st);
    st
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", {
            let mut st = load_data();
            while !st.is_done() {
                st.do_instruction();
            }
            st.out_string()
        });
    }

    #[test]
    fn task2() {
        println!("{}", {
            let sta = load_data();
            let mut c = 0;
            for i in 1_000_000..usize::MAX {
                let mut st = sta.clone();
                st.reg_a = i as u32;
                while !st.is_done() && st.is_output_fitting() {
                    st.do_instruction();
                }
                if st.output == st.program {
                    c = i;
                    break;
                }
            }
            c
        });
    }
}
