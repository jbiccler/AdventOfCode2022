use num::integer::gcd;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::{env, io};

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: Operation,
    div: u64,
    next_true: usize,
    next_false: usize,
    n_inspected: u64,
}

#[derive(Debug, Clone, Copy)]
enum OperationType {
    Addition,
    Multiplication,
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    op: OperationType,
    val: u64,
    op_self: bool,
}

fn parse_input(path: &String) -> io::Result<Vec<Monkey>> {
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(7);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // define a regex pattern per line of expected input
    // let monkey_pattern = Regex::new(r"Monkey (\d+):").unwrap();
    let starting_pattern = Regex::new(r"Starting items: (.*)").unwrap();
    let operation_pattern = Regex::new(r"Operation: new = old (\*|\+) (\d+|old)").unwrap();
    let test_pattern = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let true_pattern = Regex::new(r"If true: .*(\d+)").unwrap();
    let false_pattern = Regex::new(r"If false: .*(\d+)").unwrap();
    // define placeholders for each type of input
    let mut items: VecDeque<u64> = VecDeque::new();
    let mut op: Operation = Operation {
        op: OperationType::Addition,
        val: 0,
        op_self: false,
    };
    let mut div: u64 = 1;
    let mut true_monkey: usize = 0;
    let mut false_monkey: usize;
    // read each line
    for line in reader.lines() {
        let line = line?;
        // parse the different pattern possiblities
        if let Some(cap) = starting_pattern.captures(&line) {
            items = cap
                .get(1)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
        } else if let Some(cap) = operation_pattern.captures(&line) {
            let operator = cap.get(1).unwrap().as_str();
            let op_type = match operator {
                "+" => OperationType::Addition,
                "*" => OperationType::Multiplication,
                _ => return Err(Error::new(ErrorKind::InvalidData, "Operator not supported")),
            };
            let target = cap.get(2).unwrap().as_str();
            if target == "old" {
                op = Operation {
                    op: op_type,
                    val: 0,
                    op_self: true,
                }
            } else {
                op = Operation {
                    op: op_type,
                    val: target.parse::<u64>().unwrap(),
                    op_self: false,
                }
            }
        } else if let Some(cap) = test_pattern.captures(&line) {
            div = cap.get(1).unwrap().as_str().parse::<u64>().unwrap();
        } else if let Some(cap) = true_pattern.captures(&line) {
            true_monkey = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        } else if let Some(cap) = false_pattern.captures(&line) {
            false_monkey = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
            // assume last_monkey input is always last line per monkey
            monkeys.push(Monkey {
                items: items.clone(),
                op: op.clone(),
                div: div.clone(),
                next_true: true_monkey,
                next_false: false_monkey,
                n_inspected: 0,
            });
            items.clear();
        }
    }
    return Ok(monkeys);
}

fn calculate_lcm(a: u64, b: u64) -> u64 {
    // find lowest common multiple based on stdlib greatest common divisor.
    (a * b) / gcd(a, b)
}

fn execute_round(monkeys: &mut Vec<Monkey>, whole_divide_3: bool, lcm: u64) {
    for m in 0..monkeys.len() {
        // while there are still items for this monkey
        // process them, pass them along and clear the item from the list
        while monkeys[m].items.len() > 0 {
            // remove this item from the current monkey's queue
            let mut item = monkeys[m].items.pop_front().unwrap();
            // do worry level operation
            if monkeys[m].op.op_self {
                match monkeys[m].op.op {
                    OperationType::Addition => item += item,
                    OperationType::Multiplication => item *= item,
                };
            } else {
                match monkeys[m].op.op {
                    OperationType::Addition => item += monkeys[m].op.val,
                    OperationType::Multiplication => item *= monkeys[m].op.val,
                };
            };
            if whole_divide_3 {
                // monkey gets bored -> whole division by 3
                item /= 3;
            } else if item > lcm {
                // avoid integer overflow by modula LCM operation
                item = item % lcm;
            }
            // test divisibility
            let current = item.clone();
            if current % monkeys[m].div == 0 {
                // pass to monkey in true case
                let nt = monkeys[m].next_true;
                monkeys[nt].items.push_back(current);
            } else {
                // pass to monkey in false case
                let nt = monkeys[m].next_false;
                monkeys[nt].items.push_back(current);
            }
            monkeys[m].n_inspected += 1;
        }
    }
}

fn score(monkeys: &Vec<Monkey>) -> Result<u64, String> {
    // score is the multiplication of the number of inspections of the two monkeys with the most inpsections.
    if monkeys.len() < 2 {
        return Err("Not enough monkeys to score".to_string());
    }
    let mut res: Vec<u64> = Vec::with_capacity(monkeys.len());
    for m in 0..monkeys.len() {
        res.push(monkeys[m].n_inspected);
    }
    res.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    return Ok(res[0] * res[1]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut monkeys = parse_input(&args[1]).unwrap();
        // take clone to avoid having to parse input again for part2
        let mut monkeys2 = monkeys.clone();
        println!("Part 1, 20 rounds ");
        for _ in 0..20 {
            execute_round(&mut monkeys, true, 0);
        }
        println!("Monkey business: {}", score(&monkeys).unwrap());
        println!("Part 20, 10000 rounds ");
        let lcm: u64 = monkeys2.iter().fold(monkeys2[0].div, |acc, monkey| {
            calculate_lcm(acc, monkey.div)
        });
        for _ in 0..10000 {
            execute_round(&mut monkeys2, false, lcm);
        }
        println!("Monkey business: {}", score(&monkeys2).unwrap());
    }
}
