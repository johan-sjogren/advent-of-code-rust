fn main() {
    let mut elf: u32 = 0;
    let mut calories: u32 = 0;
    let mut max_cal = 0;
    let mut max_elf: u32 = 0;

    for line in include_str!("../input.txt").lines() {
        let line_num: u32 = match line.parse() {
            Ok(num) => num,
            Err(_) => {
                // New line means a new elf!!
                if calories > max_cal {
                    max_cal = calories;
                    max_elf = elf;
                };
                elf += 1;
                calories = 0;
                continue;
            }
        };
        calories += line_num;
    }
    println!("{}, {}", max_cal, max_elf)
}
