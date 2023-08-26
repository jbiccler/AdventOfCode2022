use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Input file: {}", args[1]);
        let line = parse_input(&args[1]).unwrap();
        let mark1 = find_mark(&line, 4).unwrap();
        println!("Mark of length 4 found at: {}", mark1);
        let mark2 = find_mark(&line, 14).unwrap();
        println!("Mark of length 14 found at: {}", mark2);
    }
}

fn parse_input(path: &String) -> io::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    return reader.lines().next().unwrap();
}

fn find_mark(str: &String, len: usize) -> Option<usize> {
    for i in 0..(str.len() - len) {
        if has_unique_characters(&str[i..i + len]) {
            return Some(i + len);
        }
    }
    return None;
}

fn has_unique_characters(input: &str) -> bool {
    // assuming ASCII encoding;
    if input.len() > 128 {
        return false; // More characters than possible ASCII characters, so not unique
    }

    let mut seen_chars = [false; 128];

    for &byte in input.as_bytes() {
        if seen_chars[byte as usize] {
            return false; // Character already seen, not unique
        }
        seen_chars[byte as usize] = true;
    }
    true // All characters are unique
}
