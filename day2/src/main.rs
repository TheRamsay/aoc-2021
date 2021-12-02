use std::fs;

fn main() {
    let (part1_ans, part2_ans) = solve();
    println!("Part1 answer is: {}", part1_ans); 
    println!("Part2 answer is: {}", part2_ans);
}

struct Position {
    x: i32,
    y: i32,
}

fn solve() -> (i32, i32) {
    let mut part1_pos = Position { x: 0, y: 0 };
    let mut part2_pos = Position { x: 0, y: 0 };
    let mut aim: i32 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let dir = parts[0];
        let val = parts[1].parse::<i32>().unwrap();

        match dir.as_ref() {
            "forward" => {
                part2_pos.y += val * aim;
                part1_pos.x += val;
                part2_pos.x += val;
            }
            "up" => {
                part1_pos.y -= val;
                aim -= val;
            }
            "down" => {
                part1_pos.y += val;
                aim += val;
            },
            _ => {
                panic!()
            }
        }
    }

    (part1_pos.x * part1_pos.y, part2_pos.x * part2_pos.y)
}
