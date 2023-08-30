use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

#[derive(Debug, Clone)]
struct Edge {
    // store from and to as indices to nodes to keep them on the stack as grid doesnt change once initialized
    from: (usize, usize),
    to: (usize, usize),
    weight: i32,
}

#[derive(Debug, Clone)]
struct Node {
    edges: Vec<Edge>,
    height: usize,
    c: char,
}

#[derive(Debug, Clone)]
struct Grid {
    nodes: Vec<Vec<Node>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let grid = parse_input(&args[1]).unwrap();
        print_grid(&grid, true);
    }
}

fn parse_input(path: &String) -> io::Result<Grid> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut nodes: Vec<Vec<Node>> = Vec::new();
    // set up grid
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        // assume input is well behaved
        // i.e. only valid characters and all lowercase
        // iterate over characters
        let mut tmp: Vec<Node> = Vec::new();
        if line.len() > 0 {
            for (j, c) in line.chars().enumerate() {
                // find corresponding height, i.e. index in alphabet
                if let Some(height) = alphabet.find(c) {
                    tmp.push(Node {
                        edges: Vec::new(),
                        height,
                        c,
                    });
                } else if c == 'S' {
                    // start node has height a
                    tmp.push(Node {
                        edges: Vec::new(),
                        height: 0,
                        c,
                    });
                    start = (i, j);
                } else if c == 'E' {
                    // start node has height a
                    tmp.push(Node {
                        edges: Vec::new(),
                        height: 25,
                        c,
                    });
                    end = (i, j);
                }
            }
            nodes.push(tmp);
        }
    }
    let nrows = nodes.len();
    let ncols = nodes[0].len();
    println!("nrows: {}, ncols: {}", nrows, ncols);
    let mut grid = Grid { nodes, start, end };

    // set edges
    for r in 0..nrows {
        for c in 0..ncols {
            // up
            if r > 0 {
                if grid.nodes[r - 1][c]
                    .height
                    .saturating_sub(grid.nodes[r][c].height)
                    <= 1
                {
                    grid.nodes[r][c].edges.push(Edge {
                        from: (r, c),
                        to: (r - 1, c),
                        weight: 1,
                    });
                }
            }
            // down
            if r < nrows - 1 {
                if grid.nodes[r + 1][c]
                    .height
                    .saturating_sub(grid.nodes[r][c].height)
                    <= 1
                {
                    grid.nodes[r][c].edges.push(Edge {
                        from: (r, c),
                        to: (r + 1, c),
                        weight: 1,
                    });
                }
            }
            // left
            if c > 0 {
                if grid.nodes[r][c - 1]
                    .height
                    .saturating_sub(grid.nodes[r][c].height)
                    <= 1
                {
                    grid.nodes[r][c].edges.push(Edge {
                        from: (r, c),
                        to: (r, c - 1),
                        weight: 1,
                    });
                }
            }
            // right
            if c < ncols - 1 {
                if grid.nodes[r][c + 1]
                    .height
                    .saturating_sub(grid.nodes[r][c].height)
                    <= 1
                {
                    grid.nodes[r][c].edges.push(Edge {
                        from: (r, c),
                        to: (r, c + 1),
                        weight: 1,
                    });
                }
            }
        }
    }
    return Ok(grid);
}

fn dijkstra(grid: &Grid) {
    let dist: Vec<Vec<i32>> = vec![vec![0; grid.nodes[0].len()]; grid.nodes.len()];
    let prev: Vec<Vec<Option<(usize, usize)>>> =
        vec![vec![None; grid.nodes[0].len()]; grid.nodes.len()];

    let queue : Vec<Node> = Vec::with_capacity();
    for r in 0..grid.nodes.len(){
        for c in 0..grid.nodes[0].len()[
            queue.push(nodes.clone());
        ]
    }
}

fn print_grid(grid: &Grid, print_chars: bool) {
    for r in 0..grid.nodes.len() {
        for c in 0..grid.nodes[0].len() {
            if print_chars {
                print!("{}", grid.nodes[r][c].c);
            } else {
                print!("{}", grid.nodes[r][c].height);
            }
        }
        println!("");
    }
}
