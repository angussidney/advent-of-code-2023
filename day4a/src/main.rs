use std::fs;

fn main() {
    // const INPUT_FILE: &str = "data/sample.txt";
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let mut total = 0;
    for line in file_contents.lines() {
        let (_, nums) = line.split_once(":").unwrap();
        let (winning, ours) = nums.split_once("|").unwrap();
        let win_nums: Vec<_> = winning.split_whitespace().map(|s| s.trim().parse::<usize>()).collect();
        let our_nums: Vec<_> = ours.split_whitespace().map(|s| s.trim().parse::<usize>()).collect();


        let mut found = 0;
        for w in &win_nums {
            for o in &our_nums {
                if o == w {
                    found += 1;
                }
            }
        }
        // println!("{}", found);

        if found > 0 {
            total += 2_usize.pow(found - 1)
        }
    }

    println!("{}", total);
}
