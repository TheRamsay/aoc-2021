use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: usize,
    x: usize,
    y: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| (self.x, self.y).cmp(&(other.x, other.y)))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

fn shortest_path(
    start: (usize, usize),
    end: (usize, usize),
    matrix: &Vec<Vec<usize>>,
) -> Option<usize> {
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; matrix[0].len()]; matrix.len()];

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    let offsets: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    dist[start.1][start.0] = 0;
    heap.push(Node {
        cost: 0,
        x: start.0,
        y: start.1,
    });

    while let Some(Node { cost, x, y }) = heap.pop() {
        if (x == end.0) & (y == end.1) {
            return Some(cost);
        }

        if cost > dist[y][x] {
            continue;
        }

        for (ox, oy) in offsets.iter() {
            let new_x = x as i32 + ox;
            let new_y = y as i32 + oy;
            if (new_x < 0)
                | (new_y < 0)
                | (new_x >= matrix[0].len() as i32)
                | (new_y >= matrix.len() as i32)
            {
                continue;
            }
            let v = Node {
                cost: matrix[new_y as usize][new_x as usize],
                x: new_x as usize,
                y: new_y as usize,
            };

            let next = Node {
                cost: cost + v.cost,
                x: v.x,
                y: v.y,
            };

            if next.cost < dist[next.y][next.x] {
                heap.push(next);
                dist[next.y][next.x] = next.cost;
            }
        }
    }

    None
}

fn get_full_map(matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let expanded = (0..(5 * matrix.len()))
        .map(|x| {
            (0..(5 * matrix[0].len()))
                .map(|y| {
                    let cost = matrix[x % matrix.len()][y % matrix[0].len()]
                        + (x / matrix.len())
                        + (y / matrix[0].len());
                    if cost < 10 {
                        cost
                    } else {
                        cost - 9
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    expanded
}

fn part1() -> usize {
    let data = parse_data();
    let width = data[0].len() - 1;
    let height = data.len() - 1;
    shortest_path((0, 0), (width, height), &data).unwrap()
}

fn part2() -> usize {
    let expanded_data = get_full_map(&parse_data());
    let width = expanded_data[0].len() - 1;
    let height = expanded_data.len() - 1;
    shortest_path((0, 0), (width, height), &expanded_data).unwrap()

}

fn main() {
    println!("Part1 answer is: {}", part1());
    println!("Part2 answer is: {}", part2());
}
