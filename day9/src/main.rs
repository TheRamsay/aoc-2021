use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;

fn get_all_lowest_points(data: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut lowest_points = Vec::new();
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if is_lowest(j as i32, i as i32, &data) {
                lowest_points.push((j, i));
            }
        }
    }

    lowest_points
}

fn parse_data() -> Vec<Vec<i32>> {
    let mut data: Vec<Vec<i32>> = Vec::new();
    for line in include_str!("../input.txt").lines() {
        let mut tmp: Vec<i32> = Vec::new();
        for char in line.chars() {
            tmp.push(char.to_digit(10).unwrap() as i32);
        }
        data.push(tmp);
    }

    data
}

fn is_lowest(x: i32, y: i32, data: &Vec<Vec<i32>>) -> bool {
    let val = data[y as usize][x as usize];

    let neighbours = get_neighbours(x, y, data);
    for (n_x, n_y) in neighbours.iter() {
        if data[*n_y as usize][*n_x as usize] <= val {
            return false;
        }
    }

    true
}

fn get_neighbours(x: i32, y: i32, data: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let offsets: [[i32; 2]; 4] = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    let height = data.len() as i32;
    let width = data.first().unwrap().len() as i32;
    let mut neighbours: Vec<(usize, usize)> = Vec::new();

    for [_x, _y] in offsets {
        let new_x = x + _x;
        let new_y = y + _y;
        if (new_x >= width) | (new_y >= height) | (new_x < 0) | (new_y < 0) {
            continue;
        }
        neighbours.push((new_x as usize, new_y as usize));
    }

    neighbours
}

fn part1() -> i32 {
    let data = parse_data();
    let lowest_points = get_all_lowest_points(&data);
    let mut ans = 0;
    for (x, y) in lowest_points {
        ans += data[y][x] + 1;
    }

    ans
}

fn part2() -> usize {
    let data = parse_data();
    let lowest_points = get_all_lowest_points(&data);
    let mut basins: Vec<HashSet<(usize, usize)>> = Vec::new();

    for (x, y) in lowest_points.iter() {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut basin: HashSet<(usize, usize)> = HashSet::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        queue.push_back((*x, *y));

        while !queue.is_empty() {
            let (point_x, point_y) = queue.pop_front().unwrap();
            visited.insert((point_x, point_y));

            let val = data[point_y][point_x];

            if val == 9 {
                continue;
            }

            basin.insert((point_x, point_y));

            let neighbours = get_neighbours(point_x as i32, point_y as i32, &data);
            for n in neighbours {
                if visited.contains(&n) {
                    continue;
                }
                queue.push_back(n);
            }
        }

        basins.push(basin);
    }

    let mut sorted_basins = basins.clone();
    sorted_basins.sort_by(|a, b| {
        if a.len() > b.len() {
            Ordering::Less
        } else if a.len() == b.len() {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    sorted_basins[0].len() * sorted_basins[1].len() * sorted_basins[2].len()
}

fn main() {
    println!("{}", part1());
    println!("{}", part2());
}
