use std::fs;

fn main() {
    println!("Part1 answer is: {}", part1());
    println!("Part2 answer is: {}", part2());
}

fn part1() -> u32 {
    let content = fs::read_to_string("input.txt").expect("WRONG!!!");
    let mut prev: Option<i32> = None;
    let mut increasing_count: u32 = 0;

    let lines: Vec<i32> = content
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    for num in lines {
        if let Some(x) = prev {
            if num > x {
                increasing_count += 1;
            }
        }

        prev = Some(num);
    }

    increasing_count
}

fn part2() -> u32 {
    let content = fs::read_to_string("input.txt").expect("WRONG!!!");
    let mut prev_sum: Option<i32> = None;
    let mut increasing_count: u32 = 0;
    let mut buffer: Vec<i32> = vec![];

    let lines: Vec<i32> = content
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    for (i, &num) in lines.iter().enumerate() {
        buffer.push(num);
        if buffer.len() > 3 {
            buffer = buffer[1..].to_vec();
        }

        if i > 2 {
            let buffer_sum: i32 = buffer.iter().sum();
            if let Some(x) = prev_sum {
                if buffer_sum > x {
                    increasing_count += 1;
                }
            }

            prev_sum = Some(buffer_sum);
        }
    }

    increasing_count
}
