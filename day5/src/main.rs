// use itertools::Itertools;
use std::{collections::VecDeque, fs};

#[derive(Debug)]
struct MoveAction {
    num: usize,
    from: usize,
    to: usize,
}

impl MoveAction {
    fn from_string(input: &str) -> MoveAction {
        let instructions = input
            .split_whitespace()
            .filter_map(|c| c.parse().ok())
            .collect::<Vec<usize>>();
        MoveAction {
            num: instructions[0],
            from: instructions[1] - 1,
            to: instructions[2] - 1,
        }
    }
}

#[derive(Debug)]
struct Supplies {
    storage_map: Vec<VecDeque<String>>,
}

impl Supplies {
    fn from_strings(input: &Vec<String>) -> Supplies {
        let input: Vec<Vec<&str>> = input
            .iter()
            .map(|b| {
                b.as_bytes()
                    .chunks(4)
                    .map(std::str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap()
            })
            .collect();

        let mut storage: Vec<VecDeque<String>> = Vec::new();
        for _ in 0..input[0].len() {
            storage.push(VecDeque::new())
        }

        for row in input {
            for (i, item) in row.iter().enumerate() {
                let cleaned: String = item.trim().replace("[", "").replace("]", "");
                let to_push = match cleaned.is_empty() {
                    true => continue,
                    false => cleaned,
                };
                storage[i].push_back(to_push.clone());
            }
        }
        Supplies {
            storage_map: storage,
        }
    }

    fn perform_move(&mut self, move_action: MoveAction) {
        for _ in 0..move_action.num {
            let moved = self.storage_map[move_action.from].pop_front().expect("");
            self.storage_map[move_action.to].push_front(moved);
        }
    }

    fn perform_collective_move(&mut self, move_action: MoveAction) {
        let moved = self.storage_map[move_action.from]
            .drain(..move_action.num)
            .collect::<VecDeque<String>>();
        for item in moved.iter().rev() {
            self.storage_map[move_action.to].push_front(item.to_string());
        }
    }
}

fn split_moves_and_suppplies(input: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut supplies: Vec<String> = Vec::new();
    let mut moves: Vec<String> = Vec::new();
    let mut read_supplies: bool = true;
    for line in input {
        if line.is_empty() {
            read_supplies = false
        } else if read_supplies {
            supplies.push(line.to_string());
        } else {
            moves.push(line.to_string());
        }
    }
    supplies.pop();
    (supplies, moves)
}

fn part1(input_vec: &Vec<String>) {
    let (supplies, moves) = split_moves_and_suppplies(input_vec);

    let mut supplies = Supplies::from_strings(&supplies);
    println!("{:?}", supplies.storage_map);
    for move_action in moves {
        supplies.perform_move(MoveAction::from_string(&move_action));
    }
    println!("{:?}", supplies.storage_map);
    let firsts: String = supplies
        .storage_map
        .iter()
        .map(|vec| String::from(&vec[0]))
        .collect::<String>();
    println!("{}", firsts);
}

fn part2(input_vec: &Vec<String>) {
    let (supplies, moves) = split_moves_and_suppplies(input_vec);

    let mut supplies = Supplies::from_strings(&supplies);
    println!("{:?}", supplies.storage_map);
    for move_action in moves {
        supplies.perform_collective_move(MoveAction::from_string(&move_action));
    }
    println!("{:?}", supplies.storage_map);
    let firsts: String = supplies
        .storage_map
        .iter()
        .map(|vec| String::from(&vec[0]))
        .collect::<String>();
    println!("{}", firsts);
}
fn main() {
    let input_vec: Vec<String> = fs::read_to_string("input.txt")
        .expect("Failed to load file")
        .lines()
        .map(|t| String::from(t))
        .collect();

    part1(&input_vec);
    part2(&input_vec);
}
