use std::collections::HashMap;

fn parse_data() -> (String, HashMap<String, String>) {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();
    let mut rules: HashMap<String, String> = HashMap::new();
    let template = (*lines.first().unwrap()).to_string();

    for i in 2..lines.len() {
        let split: Vec<String> = lines[i].split(" -> ").map(|e| e.to_string()).collect();
        let a = split.first().unwrap();
        let b = split.last().unwrap();
        rules.insert(a.to_string(), b.to_string());
    }

    (template, rules)
}

fn evolve(base: &String, rules: &HashMap<String, String>, n: i128) -> HashMap<String, i128> {
    let mut pairs_map: HashMap<String, i128> = HashMap::new();
    let mut last_char = '\0';
    for ch in base.chars() {
        let mut pair = String::from(last_char);
        pair.push(ch);

        if pair.contains("\0") {
            last_char = ch;
            continue;
        }
        *pairs_map.entry(pair).or_insert(0) += 1;

        last_char = ch;
    }

    for _ in 0..n {
        let mut tmp: HashMap<String, i128> = HashMap::new();
        for (pair, n) in pairs_map.iter() {
            if let Some(val) = rules.get(pair) {
                let first = pair.chars().next().unwrap();
                let second = pair.chars().nth(1).unwrap();
                let mut x = String::from(first);
                x.push_str(val);
                *tmp.entry(x).or_insert(0) += n;
                let mut x = String::from(val);
                x.push(second);
                *tmp.entry(x).or_insert(0) += n;
            }
        }

        pairs_map = tmp;
    }

    pairs_map
}

fn count_occurence_in_map(map: &HashMap<String, i128>) -> HashMap<String, i128> {
    let mut occ: HashMap<String, i128> = HashMap::new();
    for (k, v) in map.iter() {
        let second = k.chars().next().unwrap();
        *occ.entry(second.to_string()).or_insert(0) += v;
    }

    occ
}

fn solve(n: i32) -> i128 {
    let (template, rules) = parse_data();
    let evolved = evolve(&template, &rules, n as i128);
    let occ = count_occurence_in_map(&evolved);

    let mut sorted: Vec<&i128> = occ.values().collect();
    sorted.sort_by(|a, b| b.cmp(a));

    let first = *sorted.first().unwrap();
    let last = sorted.last().unwrap();

    (first - **last) - 1
}

fn main() {
    println!("Part1 answer is: {}", solve(10));
    println!("Part2 answer is: {}", solve(40));
}
