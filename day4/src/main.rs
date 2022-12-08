use std::fs;

struct AssignmentRange {
    start: u32,
    end: u32
}

impl AssignmentRange {
    fn contains(&self, other: &Self) -> bool {
        if self.start > other.start{
            return false
        } else if self.end < other.end {
            return false
        }
        else {
            return true
        }
    }
}

impl AssignmentRange {
    fn overlaps(&self, other: &Self) -> bool {
        if self.start <= other.start && self.end >= other.start {
            return true
        }
        else if self.start <= other.end && self.end >= other.end {
            return true
        }
        else {
            return false
        }
    }
}

fn get_int(s: &str) -> u32 {
    let int: u32 = s.replace("-", "").replace(",", "").parse().expect("Bla");
    int
}

fn get_range(s: &str) -> AssignmentRange {
    let (start, end) = s.split_at(s.find("-").unwrap());
    AssignmentRange {start: get_int(start), end: get_int(end)}
}


fn find_contained(input: &Vec<String>) -> u32 {
    let mut count: u32 = 0;
    for line in input {
        let (assg1, assg2) = line.split_at(line.find(',').unwrap());
        let range1= get_range(assg1);
        let range2= get_range(assg2);
        if range1.contains(&range2) || range2.contains(&range1) {
            count += 1;
        }
    }
    count
}

fn find_overlapping(input: &Vec<String>) -> u32 {
    let mut count: u32 = 0;
    for line in input {
        let (assg1, assg2) = line.split_at(line.find(',').unwrap());
        let range1= get_range(assg1);
        let range2= get_range(assg2);
        if range1.overlaps(&range2) || range2.overlaps(&range1) {
            count += 1;
        }
    }
    count
}

fn main() {
    let input_vec: Vec<String> = fs::read_to_string("day4.txt")
        .expect("failed to load")
        .lines()
        .map(|t| String::from(t))
        .collect();
    let n_contained = find_contained(&input_vec);
    println!("{}", n_contained);
    let n_overlapping = find_overlapping(&input_vec);
    println!("{}", n_overlapping);
}
