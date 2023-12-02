use std::fs;

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let mut total = 0;
    for line in file_contents.lines() {
        let mut first_digit = None;
        let mut last_digit = None;
        for chr in line.chars() {
            if let Some(num) = chr.to_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(num);
                }

                last_digit = Some(num);
            }
        }
        let first_num = first_digit.expect("All lines should contain a digit");
        let last_num = last_digit.expect("All lines should contain a digit");
        total += first_num * 10 + last_num;
    }

    println!("{total}");
}
