use std::cmp::max;
use std::collections::HashMap;

use cached::proc_macro::cached;

#[derive(Debug, Clone)]
struct Player {
    current_space: u128,
    score: u128,
}

impl Player {
    fn new(start: u128) -> Self {
        Player { current_space: start, score: 0 }
    }
    
    fn check_loser<'a>(p1: &'a Self, p2: &'a Self) -> &'a Self {
        if p1.score < p2.score {
            return p1;
        }

        return p2;
    }

    fn move_on_board(&mut self, steps: u128) {
        let mut new_space = (self.current_space + steps) % 10;
        if new_space == 0 {
            new_space = 10;
        }
        self.current_space = new_space;
        self.score += self.current_space;
    }
}

struct Dice {
    value: i32,
    count: u128,
}

impl Dice {
    fn new() -> Self {
        Dice { value: 0 , count: 0 }
    }

    fn roll(&mut self) -> i32 {
        self.value += 1;
        self.count += 1;
        if self.value > 100 {
            self.value = 1;
        }
        self.value
    }
}

#[derive(Debug, Clone)]
struct Universe {
    player1: Player,
    player2: Player,
    goal: u128,
    winner: Option<u128>
}

impl Universe {
    fn new(s1: u128, s2: u128, goal: u128) -> Self {
        let mut player1 = Player::new(s1);
        let mut player2 = Player::new(s2);
        Universe { player1, player2, goal, winner: None }
    }

    fn move_players(&mut self, a: u128, b: u128) -> Option<u128> {
        self.player1.move_on_board(a);
        self.player2.move_on_board(b);

        if self.player1.score >= self.goal {
            self.winner = Some(1)
        } else if self.player2.score >= self.goal {
            self.winner = Some(1);
        }

        return self.winner
    }
}

fn parse_data() -> (u128, u128) {
    let mut lines = include_str!("../input.txt").lines();

    let start_1 = lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap();
    let start_2 = lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap();

    (start_1 as u128, start_2 as u128)


}

fn play1(start_1: u128, start_2: u128) -> u128 {
    let mut dice = Dice::new();
    let mut p1 = Player::new(start_1);
    let mut p2 = Player::new(start_2);
    let goal = 1000;

    let loser = loop {
        let steps = (dice.roll() + dice.roll() + dice.roll()) as u128;
        p1.move_on_board(steps);

        if p1.score >= goal {
            break &p2;
        }

        let steps = (dice.roll() + dice.roll() + dice.roll()) as u128;
        p2.move_on_board(steps);

        if p2.score >= goal {
            break &p1;
        }

    };

    loser.score * dice.count
}

// fn play2(start_1: u128, start_2: u128) -> Vec<Universe> {
//     let mut playing_universes: Vec<Universe> = Vec::new();
//     let mut finished_universes: Vec<Universe> = Vec::new();
//     let goal = 12;
//     let u = Universe::new(start_1, start_2, goal);
//     playing_universes.push(u);

//     loop {

//         if playing_universes.len() == 0 {
//             break;
//         }

//         let mut _playing_universes: Vec<Universe> = Vec::new();
//         for u in playing_universes.iter() {
//             let rolls: &[u128;3] = &[1, 2, 3];

//             for a in rolls{
//                 for b  in rolls {
//                     for c  in rolls {
//                         let mut copy = u.clone();
//                         if let Some(_) = copy.move_players(*a + *b + *c) {
//                             finished_universes.push(copy);
//                         } else {
//                             _playing_universes.push(copy);
//                         }
//                     }
//                 }
//             }

//         }
        

//         playing_universes = _playing_universes;
//     }

//     finished_universes

// }

fn count_winnings(history: &Vec<Universe>) -> (u128, u128) {
    let mut p1 = 0;
    let mut p2 = 0;
    for u in history.iter() {
        if u.winner.unwrap() == 1 {
            p1 += 1;
        } else {
            p2 += 1;
        }
    }

    (p1, p2)
}


fn part1() -> u128 {
    let (s1, s2) = parse_data();
    play1(s1, s2)
}
fn play2(p1: u128, p2: u128, s1: u128, s2: u128, cache: &mut HashMap<(u128, u128, u128, u128), (u128, u128)>) -> (u128, u128){

    if s1 >= 21 {
        return (1, 0)
    }
    if s2 >= 21 {
        return (0, 1)
    }
    if let Some(val) = cache.get(&(p1, p2, s1, s2)) {
        return *val
    }

    let mut ans = (0, 0);

    for a in 1..4 {
        for b in 1..4 {
            for c in 1..4 {
                let new_p1 = (p1 + a + b + c) % 10;
                let mut new_s1 = s1 + new_p1 + 1;

                let (x1, y1) = play2(p2, new_p1, s2, new_s1, cache);
                ans = (ans.0+y1, ans.1+x1);
            }
        }
    }

    cache.insert((p1, p2, s1, s2), ans);
    return ans;
    
}

fn part2() -> u128 {
    let (p1, p2) = parse_data();
    let mut a: HashMap<(u128, u128, u128, u128), (u128, u128)> = HashMap::new();
    let (a, b) = play2(p1-1, p2-1, 0, 0, &mut a);
    max(a, b)
}

fn main() {
    println!("Part1 answer is: {}", part1());
    println!("Part2 answer is: {}", part2());
}
