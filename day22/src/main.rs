use std::collections::HashMap;

fn main() {
    let a = parse_input();
    run(&a);
}


#[derive(Debug)]
struct Line {
    command: String,
    ranges: Vec<(i32, i32)>,
}


fn parse_input() -> Vec<Line> {
    let mut data = Vec::new();
    for line in include_str!("../input.txt").lines() {
        let (command, _coords) = line.split_once(" ").unwrap();
        let ranges = _coords.split(",").map(|e| {
            let p = e[2..].split_once("..").unwrap();
            (p.0.parse().unwrap(), p.1.parse().unwrap())
            }).collect::<Vec<(i32, i32)>>();
        let line = Line { command: String::from(command), ranges };
        data.push(line);
    }

    data
}

fn run(steps: &Vec<Line>) {
    let mut cubes: HashMap<(i32, i32, i32), i32> = HashMap::new();
    for step in steps {

        for x in step.ranges[0].0..step.ranges[0].1 + 1 {
            for y in step.ranges[1].0..step.ranges[1].1 + 1 {
                for z in step.ranges[2].0..step.ranges[2].1 + 1 {
                    if x < -50 || y < -50 || z < -50 || x > 50 || y > 50 || z > 50 {
                        continue;
                    }
                    let mut val = 0;
                    if step.command == "on"{
                        val = 1;
                    }
                    cubes.insert((x, y, z), val);

                }
            }
        }
    }
    let ans: i32 = cubes.values().sum();
    println!("{}", ans);

}