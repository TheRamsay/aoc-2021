use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug)]
struct Graph {
    _graph: HashMap<String, Vec<String>>,
}

impl Graph {
    fn from_file(file_name: &str) -> Self {
        let mut _graph: HashMap<String, Vec<String>> = HashMap::new();
        for line in read_to_string(file_name).unwrap().lines() {
            let split: Vec<String> = line.split("-").map(|e| e.to_string()).collect();
            let start = split.first().unwrap();
            let end = split.last().unwrap();

            _graph
                .entry(start.to_string())
                .or_insert(vec![])
                .push(end.to_string());

            _graph
                .entry(end.to_string())
                .or_insert(vec![])
                .push(start.to_string());
        }

        Graph { _graph }
    }

    fn day1(&self, node: String, end: &str, visited: &mut HashSet<String>) -> i32 {
        if node == end {
            return 1;
        }

        let mut ans = 0;

        for nei in self._graph.get(&node).unwrap() {
            if is_lowercase(nei) & visited.contains(nei) {
                continue;
            }

            let mut x = visited.clone();
            x.insert(node.to_string());
            ans += self.day1(nei.to_string(), end, &mut x);
        }

        ans
    }

    fn day2(&self, node: String, end: &str, visited: &mut HashSet<String>, can_twice: bool) -> i32 {
        if node == end {
            return 1;
        }

        let mut ans = 0;

        for nei in self._graph.get(&node).unwrap() {
            let mut x = visited.clone();
            x.insert(node.to_string());
            if is_lowercase(nei) {
                if !visited.contains(nei) {
                    ans += self.day2(nei.to_string(), end, &mut x, can_twice);
                } else if can_twice & !vec!["start", "end"].contains(&nei.as_str()) {
                    ans += self.day2(nei.to_string(), end, &mut x, false);
                }
            } else {
                ans += self.day2(nei.to_string(), end, &mut x, can_twice);
            }
        }

        ans
    }
}

fn is_lowercase(s: &String) -> bool {
    s.chars().all(|ch| {
        let d = ch as u8;
        (d >= 97) & (d <= 122)
    })
}

fn main() {
    let before = Instant::now();
    let graph = Graph::from_file("./input.txt");

    let mut visited = HashSet::new();
    visited.insert(String::from("start"));
    println!(
        "{:?}",
        graph.day1(String::from("start"), "end", &mut visited)
    );

    let mut visited = HashSet::new();
    visited.insert(String::from("start"));
    println!(
        "{:?}",
        graph.day2(String::from("start"), "end", &mut visited, true)
    );
    println!("Elapsed time: {:.2?}", before.elapsed())
}

// Solution is really slow