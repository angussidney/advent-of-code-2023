use std::fs;

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

    let mut total = 0;

    enum NumState {
        Num((String, bool)),
        NoNum
    }

    for row in 0..grid.len() {
        let mut num_state = NumState::NoNum;
        for col in 0..grid[0].len() {

            if grid[row][col].is_ascii_digit() {
                let punctuation_nearby = 'x: {
                    for offset in OFFSETS {
                        if let Some(chr) = get_neighbour(&grid, col as i32, row as i32, offset.0, offset.1) {
                            if chr != '.' && chr.is_ascii_punctuation() {
                                break 'x true;
                            }
                        }
                    }
                    break 'x false;
                };

                if let NumState::Num((num_so_far, punctuation)) = num_state {
                    num_state = NumState::Num((format!("{}{}", num_so_far, grid[row][col]), punctuation || punctuation_nearby));
                } else {
                    num_state = NumState::Num((grid[row][col].to_string(), punctuation_nearby));
                }
            } else {
                if let NumState::Num((ref num_so_far, true)) = num_state {
                    total += num_so_far.parse::<usize>().unwrap();
                    // println!("{}", num_so_far);
                }
                num_state = NumState::NoNum;
            }
        }

        if let NumState::Num((ref num_so_far, true)) = num_state {
            total += num_so_far.parse::<usize>().unwrap();
            // println!("{}", num_so_far);
        }
    }

    println!("{}", total);
}