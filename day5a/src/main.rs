use std::fs;

fn main() {
    // const INPUT_FILE: &str = "/home/angus/aoc/day5a/data/sample.txt";
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    struct RangeMap {
        dst: i64,
        src: i64,
        range: i64,
    }

    let mut maps = file_contents.split("\n\n");

    let seeds = maps.next().unwrap();
    let mut seed_nums: Vec<_> = seeds.strip_prefix("seeds: ").unwrap().split(" ").map(|s| s.parse::<i64>().unwrap()).collect();

    let mut maps_structured: Vec<Vec<RangeMap>> = vec![];
    for map in maps {
        let ranges = map.split_once(":\n").unwrap().1.lines();
        let ranges_vec: Vec<_> = ranges.map(|l| {
            let nums: Vec<_> = l.split_ascii_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();
            RangeMap {
                dst: nums[0],
                src: nums[1],
                range: nums[2],
            }
        }).collect();
        maps_structured.push(ranges_vec);
    }

    for map in maps_structured {
        seed_nums = seed_nums.iter().map(|n| {
            let mut result = *n;
            for rangemap in &map {
                if *n >= rangemap.src && *n < rangemap.src + rangemap.range {
                    result = *n + (rangemap.dst - rangemap.src);
                }
            }
            result
        }).collect();
    }

    let min = seed_nums.iter().min().unwrap();
    println!("{min}");
}
