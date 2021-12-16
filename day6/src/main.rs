fn main() {
    println!("Part1 answer is: {}", part1(80));
    println!("Part2 answer is: {}", part2(256));
}

fn part1(days: i32) -> usize {
    let mut fish_vec: Vec<i32> = include_str!("../test.txt")
        .split(",")
        .map(|e| e.parse().expect(e))
        .collect();

    for _ in 0..days {
        let mut temp: Vec<i32> = Vec::new();
        for fish in fish_vec.iter_mut() {
            *fish -= 1;
            if *fish == 0 {
                temp.push(9);
                *fish = 7;
            }
        }

        for t in temp.iter() {
            fish_vec.push(*t)
        }
    }

    fish_vec.len()
}

fn part2(days: i32) -> i128 {
    let mut fish_table: Vec<i128> = vec![0; 10];

    for fish in include_str!("../input.txt")
        .split(",")
        .map(|e| e.parse::<usize>().unwrap())
    {
        fish_table[fish] += 1;
    }

    for _ in 0..days {
        let mut temp: Vec<i128> = vec![0; 10];
        for (idx, val) in fish_table.iter().enumerate() {
            let new_val;
            if idx == 0 {
                new_val = 6;
                temp[8] += val;
            } else {
                new_val = idx - 1;
            }
            temp[new_val] += val;
        }

        fish_table = temp.to_vec();
    }

    fish_table.iter().sum()
}
