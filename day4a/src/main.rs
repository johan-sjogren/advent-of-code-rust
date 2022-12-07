use std::fs;

fn main() {
    let input_vec: Vec<String> = fs::read_to_string("day4.txt")
        .expect("failed to load")
        .lines()
        .map(|t| String::from(t))
        .collect();

    // for line in input_vec {
    //     let splits: Vec<&str> = line.split(",").collect();
    //     for assg in splits {
    //         let nums: Vec<u32> = assg
    //             .split("-")
    //             .map(|i| i.parse().unwrap())
    //             .collect();
    //             println!("{:?}", nums)
    //     }
    //     break;
    //     let splits: Vec<&str> = line.split(",")
    //             .split("-")
    //             .map(|i| i.parse().unwrap())
    //             .collect();
    //             println!("{:?}", nums)
    //     break;
    // }

    for line in input_vec {
        let (assg1, assg2) = line.split_at(line.find(',').unwrap());
        println!("{} {}", assg1, assg2);
        break;
    }
}
