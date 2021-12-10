use std::collections::HashMap;

fn parse_data() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn generate_brackets_map() -> (HashMap<char, char>, HashMap<char, char>) {
    let mut opening_brackets = HashMap::new();
    let mut closing_brackets = HashMap::new();
    opening_brackets.insert('(', ')');
    opening_brackets.insert('{', '}');
    opening_brackets.insert('<', '>');
    opening_brackets.insert('[', ']');
    closing_brackets.insert(')', '(');
    closing_brackets.insert('}', '{');
    closing_brackets.insert(']', '[');
    closing_brackets.insert('>', '<');

    (opening_brackets, closing_brackets)
}

fn line_is_valid(
    line: &String,
    opening_brackets: &HashMap<char, char>,
    closing_brackets: &HashMap<char, char>,
    stack: &mut Vec<char>
) -> Option<char> {
    for ch in line.chars() {
        if opening_brackets.contains_key(&ch) {
            stack.push(ch);
        } else {
            let correct = closing_brackets.get(&ch).unwrap();
            let current = stack.pop().unwrap();
            if *correct != current {
                return Some(ch);
            }
        }
    }

    None
}

fn part1() -> i32 {
    let lines = parse_data();
    let (opening_brackets, closing_brackets) = generate_brackets_map();
    let mut ans = 0;

    for line in lines.iter() {
        let mut stack: Vec<char> = Vec::new();
        if let Some(ch) = line_is_valid(line, &opening_brackets, &closing_brackets, &mut stack) {
            match ch {
                ')' => ans += 3,
                ']' => ans += 57,
                '}' => ans += 1197,
                '>' => ans += 25137,
                _ => (),
            }
        }
    }

    ans
}

fn part2() -> i128 {
    let lines = parse_data();

    let (opening_brackets, closing_brackets) = generate_brackets_map();
    let mut line_values = Vec::new();

    for line in lines.iter() {
        let mut stack: Vec<char> = Vec::new();

        if let Some(_) = line_is_valid(line, &opening_brackets, &closing_brackets, &mut stack) {
            continue;
        }

        let val = stack.iter().rev().fold(0, |score, c| {
            let correct = opening_brackets.get(c).unwrap();
            match correct {
                ')' => score * 5 + 1,
                ']' => score * 5 + 2,
                '}' => score * 5 + 3,
                '>' => score * 5 + 4,
                _ => unreachable!(),
            }
        });

        line_values.push(val);

    }

    line_values.sort();

    let mid = (line_values.len() / 2) as usize;

    line_values[mid]
}

fn main() {
    println!("Part1 answer is: {}", part1());
    println!("Part2 answer is: {}", part2());
}