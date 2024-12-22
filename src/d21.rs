fn load_data() -> Vec<String> {
    use std::fs;

    let content = fs::read_to_string("./input/d21_input.txt").expect("could not read file");
    content
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
}

fn find_optimal_numeric(goal: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!("{}", 0);
    }

    #[test]
    fn task2() {
        // println!("{}", find_most_bananas());
    }
}
