use std::fs;

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let file = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let nums = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut total = 0;
    for line in file.lines() {
        let mut first_num = None;
        let mut last_num = None;
        for i in 0..line.len() {
            let slice = &line[i..];
            for (i, num) in nums.iter().enumerate() {
                if slice.starts_with(num) {
                    let decimal_digit = i % 10;
                    if first_num.is_none() {
                        first_num = Some(decimal_digit);
                    }
                    last_num = Some(decimal_digit);
                }
            }
        }
        total += first_num.expect("Line should have a digit") * 10 + last_num.expect("Line should have a digit");
    }

    println!("{total}");
}
