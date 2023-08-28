use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

#[derive(Debug, Clone)]
struct State {
    val: i32,
    op: i32,
    time: usize,
}

#[derive(Debug, Clone)]
struct Operation {
    addx: bool,
    val: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // get operations
        let ops = parse_input(&args[1]).unwrap();
        // get states from operations
        let states = generate_states(&ops);
        // score operations
        println!("Result for part1: {}", score(&states));
        println!("Part 2: ");
        // check if pixel is drawn on CRT screen and print
        let crt = generate_crt(&states, 40, 6);
        print_screen(&crt);
    }
}

fn parse_input(path: &String) -> io::Result<Vec<Operation>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut ops: Vec<Operation> = Vec::with_capacity(100);
    // read per line and split on delimiter
    for line in reader.lines() {
        let linestr = line?.clone();
        let mut spl = linestr.split(",");
        let op = spl.next().unwrap();
        let valstr = spl.next().unwrap();
        let mut val = 0;
        if valstr.len() > 0 {
            val = valstr.parse::<i32>().unwrap();
        }
        // match on types of operation, addx or noop
        // append corresponding operation to ops
        match op {
            "addx" => ops.push(Operation {
                addx: true,
                val: val,
            }),
            "noop" => ops.push(Operation {
                addx: false,
                val: val,
            }),
            _ => {
                return Err(io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "No match found on input",
                ))
            }
        }
    }

    return Ok(ops);
}

fn generate_states(ops: &Vec<Operation>) -> Vec<State> {
    let mut states: Vec<State> = Vec::with_capacity(ops.len() * 2);
    let mut current: i32 = 1;
    // for each operation check which type it is and append the corresponding state to states
    // also keep depleting the time parameter until all time has passed for that operation and CPU is free to move to next op
    for op in ops.iter() {
        let mut state = match op.addx {
            false => State {
                val: current,
                time: 1,
                op: 0,
            },
            true => State {
                val: current,
                time: 2,
                op: op.val,
            },
        };
        while state.time > 0 {
            states.push(state.clone());
            state.time -= 1;
        }
        current += state.op;
    }
    return states;
}

fn score(states: &Vec<State>) -> i32 {
    // calculate score as per the puzzle description
    let mut i = 20;
    let mut score = 0;
    while i < states.len() - 1 {
        score += i as i32 * states[i - 1].val;
        i += 40;
    }
    return score;
}

fn generate_crt(states: &Vec<State>, width: usize, height: usize) -> Vec<Vec<bool>> {
    // generate vector of vector of booleans that represent the pixels on the CRT screen
    // True if drawn, fals otherwise.
    let mut screen = vec![vec![false; width]; height];
    for r in 0..height {
        for c in 0..width {
            let state = &states[r * width + c];
            if (state.val - 1) <= c as i32 && c as i32 <= (state.val + 1) {
                screen[r][c] = true;
            }
        }
    }

    return screen;
}

fn print_screen(screen: &Vec<Vec<bool>>) {
    // print the screen to stdout
    for r in 0..screen.len() {
        for c in 0..screen[0].len() {
            if screen[r][c] {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
