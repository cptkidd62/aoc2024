use std::{collections::HashMap, ops::Shl};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, u128},
    multi::many0,
    sequence::{separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone)]
enum Exp {
    Const(u128),
    And(String, String),
    Xor(String, String),
    Or(String, String),
}

fn parse_const(input: &str) -> IResult<&str, (String, Exp)> {
    let (output, (name, val)) =
        terminated(separated_pair(alphanumeric1, tag(": "), u128), multispace0)(input)?;
    Ok((output, (name.to_string(), Exp::Const(val))))
}

fn parse_and(input: &str) -> IResult<&str, (String, Exp)> {
    let (output, ((e1, e2), res)) = terminated(
        separated_pair(
            separated_pair(alphanumeric1, tag(" AND "), alphanumeric1),
            tag(" -> "),
            alphanumeric1,
        ),
        multispace0,
    )(input)?;
    Ok((
        output,
        (res.to_string(), Exp::And(e1.to_string(), e2.to_string())),
    ))
}

fn parse_xor(input: &str) -> IResult<&str, (String, Exp)> {
    let (output, ((e1, e2), res)) = terminated(
        separated_pair(
            separated_pair(alphanumeric1, tag(" XOR "), alphanumeric1),
            tag(" -> "),
            alphanumeric1,
        ),
        multispace0,
    )(input)?;
    Ok((
        output,
        (res.to_string(), Exp::Xor(e1.to_string(), e2.to_string())),
    ))
}

fn parse_or(input: &str) -> IResult<&str, (String, Exp)> {
    let (output, ((e1, e2), res)) = terminated(
        separated_pair(
            separated_pair(alphanumeric1, tag(" OR "), alphanumeric1),
            tag(" -> "),
            alphanumeric1,
        ),
        multispace0,
    )(input)?;
    Ok((
        output,
        (res.to_string(), Exp::Or(e1.to_string(), e2.to_string())),
    ))
}

fn parse(input: &str) -> Vec<(String, Exp)> {
    let (_, v) = many0(alt((parse_const, parse_and, parse_xor, parse_or)))(input).unwrap();
    v
}

fn load_data() -> HashMap<String, Exp> {
    use std::fs;

    let content = fs::read_to_string("./input/d24_input.txt").expect("could not read file");
    let vec = parse(&content);
    let mut map = HashMap::new();
    for (k, v) in vec {
        map.insert(k, v);
    }
    map
}

fn eval(name: &String, env: &mut HashMap<String, Exp>) -> u128 {
    use Exp::*;
    let exp = env.entry(name.clone()).or_insert(Const(0)).clone();
    match exp {
        Const(v) => v,
        And(e1, e2) => {
            let v = eval(&e1, env) & eval(&e2, env);
            let entry = env.entry(name.clone()).or_insert(Const(0));
            *entry = Const(v);
            v
        }
        Xor(e1, e2) => {
            let v = eval(&e1, env) ^ eval(&e2, env);
            let entry = env.entry(name.clone()).or_insert(Const(0));
            *entry = Const(v);
            v
        }
        Or(e1, e2) => {
            let v = eval(&e1, env) | eval(&e2, env);
            let entry = env.entry(name.clone()).or_insert(Const(0));
            *entry = Const(v);
            v
        }
    }
}

fn eval_zs(env: &mut HashMap<String, Exp>) -> u128 {
    let e = env.clone();
    let mut keys = e.keys().filter(|&k| &k[0..1] == "z").collect::<Vec<_>>();
    keys.sort();
    let mut res = 0;
    for (i, &key) in keys.iter().enumerate() {
        res += eval(key, env).shl(i)
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", eval_zs(&mut load_data()));
    }

    #[test]
    fn task2() {
        // println!("{}", find_most_bananas());
    }
}
