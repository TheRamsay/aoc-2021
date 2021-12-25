fn parse_data() -> Vec<Vec<char>> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn move_all(grid: &Vec<Vec<char>>) -> (Vec<Vec<char>>, i32) {
    let width = grid[0].len();
    let height = grid.len();
    let mut new_grid = grid.clone();
    let mut steps = 0;

    let mut moved = Vec::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == '>' {
                if new_grid[i][(j + 1) % width] == '.' {
                    new_grid[i][(j + 1) % width] = '>';
                    moved.push((i, j));
                    steps += 1;
                }
            }
        }
    }

    for &(i, j) in moved.iter() {
        new_grid[i][j] = '.';
    }

    let mut moved = Vec::new();

    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.iter().enumerate() {
            if *ch == 'v' {
                if new_grid[(i + 1) % height][j] == '.' {
                    new_grid[(i + 1) % height][j] = 'v';
                    moved.push((i, j));
                    steps += 1;
                }
            }
        }
    }

    for &(i, j) in moved.iter() {
        new_grid[i][j] = '.';
    }

    (new_grid, steps)
}

fn part1() -> i32 {
    let mut grid = parse_data();
    let mut idx = 0;
    let ans = loop {
        idx += 1;
        let res = move_all(&grid);
        grid = res.0;
        let steps = res.1;

        if steps == 0 {
            break idx;
        }
    };

    ans
}

fn main() {
    println!("Part1 answer is: {}", part1());
}
