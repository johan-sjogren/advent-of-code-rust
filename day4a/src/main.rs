use std::fs;

fn main() {
    let input_vec: Vec<String> = fs::read_to_string("day4.txt")
        .expect("failed to load")
        .lines()
        .map(|t| String::from(t))
        .collect();

    for line in input_vec {
        let splits: Vec<&str> = line.split(",").collect();
        println!("{:?}", splits);

        for assg in splits {
            let nums: Vec<u32> = assg
                .split("-")
                .try_into().expect("Error")
                .collect();
        }
        break;
    }
}
