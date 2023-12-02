use std::fs;

fn main() {
    // const INPUT_FILE: &str = "data/sample.txt";
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let mut total = 0;
    for line in file_contents.lines() {

        let (_, attempts) = line.split_once(": ")
            .expect("Line should be prefixed with game");

        let mut num_red = 0;
        let mut num_green = 0;
        let mut num_blue = 0;
        
        for attempt in attempts.split(";") {
            for ball in attempt.split(",").map(|s| s.trim()) {
                let (num_str, colour) = ball.split_once(" ").unwrap();
                let num = num_str.parse::<usize>().unwrap();
                if colour.eq("red") {
                    num_red = usize::max(num, num_red);
                } else if colour.eq("green") {
                    num_green = usize::max(num, num_green);
                } else if colour.eq("blue") {
                    num_blue = usize::max(num, num_blue);
                }
            }
        }

        total += num_red * num_green * num_blue;
    }

    println!("{total}");
}
