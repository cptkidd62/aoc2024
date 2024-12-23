use std::collections::{HashMap, HashSet};

fn load_data() -> Vec<(String, String)> {
    use std::fs;

    let content = fs::read_to_string("./input/d23_input.txt").expect("could not read file");
    content
        .split_whitespace()
        .map(|s| {
            let spl = s.split("-").collect::<Vec<_>>();
            (spl[0].to_string(), spl[1].to_string())
        })
        .collect()
}

fn pairs_to_clusters(v: Vec<(String, String)>) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();
    for (a, b) in v {
        let e1 = map.entry(a.clone()).or_insert(HashSet::new());
        _ = e1.insert(b.clone());
        let e2 = map.entry(b.clone()).or_insert(HashSet::new());
        _ = e2.insert(a.clone());
    }
    map
}

fn find_in_three(
    s: &String,
    map: &HashMap<String, HashSet<String>>,
) -> Vec<(String, String, String)> {
    let mut sols = vec![];
    match map.get(s) {
        Some(n) => {
            for two in n {
                match map.get(two) {
                    Some(m) => {
                        for three in m {
                            match map.get(three) {
                                Some(p) => {
                                    if *three != *s && p.contains(s) {
                                        sols.push((s.clone(), two.clone(), three.clone()));
                                    }
                                }
                                None => (),
                            }
                        }
                    }
                    None => (),
                }
            }
        }
        None => (),
    }
    sols
}

fn triples_in_clusters(map: HashMap<String, HashSet<String>>) -> HashSet<(String, String, String)> {
    let mut set = HashSet::new();
    for k in map.keys() {
        let sols = find_in_three(k, &map);
        for (a, b, c) in sols {
            if !(set.contains(&(a.clone(), b.clone(), c.clone()))
                || set.contains(&(a.clone(), c.clone(), b.clone()))
                || set.contains(&(b.clone(), a.clone(), c.clone()))
                || set.contains(&(b.clone(), c.clone(), a.clone()))
                || set.contains(&(c.clone(), a.clone(), b.clone()))
                || set.contains(&(c.clone(), b.clone(), a.clone())))
            {
                set.insert((a, b, c));
            }
        }
    }
    set
}

fn count_ts(set: HashSet<(String, String, String)>) -> usize {
    set.iter()
        .filter(|(a, b, c)| &a[0..1] == "t" || &b[0..1] == "t" || &c[0..1] == "t")
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task1() {
        println!(
            "{}",
            count_ts(triples_in_clusters(pairs_to_clusters(load_data())))
        );
    }

    #[test]
    fn task2() {
        // println!("{}", find_most_bananas());
    }
}
