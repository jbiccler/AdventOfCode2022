use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", &args[1]);
        let _ = parse_input(&args[1]);
    }
}

fn parse_input(path: &String) -> std::io::Result<()> {
    // open file
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // track sums of part 1 and 2
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for line in reader.lines() {
        let line_str = line.unwrap();
        sum_part1 += score_part1(&line_str).unwrap();
        sum_part2 += score_part2(&line_str).unwrap();
    }
    println!("{}", sum_part1);
    println!("{}", sum_part2);
    return Ok(());
}

fn score_part1(str: &String) -> Result<i32, String> {
    let first = str.chars().nth(0).unwrap();
    let snd = str.chars().nth(2).unwrap();
    let win_draw_lose: i32;

    match first {
        'A' => win_draw_lose = (snd == 'X') as i32 * 3 + (snd == 'Y') as i32 * 6,
        'B' => win_draw_lose = (snd == 'Y') as i32 * 3 + (snd == 'Z') as i32 * 6,
        'C' => win_draw_lose = (snd == 'Z') as i32 * 3 + (snd == 'X') as i32 * 6,
        _ => return Err("First character does not match any of [A,B,C]".to_string()),
    }
    return Ok(win_draw_lose
        + (snd == 'X') as i32 * 1
        + (snd == 'Y') as i32 * 2
        + (snd == 'Z') as i32 * 3);
}

fn score_part2(str: &String) -> Result<i32, String> {
    let first = str.chars().nth(0).unwrap();
    let snd = str.chars().nth(2).unwrap();
    let win_draw_lose: i32 =
        (snd == 'X') as i32 * 0 + (snd == 'Y') as i32 * 3 + (snd == 'Z') as i32 * 6;
    match first {
        'A' => {
            return Ok(win_draw_lose
                + (snd == 'X') as i32 * 3
                + (snd == 'Y') as i32 * 1
                + (snd == 'Z') as i32 * 2)
        }
        'B' => {
            return Ok(win_draw_lose
                + (snd == 'X') as i32 * 1
                + (snd == 'Y') as i32 * 2
                + (snd == 'Z') as i32 * 3)
        }
        'C' => {
            return Ok(win_draw_lose
                + (snd == 'X') as i32 * 2
                + (snd == 'Y') as i32 * 3
                + (snd == 'Z') as i32 * 1)
        }
        _ => return Err("No matching input found".to_string()),
    }
}
