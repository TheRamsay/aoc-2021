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

fn check_all(x_range: (i128, i128), y_range: (i128, i128)) -> i128 {
    let mut peeks: HashSet<i128> = HashSet::new();
    for i in -1000..1000 {
        for j in -1000..1000 {
            peeks.insert(simulate_trajectory(i, j, x_range, y_range));
        }
    }

    *peeks.iter().max().unwrap()
}

fn simulate_trajectory(
    mut vx: i128,
    mut vy: i128,
    x_range: (i128, i128),
    y_range: (i128, i128),
) -> i128 {
    let mut x = 0;
    let mut y = 0;
    let mut highest = y;
    loop {
        if point_in_rect((x, y), x_range, y_range) {
            return highest;
        }

        if (vy < 0) & (y < min(y_range.0, y_range.1)) {
            return i128::MIN;
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

fn part1() -> i128 {
    let (x_range, y_range) = parse_data();
    check_all(x_range, y_range)
}

fn main() {
    println!("Part1 answer is: {}", part1());
}

fn point_in_rect(
    point: (i128, i128),
    mut x_range: (i128, i128),
    mut y_range: (i128, i128),
) -> bool {
    if y_range.0 > y_range.1 {
        x_range = (x_range.1, x_range.0);
    }

    if y_range.0 > y_range.1 {
        y_range = (y_range.1, y_range.0);
    }

    ((point.0 >= x_range.0) & (point.0 <= x_range.1))
        & ((point.0 >= y_range.0) & (point.1 <= y_range.1))
}
