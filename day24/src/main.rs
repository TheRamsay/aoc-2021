use std::cell::Cell;
use std::fs::read_to_string;

fn main() {
    let mut alu = ALU::new(vec![1, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 3]);
    let mut a = &alu.pos;
    *a = 4;
    println!("{}", alu.pos);
    // 0


    // alu.run("./test.txt");
}

struct ALU {
    w: Cell<i32>,
    x: Cell<i32>,
    y: Cell<i32>,
    z: Cell<i32>,
    pos: usize,
    input: Vec<i32>,
}

impl ALU {
    fn new(input: Vec<i32>) -> Self {
        ALU {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            pos: 0,
            input,
        }
    }

    fn run(&mut self, file_name: &str) {
        for line in read_to_string(file_name).unwrap().lines() {
            let split = line.split(" ").collect::<Vec<_>>();
            let res: Option<(&str, i32, i32)>;
            if split.len() == 2 {
                match split[1] {
                    "x" => self.x = self.input[self.pos],
                    "y" => self.y = self.input[self.pos],
                    "z" => self.z = self.input[self.pos],
                    "w" => self.w = self.input[self.pos],
                    _ => unreachable!(),
                };
            } else {
                let instruction = split[0];
                let mut a = match split[1] {
                    "x" => &self.x,
                    "y" => &self.y,
                    "z" => &self.z,
                    _ => unreachable!(),
                };
                let b;
                if let Ok(x) = split[2].parse::<i32>() {
                    b = x;
                } else {
                    b = match split[2] {
                        "x" => self.x,
                        "y" => self.y,
                        "z" => self.z,
                        _ => unreachable!(),
                    };
                }

                match instruction {
                    "add" => a = &(*a + b),
                    "mul" => a = &(*a * b),
                    "div" => a = &(*a / b),
                    "mod" => a = &(*a % b),
                    "eql" => a = &(if *a == b { 1 } else { 0 }),
                    _ => unimplemented!(),
                }
            }
        }
        println!(
            "{:?} {} {} {} {} {}",
            self.input, self.w, self.x, self.y, self.z, self.pos
        );
    }
}
