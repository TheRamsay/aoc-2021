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
        let (mut x1, mut y1) = line[0];
        let (x2, y2) = line[1];

        let mut x_step = if x2 > x1 { 1 } else { -1 };
        let mut y_step = if y2 > y1 { 1 } else { -1 };

        if x1 == x2 {
            x_step = 0;
        }
        if y1 == y2 {
            y_step = 0;
        }

        while (x1 != (x2 + x_step)) || (y1 != (y2 + y_step)) {
            if (x_step == 0) | (y_step == 0) {
                let val1 = map_part1.entry((x1, y1)).or_insert(0);
                *val1 += 1;
            }
            let val2 = map_part2.entry((x1, y1)).or_insert(0);
            *val2 += 1;

            x1 += x_step;
            y1 += y_step;
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
