use std::cmp::max;
use std::cmp::min;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    println!("{}", part1());
    println!("{}", part2());
    let x = start.elapsed();
    println!("Elapsed time: {:?} seconds", x);
}

fn parse_data() -> Vec<i32> {
    let positions: Vec<i32> = include_str!("../test.txt")
        .split(",")
        .map(|e| e.trim().parse::<i32>().expect(e))
        .collect();
    positions
}

fn part1() -> i32 {
    let mut positions = parse_data();
    positions.sort();
    let mid = positions.len() / 2;
    let best_pos = positions[mid];

    let mut ans = 0;
    for pos in positions.iter() {
        ans += (pos - best_pos).abs();
    }

    ans
}

fn part2() -> i32 {
    let positions = parse_data();
    let mean = positions.iter().sum::<i32>() as f32 / positions.len() as f32;

    let best_pos1 = mean.ceil() as i32;
    let best_pos2 = mean.floor() as i32;

    let mut ans1 = 0;
    let mut ans2 = 0;

    for &pos in positions.iter() {
        ans1 += (0..(pos - best_pos1).abs()+1).sum::<i32>();
        ans2 += (0..(pos - best_pos2).abs()+1).sum::<i32>();
    }

    min(ans1, ans2)
}
