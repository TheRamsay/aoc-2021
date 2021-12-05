use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

fn main() {
    let (part1_ans, part2_ans) = solve();
    println!("Part1 answer is: {}", part1_ans);
    println!("Part1 answer is: {}", part2_ans);
}

fn solve() -> (i32, i32) {
    let mut map_part1: HashMap<(i32, i32), i32> = HashMap::new();
    let mut map_part2: HashMap<(i32, i32), i32> = HashMap::new();

    let data: Vec<Vec<(i32, i32)>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut parsed_line: Vec<(i32, i32)> = Vec::new();
            for pair in line.split(" -> ") {
                let coords: Vec<i32> = pair.split(",").map(|e| e.parse::<i32>().unwrap()).collect();
                parsed_line.push((coords[0], coords[1]));
            }

            parsed_line
        })
        .collect();

    for line in data {
        let (x1, y1) = line[0];
        let (x2, y2) = line[1];

        // Vertical
        if x1 == x2 {
            for i in min(y1, y2)..(max(y1, y2) + 1) {
                let val1 = map_part1.entry((x1, i)).or_insert(0);
                let val2 = map_part2.entry((x1, i)).or_insert(0);
                *val1 += 1;
                *val2 += 1;
            }
        // Horizontal
        } else if y1 == y2 {
            for i in min(x1, x2)..(max(x1, x2) + 1) {
                let val1 = map_part1.entry((i, y1)).or_insert(0);
                let val2 = map_part2.entry((i, y1)).or_insert(0);
                *val1 += 1;
                *val2 += 1;
            }
        // Diagonal
        } else {
            let steps = i32::abs(x1 - x2) + 1;
            let x_step = if x2 > x1 { 1 } else { -1 };
            let y_step = if y2 > y1 { 1 } else { -1 };

            for i in 0..steps {
                let new_x = x1 + x_step * i;
                let new_y = y1 + y_step * i;

                let val2 = map_part2.entry((new_x, new_y)).or_insert(0);
                *val2 += 1;
            }
        }
    }

    (count_lines(&map_part1), count_lines(&map_part2))
}

fn count_lines(map: &HashMap<(i32, i32), i32>) -> i32 {
    let mut counter = 0;
    for &val in map.values() {
        if val > 1 {
            counter += 1;
        }
    }

    counter
}
