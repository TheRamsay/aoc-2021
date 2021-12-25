use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;

fn parse_data() -> ((i128, i128), (i128, i128)) {
    let s = &include_str!("../input.txt")[13..];
    let (x_data, y_data) = s.split_once(", ").unwrap();
    let x_range = x_data[2..]
        .split_once("..")
        .map(|x| (x.0.parse::<i128>().unwrap(), x.1.parse::<i128>().unwrap()))
        .unwrap();
    let y_range = y_data[2..]
        .split_once("..")
        .map(|x| (x.0.parse::<i128>().unwrap(), x.1.parse::<i128>().unwrap()))
        .unwrap();
    (x_range, y_range)
}

fn check_all(x_range: (i128, i128), y_range: (i128, i128)) -> (i128, usize) {
    let mut peeks: HashSet<i128> = HashSet::new();
    let mut vectors = HashSet::new();
    for i in 0..x_range.1 {
        for j in -250..250 {
            if let Some((height, vector)) = (simulate_trajectory(i, j, x_range, y_range)) {
                peeks.insert(height);
                vectors.insert(vector);
            }
        }
    }

    (*peeks.iter().max().unwrap(), vectors.len())
}

fn simulate_trajectory(
    mut vx: i128,
    mut vy: i128,
    x_range: (i128, i128),
    y_range: (i128, i128),
) -> Option<(i128, (i128, i128))> {
    let start = (vx, vy);
    let mut x = 0;
    let mut y = 0;
    let mut highest = y;
    loop {
        if point_in_rect((x, y), x_range, y_range) {
            return Some((highest, start));
        }

        if (vy < 0) & (y < min(y_range.0, y_range.1)) {
            return None;
        }

        x += vx;
        y += vy;

        if y > highest {
            highest = y;
        }

        if vx > 0 {
            vx -= 1;
        } else if vx < 0 {
            vx += 1;
        }
        vy -= 1;
    }
}

fn point_in_rect(point: (i128, i128), x_range: (i128, i128), y_range: (i128, i128)) -> bool {
    let x1 = min(x_range.0, x_range.1);
    let x2 = max(x_range.0, x_range.1);
    let y1 = min(y_range.0, y_range.1);
    let y2 = max(y_range.0, y_range.1);
    ((point.0 >= x1) & (point.0 <= x2)) & ((point.1 >= y1) & (point.1 <= y2))
}

fn main() {
    let (x_range, y_range) = parse_data();
    let (part1, part2) = check_all(x_range, y_range);
    println!("Part1 answer is: {:?}", part1);
    println!("Part1 answer is: {:?}", part2);
}
