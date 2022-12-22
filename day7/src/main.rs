use std::fs;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// pub struct Tree {
//     count: usize,
//     root: Option<NodeLink>,
// }


#[derive(Debug)]
struct Directory {
    entries: HashMap<String, Entry>,
    files: Option<File>,
    directories: Option<Weak<RefCell<Directory>>>,
    parent: Option<Weak<RefCell<Directory>>>,
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

fn main() {
    let input_str: String = fs::read_to_string("input.txt")
        .expect("Failed to load file");
    for line in input_str.lines().take(10){
        println!("{}", line);
    }
}
