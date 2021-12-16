fn main() {
    part1();
    part2();
}

fn part1() {
    let (points, folds) = parse_data();
    let first_fold = folds.first().unwrap();
    let x;
    if first_fold.0 == "x" {
        x = fold_by_x(points, first_fold.1);
    } else {
        x = fold_by_y(points, first_fold.1);
    }

    println!("Part1 answer is: {}", count_points(&x));
}

fn part2() {
    let (mut points, folds) = parse_data();
    for (dir, val) in folds.iter() {
        if dir == "x" {
            points = fold_by_x(points, *val);
        } else {
            points = fold_by_y(points, *val);
        }
    }

    println!("Part2 answer is: ");
    display_matrix(&points);
}

fn parse_data() -> (Vec<Vec<i32>>, Vec<(String, i32)>) {
    let mut points: Vec<(i32, i32)> = Vec::new();
    let mut folds: Vec<(String, i32)> = Vec::new();
    let mut x_max = -1;
    let mut y_max = -1;
    for line in include_str!("../input.txt").lines() {
        if line.contains("fold") {
            let data = line.replace("fold along ", "");
            let split: Vec<String> = data.split("=").map(|e| e.to_string()).collect();
            let direction = split.first().unwrap();
            let value = split.last().unwrap().parse::<i32>().unwrap();
            folds.push((direction.to_string(), value));
            continue;
        }
        if line == "" {
            continue;
        }

        let split: Vec<i32> = line.split(",").map(|e| e.parse::<i32>().unwrap()).collect();
        let x = split.first().unwrap();
        if *x > x_max {
            x_max = *x;
        }
        let y = split.last().unwrap();
        if *y > y_max {
            y_max = *y;
        }
        points.push((*x, *y))
    }

    let mut coords = vec![vec![0; (x_max + 1) as usize]; (y_max + 1) as usize];
    for &(x, y) in points.iter() {
        coords[y as usize][x as usize] = 1;
    }

    (coords, folds)
}

fn fold_by_y(points: Vec<Vec<i32>>, val: i32) -> Vec<Vec<i32>> {
    let row_len = points[0].len();
    let mut new_arr = vec![vec![0; row_len]; val as usize];
    for i in 0..val {
        for j in 0..row_len {
            new_arr[i as usize][j as usize] = points[i as usize][j as usize];
        }
    }
    for i in (val + 1)..(points.len() as i32) {
        for j in 0..row_len {
            let diff = i - val;
            let new_row = val - diff;
            if new_row < 0 {
                continue;
            }

            if new_arr[new_row as usize][j as usize] == 1 {
                continue;
            }

            new_arr[new_row as usize][j as usize] = points[i as usize][j as usize];
        }
    }

    new_arr
}

fn fold_by_x(points: Vec<Vec<i32>>, val: i32) -> Vec<Vec<i32>> {
    let matrix_len = points.len();
    let mut new_arr = vec![vec![0; val as usize]; matrix_len as usize];
    for i in 0..matrix_len {
        for j in 0..val {
            new_arr[i as usize][j as usize] = points[i as usize][j as usize];
        }
    }
    for i in 0..matrix_len {
        for j in (val + 1)..(points[0].len() as i32) {
            let diff = j - val;
            let new_col = val - diff;
            if new_col < 0 {
                continue;
            }

            if new_arr[i as usize][new_col as usize] == 1 {
                continue;
            }

            new_arr[i as usize][new_col as usize] = points[i as usize][j as usize];
        }
    }


    new_arr
}
fn display_matrix(matrix: &Vec<Vec<i32>>) {
    for line in matrix.iter() {
        for &num in line.iter() {
            if num == 1 {
                print!("█");
            } else {
                print!(" ")
            }
        }
        println!();
    }
    println!();
}

fn count_points(matrix: &Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    for line in matrix.iter() {
        for d in line.iter() {
            ans += d;
        }
    }

    ans
}
