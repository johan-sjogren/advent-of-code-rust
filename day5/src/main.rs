use itertools::Itertools;
use std::fs;

struct Supplies {
    storage_map: Vec<Vec<char>>,
}

struct MoveAction {
    num: u32,
    from: u32,
    to: u32,
}

// impl Supplies {
//     fn from_strngs(input: &Vec<String>)-> Supplies {
//     let test: Vec<Vec<&str>> = input
//          .iter()
//          .map(|b| b.as_bytes()
//          .chunks(4)
//          .map(std::str::from_utf8)
//          .collect::<Result<Vec<&str>, _>>()
//          .unwrap()).collect();
//     }
// }

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

    // let supplies = Supplies::from_strings(supplies);

    let test: Vec<Vec<&str>> = supplies
        .iter()
        .map(|b| {
            b.as_bytes()
                .chunks(4)
                .map(std::str::from_utf8)
                .collect::<Result<Vec<&str>, _>>()
                .unwrap()
        })
        .collect();

    println!("{:?}", test.len());
    println!("{:?}", test[0].len());

    let mut bla: Vec<Vec<String>> = Vec::new();
    for x in test[0].iter() {
        bla.push(Vec::new())
    }

    println!("{:?}", bla);

    for row in test {
        for (i, item) in row.iter().enumerate() {
            let cleaned: String = item.trim().replace("[", "").replace("]", "");
            let to_push = match cleaned.is_empty() {
                true => continue,
                false => cleaned
                // false => println!("Something {} count {}", cleaned, i),
            };
            // println!("{} {}", i, to_push);
            bla[i].push(to_push.clone());
        }
    }
    
    println!("{:?}", bla);
    // for line in &supplies {
    //     let test: Vec<String> = line.chars().map(|t| String::from(t)).chunks(4).collect();
    //     println!("{:?}", test);
    // }
    // println!("{:?}", moves);
}
