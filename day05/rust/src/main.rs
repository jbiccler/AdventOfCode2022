use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Input file: {}", args[1]);
        let _res = parse_input(&args[1]).unwrap();
    }
}

fn parse_input(path: String, start_row: i32, num_cols: i32) -> std::io::Result(){
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines = reader.lines().map(String::from).collect();

    let start: Vec<Vec<char>> = Vec::new(Vec::new());

    for i in 0..num_cols {
        tmp : Vec<char> = Vec::new();
        for r in (0..=(start_row-3)).rev(){
            if lines[r].chars().nth(1+i*4) != ' ' {
                tmp.push(lines[r].chars().nth(1+i*4));
            }
        }
        res.push(tmp);
    }

    let RE : Regex = Regex::new(".*move (\\d+) from (\\d+) to (\\d+).*").unwrap();
    let moves : Vec<[i32;3] = Vec::new();
    let matches : Vec<String> = Vec::new();
    for i in start_row..lines.len(){
        if let Some(captures) = RE.captures(lines[r]) {
            for capture in captures.iter().skip(1) {
                if let Some(capture) = capture {
                    matches.push(capture.as_str().to_string());
                }
            }
        if matches.len() == 3 {
            moves.push(matches);
        }
        //     let groups : [i32;3] = [0,0,0];
        //     for g in 0..3{
        //         groups[g] = matches[g]
        //     }
    }
}
};

