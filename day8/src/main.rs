use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}

fn parse_data() -> Vec<(Vec<String>, Vec<String>)> {
    let mut data: Vec<(Vec<String>, Vec<String>)> = Vec::new();
    for line in include_str!("../input.txt").lines() {
        let split = line.split(" | ").collect::<Vec<&str>>();
        let patterns = split[0]
            .split(" ")
            .map(|el| el.to_string())
            .collect::<Vec<String>>();
        let output = split[1]
            .split(" ")
            .map(|el| el.to_string())
            .collect::<Vec<String>>();
        data.push((patterns, output));
    }

    data
}

fn part1() -> i32 {
    let data = parse_data();
    let mut ans = 0;
    for line in data.iter() {
        for digit in line.1.iter() {
            match digit.len() {
                2 => ans += 1,
                3 => ans += 1,
                4 => ans += 1,
                7 => ans += 1,
                _ => (),
            }
        }
    }
    ans
}
fn part2() -> i32 {
    let data = parse_data();
    let mut ans = 0;
    for (input, output) in data.iter() {
        let mut digits: HashMap<&str, &String> = HashMap::new();
        let mut sorted_input: Vec<String> = input.clone();
        sorted_input.sort_by(|a, b| {
            if a.len() > b.len() {
                Ordering::Greater
            } else if a.len() == b.len() {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        });

        digits.insert("1", &sorted_input[0]);
        digits.insert("4", &sorted_input[2]);
        digits.insert("7", &sorted_input[1]);
        digits.insert("8", &sorted_input.last().unwrap());

        for num in sorted_input[3..9].into_iter() {
            if num.len() == 5 {
                if common_characters(num, digits.get("1").unwrap()) == 2 {
                    digits.insert("3", num);
                } else if common_characters(num, digits.get("4").unwrap()) == 3 {
                    digits.insert("5", num);
                } else {
                    digits.insert("2", num);
                }
            } else if num.len() == 6 {
                if common_characters(num, digits.get("4").unwrap()) == 4 {
                    digits.insert("9", num);
                } else if common_characters(num, digits.get("1").unwrap()) != 2 {
                    digits.insert("6", num);
                } else {
                    digits.insert("0", num);
                }
            }

        }
        let mut out = String::new();
        for num in output.iter() {
            out.push_str(find_digit_value(&num, &digits).as_str());
        }

        ans += out.parse::<i32>().unwrap();
    }

    ans
}

fn common_characters(a: &str, b: &str) -> i32 {
    let set_a: HashSet<char> = a.chars().collect();
    let set_b: HashSet<char> = b.chars().collect();

    (set_a.intersection(&set_b).collect::<Vec<&char>>()).len() as i32
}

fn find_digit_value(digit: &str, digits: &HashMap<&str, &String>) -> String {
    for key in digits.keys() {
        let val = digits.get(key).unwrap();
        if val.len() != digit.len() {
            continue;
        }

        if val.chars().all(|e| digit.contains(e)) {
            return key.to_string();
        }
    }

    String::new()

}