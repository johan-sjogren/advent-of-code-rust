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

fn get_shape(opponent: &str, strat: &str) -> String {
    // A for Rock, B for Paper, and C for Scissors.
    // Output: X for Rock, Y for Paper, and Z for Scissors
    // Input: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win
    if opponent == "A" {
        return String::from( match strat {
            "X" => "Z",
            "Y" => "X",
            "Z" => "Y",
            _ => "",
        })
    } else if opponent == "B" {
        return String::from( match strat {
            "X" => "X",
            "Y" => "Y",
            "Z" => "Z",
            _ => "",
        })
    } else if opponent == "C" {
        return String::from( match strat {
            "X" => "Y",
            "Y" => "Z",
            "Z" => "X",
            _ => "",
        })
    } else {
        return String::from("")
    }
}

fn main() {
    let mut sum: u32 = 0;
    for line in include_str!("../day2.txt").lines() {
        let split_arr: Vec<_> = line.split_whitespace().collect();
        let chosen: String = get_shape(&split_arr[0], &split_arr[1]);
        sum += shape_score(&chosen);
        sum += result_score(&split_arr[0], &chosen);
    }
    println!("Final score:  {}", sum);
}
