use itertools::Itertools;
// use std::collections::HashSet;
use std::fs;

fn main() {

    let alphabet: Vec<_> = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect())
        .unwrap()
        .chars()
        .map(|c| String::from(c))
        .collect();

    let input_vec: Vec<String> = fs::read_to_string("day3.txt")
        .expect("failed to load")
        .lines()
        .map(|t| String::from(t))
        .collect();

    let mut sum = 0;

    for line in input_vec.chunks(3) {
        // let set1: HashSet<char> = HashSet::from_iter(line[0].chars());
        // let set2: HashSet<char> = HashSet::from_iter(line[1].chars());
        // let overlap: String = set1.intersection(&set2).collect();

        let overlap = line[0]
            .chars()
            .into_iter()
            .filter(|k| line[1].chars().contains(k))
            .filter(|k| line[2].chars().contains(k))
            .take(1)
            .collect::<String>();

        let index: u32 = match alphabet.iter().position(|r| r == &overlap) {
            Some(p) => p + 1,
            None => 0,
        }
        .try_into()
        .expect("Could not convert to int");

        sum += index;
    }
    println!("{}", sum);
}
