use std::{
    collections::{HashMap, HashSet},
    iter,
};

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

fn form_clique(
    clique: HashSet<String>,
    map: &HashMap<String, HashSet<String>>,
    to_add: HashSet<String>,
) -> HashSet<String> {
    to_add
        .iter()
        .filter_map(|v| match map.get(v) {
            Some(arr) => {
                if clique.iter().all(|a| arr.contains(a)) {
                    let mut clique1 = clique.clone();
                    clique1.insert(v.clone());
                    let mut to_add1 = to_add.clone();
                    to_add1.remove(v);
                    Some(form_clique(clique1, map, to_add1))
                } else {
                    None
                }
            }
            None => None,
        })
        .fold(clique.clone(), |a, c| if c.len() > a.len() { c } else { a })
}

fn find_all_cliques(map: HashMap<String, HashSet<String>>) -> Vec<HashSet<String>> {
    let mut cliques = vec![];
    let map1 = map.clone();
    for (k, v) in map {
        cliques.push(form_clique(HashSet::from([k; 1]), &map1, v));
    }
    cliques
}

fn get_pwd_to_largest(clique: HashSet<String>) -> String {
    let mut comps = clique.iter().map(|s| s.clone()).collect::<Vec<_>>();
    comps.sort();
    comps.join(",")
}

fn find_max_clique() -> HashSet<String> {
    let clusters = pairs_to_clusters(load_data());
    let triples = triples_in_clusters(clusters.clone());
    let comps = clusters
        .keys()
        .map(|k| k.to_string())
        .collect::<HashSet<_>>();
    let mut cliques = vec![];
    for (a, b, c) in triples {
        let mut s = HashSet::new();
        s.insert(a);
        s.insert(b);
        s.insert(c);
        cliques.push(s);
    }
    let mut cliquesn = vec![];
    for _ in 0..20 {
        for c in cliques.iter() {
            let to_try = comps.difference(&c).collect::<HashSet<_>>();
            for s in to_try {
                let arr = clusters.get(s).unwrap();
                if c.iter().all(|f| f == s || arr.contains(f)) {
                    let mut c1 = c.clone();
                    c1.insert(s.clone());
                    cliquesn.push(c1);
                }
            }
        }
        cliquesn.dedup();
        if cliquesn.len() == 0 {
            return cliques[0].clone();
        }
        cliques = cliquesn.clone();
        cliquesn.clear();
    }
    HashSet::new()
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
        println!("{:?}", find_max_clique());
    }
}
