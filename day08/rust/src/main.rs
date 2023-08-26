use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    // env::set_var("RUST_BACKTRACE", "full");
    if args.len() > 1 {
        let input = parse_input(&args[1]).unwrap();
        let nrows = input.len();
        let ncols = input[0].len();
        let mut sum1 = 0;
        let mut best = 0;
        let mut best_c = 0;
        let mut best_r = 0;
        for r in 0..nrows {
            for c in 0..ncols {
                if visible(&input, r, c) {
                    sum1 += 1;
                }
                let curr = view_score(&input, r, c);
                // check if better than current best so far
                if curr > best {
                    best = curr;
                    best_r = r;
                    best_c = c;
                }
            }
        }
        println!("Number of trees visible from outside of the grid {}", sum1);
        println!(
            "Tree at index {},{} has most number of trees visible: {}",
            best_r, best_c, best
        );
    }
}

fn parse_input(path: &String) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut res: Vec<Vec<u32>> = Vec::new();
    let mut tmp: Vec<u32>;
    for line in reader.lines() {
        tmp = line?.chars().map(|x| x.to_digit(10).unwrap()).collect();
        if tmp.len() > 0 {
            res.push(tmp.clone());
            tmp.clear();
        }
    }
    return Ok(res);
}

fn visible(input: &Vec<Vec<u32>>, r: usize, c: usize) -> bool {
    let nrows = input.len();
    let ncols = input[0].len();
    if r <= 0 || c <= 0 || nrows <= r + 1 || ncols <= c + 1 {
        return true;
    } else {
        // left
        for j in (0..c).rev() {
            if input[r][j] >= input[r][c] {
                break;
            } else if j == 0 {
                return true;
            }
        }
        // right
        for j in c + 1..ncols {
            if input[r][j] >= input[r][c] {
                break;
            } else if j == ncols - 1 {
                return true;
            }
        }
        // up
        for i in (0..r).rev() {
            if input[i][c] >= input[r][c] {
                break;
            } else if i == 0 {
                return true;
            }
        }
        // down
        for i in r + 1..nrows {
            if input[i][c] >= input[r][c] {
                break;
            } else if i == nrows - 1 {
                return true;
            }
        }
    }
    return false;
}

fn view_score(input: &Vec<Vec<u32>>, r: usize, c: usize) -> u32 {
    let nrows = input.len();
    let ncols = input[0].len();
    if r <= 0 || c <= 0 || nrows <= r + 1 || ncols <= c + 1 {
        return 0;
    }
    let mut score: u32 = 1;
    let mut sum: u32 = 0;
    // left
    for j in (0..c).rev() {
        sum += 1;
        if input[r][j] >= input[r][c] {
            break;
        }
    }
    score *= sum;
    sum = 0;
    // right
    for j in c + 1..ncols {
        sum += 1;
        if input[r][j] >= input[r][c] {
            break;
        }
    }
    score *= sum;
    sum = 0;
    // up
    for i in (0..r).rev() {
        sum += 1;
        if input[i][c] >= input[r][c] {
            break;
        }
    }
    score *= sum;
    sum = 0;
    // down
    for i in r + 1..nrows {
        sum += 1;
        if input[i][c] >= input[r][c] {
            break;
        }
    }
    score *= sum;
    return score;
}
