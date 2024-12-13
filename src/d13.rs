use nom::{
    bytes::complete::tag,
    character::complete::{i128, multispace0},
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Machine {
    button_a: (i128, i128),
    button_b: (i128, i128),
    prize_loc: (i128, i128),
}

enum EqSol {
    Solved(i128, i128),
    Ambiguous,
    Unsolvable,
}

// cramer
fn solve_eq(c1: (i128, i128), c2: (i128, i128), c3: (i128, i128)) -> EqSol {
    let d = c1.0 * c2.1 - c1.1 * c2.0;
    let d1 = c3.0 * c2.1 - c3.1 * c2.0;
    let d2 = c1.0 * c3.1 - c1.1 * c3.0;
    if d == 0 && d1 == 0 && d2 == 0 {
        EqSol::Ambiguous
    } else if d != 0
        && d1 % d == 0
        && d2 % d == 0
        && d1 / d > 0
        && d2 / d > 0
        && d1 / d < 101
        && d2 / d < 101
    {
        EqSol::Solved(d1 / d, d2 / d)
    } else {
        EqSol::Unsolvable
    }
}

fn solve_eq_nl(c1: (i128, i128), c2: (i128, i128), c3: (i128, i128)) -> EqSol {
    let d = c1.0 * c2.1 - c1.1 * c2.0;
    let d1 = c3.0 * c2.1 - c3.1 * c2.0;
    let d2 = c1.0 * c3.1 - c1.1 * c3.0;
    if d == 0 && d1 == 0 && d2 == 0 {
        EqSol::Ambiguous
    } else if d != 0 && d1 % d == 0 && d2 % d == 0 && d1 / d > 0 && d2 / d > 0 {
        EqSol::Solved(d1 / d, d2 / d)
    } else {
        EqSol::Unsolvable
    }
}

impl Machine {
    fn min_token(self: &Self) -> i128 {
        match solve_eq(self.button_a, self.button_b, self.prize_loc) {
            EqSol::Solved(x, y) => x * 3 + y,
            _ => 0,
        }
    }
    fn min_token_nl(self: &Self) -> i128 {
        let nloc = (
            self.prize_loc.0 + 10000000000000,
            self.prize_loc.1 + 10000000000000,
        );
        match solve_eq_nl(self.button_a, self.button_b, nloc) {
            EqSol::Solved(x, y) => x * 3 + y,
            _ => 0,
        }
    }
}

fn parse_machine(input: &str) -> IResult<&str, ((i128, i128), (i128, i128), (i128, i128))> {
    tuple((
        pair(
            preceded(tag("Button A: X+"), i128),
            delimited(tag(", Y+"), i128, multispace0),
        ),
        pair(
            preceded(tag("Button B: X+"), i128),
            delimited(tag(", Y+"), i128, multispace0),
        ),
        pair(
            preceded(tag("Prize: X="), i128),
            delimited(tag(", Y="), i128, multispace0),
        ),
    ))(input)
}

fn load_data() -> Vec<Machine> {
    use std::fs;

    let content = fs::read_to_string("./input/d13_input.txt").expect("could not read file");
    let machines = content.split("\n\n").collect::<Vec<_>>();
    machines
        .iter()
        .map(|&s| {
            let (_, (button_a, button_b, prize_loc)) = parse_machine(s).expect("parse error");
            Machine {
                button_a,
                button_b,
                prize_loc,
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", load_data().iter().fold(0, |a, m| m.min_token() + a));
    }

    #[test]
    fn task2() {
        println!(
            "{}",
            load_data().iter().fold(0, |a, m| m.min_token_nl() + a)
        );
    }
}
