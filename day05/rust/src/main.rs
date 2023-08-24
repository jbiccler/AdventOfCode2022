use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Input file: {}", args[1]);
        let (start, moves) = parse_input(&args[1], 10, 9);
        println!("------------------Start config-----------------");
        print_result(&start);
        let result_part1 = make_moves(start.clone(), &moves, false);
        println!("------------------Part 1 config-----------------");
        print_result(&result_part1);
        println!("------------------Part 1 answer-----------------");
        print_answer(&result_part1);
        let result_part2 = make_moves(start, &moves, true);
        println!("------------------Part 2 config-----------------");
        print_result(&result_part2);
        println!("------------------Part 2 answer-----------------");
        print_answer(&result_part2);
    }
}

fn parse_input(
    path: &String,
    start_row: usize,
    num_cols: usize,
) -> (Vec<Vec<char>>, Vec<[usize; 3]>) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    let mut start: Vec<Vec<char>> = Vec::new();

    for i in 0..num_cols {
        let mut tmp: Vec<char> = Vec::new();
        for r in (0..=(start_row as usize - 3)).rev() {
            let q = lines[r].chars().nth(1 + i * 4).unwrap();
            if q != ' ' {
                tmp.push(q);
            }
        }
        start.push(tmp);
    }

    let re: Regex = Regex::new(".*move (\\d+) from (\\d+) to (\\d+).*").unwrap();
    let mut moves: Vec<[usize; 3]> = Vec::new();
    let mut matches: [usize; 3] = [0, 0, 0];
    for i in start_row..lines.len() {
        if let Some(captures) = re.captures(&lines[i][..]) {
            for (c, capture) in captures.iter().enumerate().skip(1) {
                if let Some(capture) = capture {
                    matches[c - 1] = capture.as_str().parse::<usize>().unwrap();
                    // matches.push(capture.as_str().to_string().parse::<i32>().unwrap());
                }
            }
        }
        moves.push(matches.clone());
    }
    return (start, moves);
}

fn make_moves(
    mut current: Vec<Vec<char>>,
    moves: &Vec<[usize; 3]>,
    move_all_at_once: bool,
) -> Vec<Vec<char>> {
    for m in 0..moves.len() {
        let n = moves[m][0];
        let fr: usize = moves[m][1] - 1;
        let to: usize = moves[m][2] - 1;
        if move_all_at_once {
            let mut tmp: Vec<char> = Vec::new();
            for _j in 0..n {
                if let Some(last_element) = current[fr].pop() {
                    tmp.push(last_element);
                }
            }
            for x in tmp.iter().rev() {
                current[to].push(*x);
            }
        } else {
            for _j in 0..n {
                if let Some(last_element) = current[fr].pop() {
                    current[to].push(last_element);
                }
            }
        }
    }
    return current;
}

fn print_result(result: &Vec<Vec<char>>) {
    for l in result.iter() {
        for c in l.iter() {
            print!("{} ", c);
        }
        println!("");
    }
}

fn print_answer(result: &Vec<Vec<char>>) {
    for l in result.iter() {
        println!("{}", l[l.len() - 1]);
    }
}
