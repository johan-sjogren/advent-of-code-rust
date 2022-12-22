use itertools::Itertools;
use std::fs;

fn find_position_of_unique_window(input_vec: &Vec<char>, window_length: usize) -> usize {
    for ( i, window) in input_vec.windows(window_length).enumerate() {
        let max_count: usize = *window
            .iter()
            .counts()
            .values()
            .max()
            .expect("Unexpected value");
        if max_count == 1 {
            return i + window_length;
        }
    }
    0
}
fn main() {
    let input_vec: Vec<char> = fs::read_to_string("input.txt")
        .expect("failed to load")
        .chars()
        .collect();
    println!("{:?}", find_position_of_unique_window(&input_vec, 4));
    println!("{:?}", find_position_of_unique_window(&input_vec, 14));
}
