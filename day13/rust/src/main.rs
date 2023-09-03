use core::fmt;
use serde::Deserialize;
use std::cmp::Ordering;
use std::env;
use std::fs;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Number(n) => write!(f, "{n}"),
            Node::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Node::List(n) => f(&n[..]),
            Node::Number(n) => f(&[Node::Number(*n)]),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => a.partial_cmp(b),
            (l, r) => l.with_slice(|l| r.with_slice(|r| l.partial_cmp(r))),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        // part 1
        let mut sum = 0;
        for (i, groups) in fs::read_to_string(&args[1])
            .unwrap()
            .split("\n\n")
            .filter(|s| !s.is_empty())
            .enumerate()
        {
            let i = i + 1;
            let mut nodes = groups
                .lines()
                .map(|line| serde_json::from_str::<Node>(line).unwrap());
            let l = nodes.next().unwrap();
            let r = nodes.next().unwrap();
            if l < r {
                sum += i;
            }
        }
        println!("Part 1 sum: {sum}");
        // part 2
        let dividers = vec![
            Node::List(vec![Node::Number(2)]),
            Node::List(vec![Node::Number(6)]),
        ];
        // let mut packets = include_str!("../../day13.txt")
        let mut packets = fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .filter(|s| !s.is_empty())
            .map(|line| serde_json::from_str::<Node>(line).unwrap())
            .chain(dividers.iter().cloned())
            .collect::<Vec<_>>();
        packets.sort();
        let decoder_key = dividers
            .iter()
            .map(|d| packets.binary_search(d).unwrap() + 1)
            .product::<usize>();
        println!("Part 2 decoder key: {decoder_key}");
    }
}
