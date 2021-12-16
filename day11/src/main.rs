use std::collections::HashSet;

fn parse_data() -> Vec<Vec<usize>> {
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|e| e.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect()
}

fn simulate_flashes(grid: &mut Vec<Vec<usize>>) -> usize {
    let mut flash_count = 0;
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    while find_flashing(grid).len() != 0 {
        let flashing = find_flashing(grid);
        flashed.extend(flashing.clone());
        flash_count += flashing.len();

        for &(x, y) in flashing.iter() {
            update_neighbours(x, y, grid, &flashed);
        }
    }

    flash_count
}

fn update_neighbours(
    x: usize,
    y: usize,
    grid: &mut Vec<Vec<usize>>,
    flashed: &HashSet<(usize, usize)>,
) {
    let offsets: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, -1),
        (-1, -1),
        (-1, 1),
        (1, 1),
    ];
    for &(ox, oy) in offsets.iter() {
        let new_x = x as i32 + ox;
        let new_y = y as i32 + oy;
        if (new_x < 0) | (new_y < 0) | (new_x > 9) | (new_y > 9) {
            continue;
        }

        if flashed.contains(&(new_x as usize, new_y as usize)) {
            continue;
        }

        grid[new_y as usize][new_x as usize] += 1;
    }

    grid[y][x] = 0;
}

fn increase_all(grid: &mut Vec<Vec<usize>>) {
    for i in 0..10 {
        for j in 0..10 {
            grid[i][j] += 1;
        }
    }
}

fn all_flashing(grid: &Vec<Vec<usize>>) -> bool {
    for i in 0..10 {
        for j in 0..10 {
            if grid[i][j] != 0 {
                return false;
            }
        }
    }

    true
}

fn find_flashing(grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut flashing: Vec<(usize, usize)> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            if grid[i][j] > 9 {
                flashing.push((j, i));
            }
        }
    }
    flashing
}

fn part1(steps: u32) -> usize {
    let mut grid = parse_data();
    let mut ans = 0;

    for _ in 0..steps {
        increase_all(&mut grid);
        ans += simulate_flashes(&mut grid);
    }

    ans
}

fn part2() -> usize {
    let mut grid = parse_data();
    let mut step = 0;

    loop {
        increase_all(&mut grid);
        simulate_flashes(&mut grid);
        if all_flashing(&grid) {
            return step + 1;
        }

        step += 1;
    }
}

fn main() {
    println!("Part1 answer is: {}", part1(100));
    println!("Part2 answer is: {}", part2());
}
