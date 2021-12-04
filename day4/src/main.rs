use std::collections::HashSet;

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());

    // let a: &mut i32;
    // let mut b = 8;
    // a = &mut b;
    // b = 154;
    // println!("{}", a);

}

fn part1() -> i32 {
    let (numbers, mut boards) = parse_data();

    for num in numbers.iter() {
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for (cell, is_marked) in row.iter_mut() {
                    if cell == num {
                        *is_marked = true;
                    }
                }
            }
        }
        for board in boards.iter() {
            let is_completed = check_rows_and_columns(&board);
            if is_completed {
                println!("{:?}", board);
                println!("{:?}", num);
                let sum = sum_unmarked_cells(&board);
                return sum * num;
            }
        }
    }

    0
}

fn part2() -> i32 {
    let (numbers, mut boards) = parse_data();
    let mut winners: HashSet<i32> = HashSet::new();

    for num in numbers.iter() {
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for (value, is_marked) in row.iter_mut() {
                    if value == num {
                        *is_marked = true;
                    }
                }
            }
        }
        for (idx, board) in boards.iter().enumerate() {
            let is_completed = check_rows_and_columns(&board);
            if is_completed {
                if !winners.contains(&(idx as i32)) {
                    winners.insert(idx as i32);
                    if winners.len() == boards.len() {
                        println!("{:?}", board);
                        println!("{:?}", num);
                        println!("{:?}", idx);
                        let sum = sum_unmarked_cells(&board);
                        return sum * num;
                    }
                }
            }
        }
    }

    0
}

fn parse_data() -> (Vec<i32>, Vec<Vec<Vec<(i32, bool)>>>) {
    let mut data: Vec<&str> = include_str!("../input.txt")
        .lines()
        .filter(|&line| line.len() > 1)
        .collect();
    let numbers: Vec<i32> = data[0]
        .split(",")
        .map(|el| el.parse::<i32>().unwrap())
        .collect();

    data.remove(0);

    let mut boards: Vec<Vec<Vec<(i32, bool)>>> = Vec::new();
    let matrix_size = data[1].len();

    let chunks = data.chunks(5);
    for chunk in chunks {
        let mut board: Vec<Vec<(i32, bool)>> = Vec::new();
        for raw_line in chunk {
            let line: Vec<(i32, bool)> = raw_line
                .split_whitespace()
                .map(|el| (el.parse::<i32>().unwrap(), false))
                .collect();
            board.push(line);
        }

        boards.push(board);
    }

    (numbers, boards)
}

fn get_number(data: &Vec<i32>, value: i32) -> &i32 {
    for num in data.iter() {
        if *num == value {
            return &num;
        }
    }

    &-1
}

fn check_rows_and_columns(board: &Vec<Vec<(i32, bool)>>) -> bool {
    for row in board.iter() {
        if row.iter().all(|x| x.1) {
            return true;
        }
    }

    for i in 0..board.len() {
        let mut is_completed = true;
        for line in board {
            if !line[i].1 {
                is_completed = false;
            }
        }
        if is_completed {
            return true;
        }
    }

    false
}

fn sum_unmarked_cells(board: &Vec<Vec<(i32, bool)>>) -> i32 {
    let mut sum = 0;
    for line in board.iter() {
        for cell in line.iter() {
            if !cell.1 {
                sum += cell.0;
            }
        }
    }

    sum
}
