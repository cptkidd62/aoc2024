fn load_data() -> Vec<(i128, Vec<i128>)> {
    use std::fs;

    let content = fs::read_to_string("./input/d07_input.txt").expect("could not read file");
    content
        .split("\n")
        .map(|l| {
            let s = l.split(": ").collect::<Vec<_>>();
            (
                s[0].parse::<i128>().unwrap(),
                s[1].split(" ")
                    .map(|x| x.parse::<i128>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>()
}

fn check_line(acc: i128, (goal, v): (i128, &[i128])) -> bool {
    if acc > goal || v.len() == 0 {
        acc == goal
    } else {
        check_line(acc * v[0], (goal, &v[1..])) || check_line(acc + v[0], (goal, &v[1..]))
    }
}

fn concat_ints(left: i128, right: i128) -> i128 {
    let mult = 10i128.pow(right.checked_ilog10().unwrap_or(0) + 1);
    left * mult + right
}

fn check_line_w_concat(acc: i128, (goal, v): (i128, &[i128])) -> bool {
    if acc > goal || v.len() == 0 {
        acc == goal
    } else {
        check_line_w_concat(concat_ints(acc, v[0]), (goal, &v[1..]))
            || check_line_w_concat(acc * v[0], (goal, &v[1..]))
            || check_line_w_concat(acc + v[0], (goal, &v[1..]))
    }
}

fn calibration_sum(fun: &dyn Fn(i128, (i128, &[i128])) -> bool) -> i128 {
    let data = load_data();
    data.iter()
        .filter(|(g, v)| fun(v[0], (*g, &v[1..])))
        .fold(0, |a, (g, _)| a + g)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", calibration_sum(&check_line));
    }

    #[test]
    fn task2() {
        println!("{}", calibration_sum(&check_line_w_concat));
    }
}
