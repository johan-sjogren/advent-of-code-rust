use std::collections::HashSet;

fn main() {
    for line in include_str!("../day3.txt").lines() {
        let split_arr: Vec<_> = line.chars().collect();
        let mid_point: usize = split_arr.len() / 2;
        // println!("{:?}", line);
        // println!("{:?}", split_arr);
        // println!("{:?}", mid_point);
        let first_comp = &split_arr[..mid_point];
        let second_comp = &split_arr[mid_point..];
        println!("{:?}", first_comp);
        println!("{:?}", second_comp);
        let mut set1 = HashSet::new();
        for x in first_comp {
            set1.insert(x);
        }
        let mut set2 = HashSet::new();
        for x in second_comp {
            set2.insert(x);
        }
        let inter = set1.intersection(&set2).collect();
        println!("{:?}", inter);

        let alphabet: Vec<_> = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
            .unwrap()
            .chars()
            .collect();
        println!("{:?}", alphabet);

        let mut index: u32 = alphabet
            .iter()
            .position(|&r| r == 't')
            .unwrap()
            .try_into()
            .expect("Could not convert to int");
        index += 1;

        println!("{}", index);
        break;
    }
}
