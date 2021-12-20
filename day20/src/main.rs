fn parse_data() -> (Vec<char>, Vec<Vec<char>>) {
    let (_algo, _img) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let mut algorithm = Vec::new();
    let width = _img.lines().next().unwrap().len();
    let height = _img.lines().count();
    let mut image = init_new_img(width, height, 0);
    _algo.lines().for_each(|l| l.chars().for_each(|ch| algorithm.push(ch)));

    _img.lines().enumerate().for_each(|(i, l)| l.chars().enumerate().for_each(|(j, ch)| image[2 + i][2 + j] = ch));

    (algorithm, image)
}

fn enhance(algo: &Vec<char>, img: &Vec<Vec<char>>, n: i32) -> Vec<Vec<char>> {
    let width = img[0].len();
    let height = img.len();
    let mut new_image = init_new_img(width, height, n);

    let shift = 1;

    for i in shift..height - shift {
        for j in shift..width - shift {
            let idx = usize::from_str_radix(&get_pixel_val(j, i, img), 2).unwrap();
            let new_val = algo[idx];
            new_image[2 + i][2 + j] = new_val;
        }
    }

    new_image

}

fn get_pixel_val(x: usize, y: usize, img: &Vec<Vec<char>>) -> String {
    let offsets: [(i32, i32);9] = [(-1, -1), (0, -1), (1, -1), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let mut val = String::new();
    for (ox, oy) in offsets {
        let new_x = x as i32 + ox;
        let new_y = y as i32 + oy;
        let digit = img[new_y as usize][new_x as usize];
        if digit == '#' {
            val.push('1');
        } else {
            val.push('0');
        }

    }

    val
}

fn init_new_img(width: usize, height: usize, n: i32) -> Vec<Vec<char>> {
    let mut fill = '.';
    if n % 2 != 0 {
        fill = '#';
    }
    vec![vec![fill;width+4];height+4]
}

fn count_lit_pixels(img: &Vec<Vec<char>>) -> usize {
    img.iter().flatten().filter(|&&e| e == '#').count()

}

fn display_img(img: &Vec<Vec<char>>) {
    for line in img.iter() {
        for ch in line.iter() {
            print!("{}", ch);
        }
        println!();
    }
}

fn part1() -> usize {
    let (algo, mut img) = parse_data();
    for i in 0..2 {
        img = enhance(&algo, &img, i + 1);
    }
    count_lit_pixels(&img)
}

fn part2() -> usize {
    let (algo, mut img) = parse_data();
    for i in 0..50 {
        img = enhance(&algo, &img, i + 1);
    }
    count_lit_pixels(&img)
}

fn main() {
    println!("Part1 answer is: {}", part1());
    println!("Part2 answer is: {}", part2());
}

