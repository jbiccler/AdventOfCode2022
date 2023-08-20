use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}",&args[1]);
    let _ = parse_input(&args[1]);
}

fn parse_input(file_path : &String) -> std::io::Result<()>{
    let mut sum: u32 = 0;
    let mut c: usize = 0;
    let mut elves = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line_str = line.unwrap();
        if line_str.is_empty(){
            sum = 0;
            c += 1;
        } else {
            sum += line_str.parse::<u32>().unwrap();
            if c < elves.len(){
                elves[c] = sum;
            } else {
                elves.push(sum);
            }
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    sum = 0;
    for i in 0..3{
        println!("Sorted elf {} is carrying {} cals", i, elves[i]);
        sum += elves[i];
    }
    println!("For a total of {}", sum);
    return Ok(())
}