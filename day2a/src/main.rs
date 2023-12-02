use std::fs;
// use regex::Regex;

fn main() {
    // const INPUT_FILE: &str = "data/sample.txt";
    const INPUT_FILE: &str = "data/input.txt";

    const MAX_RED : usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let mut total = 0;
    'game: for (i, line) in file_contents.lines().enumerate() {
        let game_no = i + 1;

        let (_, attempts) = line.split_once(": ")
            .expect("Line should be prefixed with game");
        
        for attempt in attempts.split(";") {
            for ball in attempt.split(",").map(|s| s.trim()) {
                let (num_str, colour) = ball.split_once(" ").unwrap();
                let num = num_str.parse::<usize>().unwrap();
                if colour.eq("red") && num > MAX_RED {
                    continue 'game;
                }
                if colour.eq("green") && num > MAX_GREEN {
                    continue 'game;
                }
                if colour.eq("blue") && num > MAX_BLUE {
                    continue 'game;
                }
            }
        }

        total += game_no;
    }

    println!("{total}");
}
