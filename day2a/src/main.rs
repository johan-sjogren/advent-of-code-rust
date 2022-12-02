fn shape_score(shape: &str) -> u32 {
    return match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };
}

// A for Rock, B for Paper, and C for Scissors.
// X for Rock, Y for Paper, and Z for Scissors
fn result_score(opponent: &str, player: &str) -> u32 {
    if opponent == "A" {
        return match player {
            "X" => 3, // Draw
            "Y" => 6, // Win
            "Z" => 0, // Lose
            _ => 0,
        }
    } else if opponent == "B" {
        return match player {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        }
    } else if opponent == "C" {
        return match player {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => 0,
        }
    }
    return 0;
}

fn main() {
    let mut sum: u32 = 0;
    for line in include_str!("../day2.txt").lines() {
        // println!("{}", line);
        let split_arr: Vec<_> = line.split_whitespace().collect();
        sum += shape_score(&split_arr[1]);
        sum += result_score(&split_arr[0], &split_arr[1]);
    }
    println!("Final score:  {}", sum);
}
