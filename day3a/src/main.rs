fn main() {
    for line in include_str!("../day3.txt").lines() {
        let split_arr: Vec<_> = line.chars().collect();
        let mid_point: usize = split_arr.len() / 2;
        // println!("{:?}", line);
        println!("{:?}", split_arr);
        println!("{:?}", mid_point);
        let first_comp = &split_arr[..mid_point];
        let second_comp = &split_arr[mid_point..];
        println!("{:?}", first_comp);
        println!("{:?}", second_comp);
        break;
    }
}
