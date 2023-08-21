use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Input file: {}", args[1]);
        let _res1 = parse_input_part1(&args[1]).unwrap();
        let _res2 = parse_input_part2(&args[1]).unwrap();
    }
}

fn find_common(v: &Vec<&str>) -> HashSet<char> {
    if v.len() < 2 {
        return HashSet::new();
    }
    let mut sep: Vec<HashSet<char>> = Vec::new();
    for i in 0..v.len() {
        let mut s: HashSet<char> = HashSet::new();
        for j in 0..v[i].len() {
            s.insert(v[i].chars().nth(j).unwrap());
        }
        sep.push(s);
    }
    let mut result: HashSet<char> = sep.iter().nth(0).cloned().unwrap();
    for i in sep.iter() {
        for c in result.clone().iter() {
            if !i.contains(&c) {
                result.remove(&c);
            }
        }
    }

    return result;
}

fn split_half(str: &String) -> Vec<&str> {
    let n = str.len();
    let mut res: Vec<&str> = Vec::new();
    res.push(&str[0..n / 2]);
    res.push(&str[n / 2..]);
    return res;
}

fn score(s: HashSet<char>) -> i32 {
    let alphabet = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let mut sum: i32 = 0;
    let _ = s
        .into_iter()
        .map(|c| sum += alphabet.find(c).unwrap() as i32 + 1)
        .collect::<HashSet<_>>();
    return sum;
}

fn parse_input_part1(path: &String) -> std::io::Result<()> {
    // open file
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    // call splitting functions
    for line in reader.lines() {
        let line_str = line?;
        let halves = split_half(&line_str);
        let common_part = find_common(&halves);
        sum += score(common_part);
    }
    println!("Total sum for part1 is {}", sum);
    return Ok(());
}

fn parse_input_part2(path: &String) -> std::io::Result<()> {
    // open file
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    let bp: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut vec1: Vec<&str> = Vec::new();
    for (i, line) in bp.iter().enumerate() {
        if i % 3 == 0 {
            vec1.clear();
        }
        vec1.push(line);
        if vec1.len() == 3 {
            let common_part = find_common(&vec1);
            sum += score(common_part);
        }
    }
    println!("Total sum for part2 is {}", sum);
    return Ok(());
}
