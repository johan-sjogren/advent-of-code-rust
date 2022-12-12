use itertools::Itertools;
use std::fs;

struct Supplies {
    storage_map: Vec<Vec<char>>,
}

impl Supplies {
    fn from_strngs(input: &Vec<String>)-> Supplies {
    let test: Vec<Vec<&str>>= supplies
         .iter()
         .map(|b| b.as_bytes()
         .chunks(4)
         .map(std::str::from_utf8)
         .collect::<Result<Vec<&str>, _>>()
         .unwrap()).collect();
         
    }
}

struct MoveAction {
    num: u32,
    from: u32,
    to: u32,
}

fn main() {
    let input_vec: Vec<String> = fs::read_to_string("input.txt")
        .expect("Failed to load file")
        .lines()
        .map(|t| String::from(t))
        .collect();
    let mut supplies: Vec<String> = Vec::new();
    let mut moves: Vec<String> = Vec::new();
    let mut read_supplies: bool = true;
    for line in input_vec {
        if line.is_empty() {
            read_supplies = false
        } else if read_supplies {
            supplies.push(line);
        } else {
            moves.push(line);
        }
    }
    supplies.pop();

    let supplies = Supplies::from_strings(supplies);
    println!("{:?}", test);
    // for line in &supplies {
    //     let test: Vec<String> = line.chars().map(|t| String::from(t)).chunks(4).collect();
    //     println!("{:?}", test);
    // }
    // println!("{:?}", moves);
}
