use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Input file: {}", args[1]);
        let _res = parse_input(&args[1]).unwrap();
    }
}

fn split<'a>(str: &'a String, delimiter : &str) -> [&'a str;2]{
    let mut v : [&str;2] = ["",""];
    let delimiter_index = str.find(delimiter);
    match delimiter_index {
        Some(i) => {
            v[0] = &str[0..i];
            v[1] = &str[(i+delimiter.len())..];},
        None => {println!("No match found")},
    };
    return v;
}

fn to_num(str : &str) -> [i32;2] {
    let str_conv = str.to_string();
    let v = split(&str_conv, "-");
    let res : [i32;2] = [v[0].parse().unwrap(), v[1].parse().unwrap()];
    return res
}

fn full_overlap(left : &[i32;2], right: &[i32;2]) -> i32{
  if (left[0] >= right[0]) && (left[1] <= right[1]) {
    return 1
  } else if (right[0] >= left[0]) && (right[1] <= left[1]) {
    return 1
  } else {
    return 0
  }
}
fn partial_overlap(left: &[i32; 2], right: &[i32;2]) -> i32{
  if (left[0] <= right[1]) && (left[1] >= right[0]) {
    return 1
  } else if (right[0] <= left[1]) && (right[1] >= left[0]) {
    return 1
  } else {
    return 0
  }
}

fn parse_input(path: &String) -> std::io::Result<()> {
    // open file
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    // call splitting functions
    for line in reader.lines() {
        let line_str = line?;
        let spl = split(&line_str,",");
        let left = to_num(spl[0]);
        let right = to_num(spl[1]);
        sum_part1 += full_overlap(&left,&right);
        sum_part2 += partial_overlap(&left,&right);
    }
    println!("Total sum for part 1 is {}", sum_part1);
    println!("Total sum for part 2 is {}", sum_part2);
    return Ok(());
}
