fn load_data() -> Vec<u128> {
    use std::fs;

    let content = fs::read_to_string("./input/d22_input.txt").expect("could not read file");
    content
        .split_whitespace()
        .map(|s| s.parse::<u128>().unwrap())
        .collect::<Vec<_>>()
}

fn mix(x: u128, y: u128) -> u128 {
    x ^ y
}

fn prune(x: u128) -> u128 {
    x % 16777216
}

fn get_next_secret(x: u128) -> u128 {
    let mut r = prune(mix(x, x * 64));
    r = prune(mix(r, r / 32));
    r = prune(mix(r, r * 2048));
    r
}

fn get_nth_next_secret(x: u128, n: usize) -> u128 {
    let mut r = x;
    for _ in 0..n {
        r = get_next_secret(r);
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", load_data().iter().map(|&x| get_nth_next_secret(x, 2000)).sum::<u128>());
    }

    #[test]
    fn task2() {
        // println!("{}", {
        //     let sta = load_data();
        //     let mut c = 0;
        //     for i in 1_000_000..usize::MAX {
        //         let mut st = sta.clone();
        //         st.reg_a = i as u32;
        //         while !st.is_done() && st.is_output_fitting() {
        //             st.do_instruction();
        //         }
        //         if st.output == st.program {
        //             c = i;
        //             break;
        //         }
        //     }
        //     c
        // });
    }
}
