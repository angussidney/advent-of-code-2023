use std::fs;

fn main() {
    // const INPUT_FILE: &str = "/home/angus/aoc/day4b/data/sample.txt";
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let mut matches = vec!();
    for line in file_contents.lines() {
        let (_, nums) = line.split_once(":").unwrap();
        let (winning, ours) = nums.split_once("|").unwrap();
        let win_nums: Vec<_> = winning.split_whitespace().map(|s| s.trim().parse::<i32>()).collect();
        let our_nums: Vec<_> = ours.split_whitespace().map(|s| s.trim().parse::<i32>()).collect();


        let mut found: usize = 0;
        for w in &win_nums {
            for o in &our_nums {
                if o == w {
                    found += 1;
                }
            }
        }
        matches.push(found);
    }

    let mut total_winnings: Vec<usize> = vec![1; matches.len()];
    for (i, count) in matches.iter().enumerate() {
        for j in 1..(*count + 1) {
            total_winnings[i + j] += total_winnings[i];
        }
    }

    // for i in total_winnings.clone() {
    //     println!("{}", i);
    // }

    println!("{}", total_winnings.iter().sum::<usize>());
}
