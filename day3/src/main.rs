use std::cmp;
use std::collections::HashMap;

fn main() {
    // println!("Part1 answer is: {}", part1());
    println!("Part2 answer is: {}", part2());
}

struct Counter {
    zeroes: i64,
    ones: i64,
}

fn part1() -> i64 {
    // let mut bit_count: HashMap<&str, i64> = HashMap::new();
    let mut counter = Counter { ones: 0, zeroes: 0 };
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    let lines = include_str!("../input.txt")
        .lines()
        .map(|e| String::from(e))
        .collect::<Vec<String>>();
    let row_len = lines[0].len();

    for i in 0..row_len {
        lines.iter().for_each(|el| {
            let val = el.chars().nth(i).unwrap();
            match val {
                '0' => counter.zeroes += 1,
                '1' => counter.ones += 1,
                _ => panic!(),
            }
        });

        if counter.zeroes > counter.ones {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }

        counter.ones = 0;
        counter.zeroes = 0;
    }

    i64::from_str_radix(gamma_rate.as_str(), 2).unwrap()
        * i64::from_str_radix(epsilon_rate.as_str(), 2).unwrap()
}

fn part2() -> i64 {
    let lines: Vec<String> = include_str!("../input.txt")
        .lines()
        .map(|e| e.to_string())
        .collect();
    let mut oxygen_pattern = String::from("");
    let mut co2_pattern = String::from("");
    let mut oxygen_data: Vec<&String> = lines.iter().collect();
    let mut co2_data: Vec<&String> = lines.iter().collect();
    let mut idx = 0;
    let line_len = lines[0].len();

    while (oxygen_data.len() != 1) | (idx < line_len) {
        let mut counter = Counter { zeroes: 0, ones: 0 };

        for oxygen_line in oxygen_data.iter() {
            println!("{}, {}", oxygen_line, idx);
            if oxygen_line.chars().nth(idx).unwrap() == '0' {
                counter.zeroes += 1;
            } else {
                counter.ones += 1;
            }
        }

        if counter.ones >= counter.zeroes {
            oxygen_pattern.push('1');
        } else {
            oxygen_pattern.push('0');
        }

        idx += 1;
        oxygen_data = lines
            .iter()
            .filter(|e| e.starts_with(&oxygen_pattern))
            .collect();

        println!("{:?}", oxygen_data);
        println!("{:?}", oxygen_pattern);
    }

    idx = 0;
    while (co2_data.len() != 1) | (idx < line_len) {
        let mut counter = Counter { zeroes: 0, ones: 0 };
        
        println!("{:?}", co2_data);
        println!("{:?}", co2_pattern);
        for co2_line in co2_data.iter() {
            if co2_line.chars().nth(idx).unwrap() == '0' {
                counter.zeroes += 1;
            } else {
                counter.ones += 1;
            }
        }

        if counter.zeroes <= counter.ones {
            co2_pattern.push('0');
        } else {
            co2_pattern.push('1');
        }

        idx += 1;
        co2_data = lines
            .iter()
            .filter(|e| e.starts_with(&co2_pattern))
            .collect();

    }

    // println!("Oxygen: {:?}", oxygen_data);
    println!("CO2: {:?}", co2_data);

    3
}
