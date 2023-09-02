use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

#[derive(Debug, Clone, PartialEq)]
struct Edge {
    // store from and to as indices to nodes to keep them on the stack as grid doesnt change once initialized
    from: (usize, usize),
    to: (usize, usize),
    weight: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    edges: Vec<Edge>,
    height: usize,
    c: char,
    idx: (usize, usize),
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
        let grid = parse_input(&args[1], false).unwrap();
        print_grid(&grid, true);
        // Part 1
        println!(
            "Part 1 -- number of steps required: {}",
            dijkstra(&grid, grid.start, Some(grid.end), None)
        );

        // Part 2
        let grid2 = parse_input(&args[1], true).unwrap();
        println!(
            "Part 2 -- number of steps required: {}",
            dijkstra(&grid2, grid2.end, None, Some(0))
        );
    }
}

fn parse_input(path: &String, reverse: bool) -> io::Result<Grid> {
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
                        idx: (i, j),
                    });
                } else if c == 'S' {
                    // start node has height a = 0
                    tmp.push(Node {
                        edges: Vec::new(),
                        height: 0,
                        c,
                        idx: (i, j),
                    });
                    start = (i, j);
                } else if c == 'E' {
                    // end node has height z = 25
                    tmp.push(Node {
                        edges: Vec::new(),
                        height: 25,
                        c,
                        idx: (i, j),
                    });
                    end = (i, j);
                }
            }
            nodes.push(tmp);
        }
    }
    let nrows = nodes.len();
    let ncols = nodes[0].len();
    let mut grid = Grid { nodes, start, end };

    for r in 0..nrows {
        for c in 0..ncols {
            // up
            if r > 0 {
                check_construct_edge(&mut grid, (r, c), (r - 1, c), reverse);
            }
            // down
            if r < nrows - 1 {
                check_construct_edge(&mut grid, (r, c), (r + 1, c), reverse);
            }
            // left
            if c > 0 {
                check_construct_edge(&mut grid, (r, c), (r, c - 1), reverse);
            }
            // right
            if c < ncols - 1 {
                check_construct_edge(&mut grid, (r, c), (r, c + 1), reverse);
            }
        }
    }
    return Ok(grid);
}

fn check_construct_edge(grid: &mut Grid, from: (usize, usize), to: (usize, usize), reverse: bool) {
    let (fr, fc) = from;
    let (tr, tc) = to;
    if reverse {
        if grid.nodes[tr][tc]
            .height
            .saturating_sub(grid.nodes[fr][fc].height)
            <= 1
        {
            grid.nodes[tr][tc].edges.push(Edge {
                from: to,
                to: from,
                weight: 1,
            });
        }
    } else {
        if grid.nodes[tr][tc]
            .height
            .saturating_sub(grid.nodes[fr][fc].height)
            <= 1
        {
            grid.nodes[fr][fc].edges.push(Edge {
                from: from,
                to: to,
                weight: 1,
            });
        }
    }
}

fn dijkstra(
    grid: &Grid,
    start_idx: (usize, usize),
    end_idx: Option<(usize, usize)>,
    target_height: Option<usize>,
) -> i32 {
    if end_idx == None && target_height == None {
        return -1;
    }
    // distance matrix -> same structure as nodes
    let mut dist: Vec<Vec<i32>> = vec![vec![std::i32::MAX; grid.nodes[0].len()]; grid.nodes.len()];
    // setup queue of to be visited nodes
    let mut queue: VecDeque<Node> = VecDeque::with_capacity(grid.nodes.len() * grid.nodes[0].len());
    // add start as first
    for r in 0..grid.nodes.len() {
        for c in 0..grid.nodes[0].len() {
            queue.push_back(grid.nodes[r][c].clone());
        }
    }
    // set start position to dist 0
    dist[start_idx.0][start_idx.1] = 0;
    // initialize end index
    let mut target_idx: (usize, usize) = (0, 0);
    while queue.len() > 0 {
        // find node with current minimum distance
        let mut min_dist: i32 = std::i32::MAX;
        let mut min_idx: usize = 0;
        for (i, n) in queue.iter().enumerate() {
            if dist[n.idx.0][n.idx.1] < min_dist {
                min_idx = i;
                min_dist = dist[n.idx.0][n.idx.1];
            }
        }
        // pop the node with current min distance from the queue and update the distances to the
        // nodes that are reachable from this visited node
        if let Some(visited) = queue.remove(min_idx) {
            if let Some(target) = target_height {
                if visited.height == target {
                    // visiting end node -> stop
                    target_idx = visited.idx;
                    break;
                }
            } else if let Some(end) = end_idx {
                if visited.idx == end {
                    // visiting end node -> stop
                    target_idx = visited.idx;
                    break;
                }
            }

            let current_dist = dist[visited.idx.0][visited.idx.1];
            if current_dist == std::i32::MAX {
                // node that can't be visited
                // as only nodes with MAX distance are left...
                break;
            }
            for e in visited.edges {
                if current_dist + e.weight < dist[e.to.0][e.to.1] {
                    dist[e.to.0][e.to.1] = current_dist + e.weight;
                }
            }
        }
    }
    return dist[target_idx.0][target_idx.1];
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
