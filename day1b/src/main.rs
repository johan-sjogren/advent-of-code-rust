fn main() {
    let mut calories: u32 = 0;
    let mut tracker: Vec<u32> = Vec::new();

    for line in include_str!("../input.txt").lines() {
        let line_num: u32 = match line.parse() {
            Ok(num) => num,
            Err(_) => {
                // New line means a new elf!!
                tracker.push(calories);
                calories = 0;
                continue;
            }
        };
        calories += line_num;
    };
    tracker.sort();
    tracker.reverse();
    let top_3_total: u32 = tracker.iter().take(3).sum();
    println!("{}", top_3_total);
    println!("{:?}", tracker);
}
