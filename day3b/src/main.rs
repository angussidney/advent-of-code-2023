use std::{fs, collections::{HashSet, HashMap}};

const OFFSETS: [(i32, i32); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

fn get_neighbour(grid: &Vec<Vec<char>>, x: i32, y: i32, dx: i32, dy: i32) -> Option<char> {
    let new_x = x + dx;
    let new_y = y + dy;

    if new_x < 0 || new_y < 0 {
        return None;
    }

    let usize_x = new_x as usize;
    let usize_y = new_y as usize;
    
    
    if usize_x >= grid[0].len() || usize_y >= grid.len() {
        return None;
    } else {
        return Some(grid[usize_y][usize_x]);
    }
}

fn main() {
    // const INPUT_FILE: &str = "data/sample.txt";
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");
    let grid: Vec<Vec<char>> = file_contents.lines().map(|l| l.chars().collect()).collect();

    enum NumState {
        Num((String, HashSet<(i32, i32)>)),
        NoNum
    }

    let mut nums: Vec<NumState> = vec!();

    for row in 0..grid.len() {
        let mut num_state = NumState::NoNum;
        for col in 0..grid[0].len() {

            if grid[row][col].is_ascii_digit() {
                let mut gears_nearby = HashSet::new();
                for offset in OFFSETS {
                    if let Some(chr) = get_neighbour(&grid, col as i32, row as i32, offset.0, offset.1) {
                        if chr == '*' {
                            gears_nearby.insert((col as i32 + offset.0, row as i32 + offset.1));
                        }
                    }
                }

                if let NumState::Num((num_so_far, existing_gears)) = num_state {
                    num_state = NumState::Num((format!("{}{}", num_so_far, grid[row][col]), &existing_gears | &gears_nearby));
                } else {
                    num_state = NumState::Num((grid[row][col].to_string(), gears_nearby));
                }
            } else {
                if let NumState::Num(_) = num_state {
                    nums.push(num_state);
                }
                num_state = NumState::NoNum;
            }
        }

        if let NumState::Num(_) = num_state {
            nums.push(num_state);
        }
    }

    let mut map: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    for num in nums {
        if let NumState::Num((number, gears)) = num {
            let number_parsed = number.parse::<i32>().unwrap();
            
            for gear in gears {
                if map.contains_key(&gear) {
                    let num_vec = map.get_mut(&gear).unwrap();
                    num_vec.push(number_parsed);
                } else {
                    map.insert(gear, vec![number_parsed]);
                }
            }
        }
    }

    let mut total = 0;
    for (_, num) in map {
        if num.len() == 2 {
            total += num[0] * num[1];
        }
    }

    println!("{}", total);
}
