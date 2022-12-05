use std::collections::HashSet;

fn main() {
    let alphabet: Vec<_> = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
        .unwrap()
        .chars()
        .map(|c| String::from(c))
        .collect();
    let mut sum = 0;

    for line in include_str!("../day3.txt").lines() {
        let split_arr: Vec<String> = line.chars().map(|t| String::from(t)).collect();
        let mid_point: usize = split_arr.len() / 2;
        let first_comp: Vec<String> = split_arr[..mid_point].to_vec();
        let second_comp: Vec<String> = split_arr[mid_point..].to_vec();
        let set1: HashSet<String> = HashSet::from_iter(first_comp);
        let set2: HashSet<String> = HashSet::from_iter(second_comp);
        println!("{:?}", set1);
        let in_both: Vec<_> = Vec::from_iter(set1.intersection(&set2));

        let mut index: u32 = alphabet
            .iter()
            .position(|r| r == in_both[0])
            .unwrap()
            .try_into()
            .expect("Could not convert to int");
        index += 1;
        sum += index;
    }
    println!("{}", sum);
}
