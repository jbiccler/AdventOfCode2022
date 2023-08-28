use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

#[derive(Clone)]
struct Move {
    x: i32,
    y: i32,
}
struct Grid {
    grid: Vec<Vec<bool>>,
    pos: (usize, usize),
    visited: Vec<Vec<bool>>,
    n_visited: usize,
}
impl Grid {
    fn get_n_visited(&self) -> usize {
        self.n_visited
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let moves = input_parse(&args[1]).unwrap();
        // TODO set these dynamically based on input or pass as arg
        let nrows = 1000;
        let ncols = 1000;
        // part 1
        println!(
            "Number of visited squares by tail of part 1: {}",
            run(&moves, 2, ncols, nrows)
        );
        // part 2
        println!(
            "Number of visited squares by tail of part 1: {}",
            run(&moves, 10, ncols, nrows)
        );
    }
}

fn run(moves: &Vec<Move>, n: usize, ncols: usize, nrows: usize) -> usize {
    let mut grids: Vec<Grid> = (0..n).map(|_x| construct_grid(nrows, ncols)).collect();
    for m in moves {
        make_move(&mut grids[0], &m);
        for i in 1..n {
            if let Some(newmove) = determine_move(&grids[i - 1], &grids[i]) {
                make_move(&mut grids[i], &newmove);
            }
        }
    }
    return grids[n.saturating_sub(1)].get_n_visited();
}

fn input_parse(path: &String) -> io::Result<Vec<Move>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut res: Vec<Move> = Vec::new();
    for line in reader.lines() {
        let linestr = line?.clone();
        let mut spl = linestr.split(",");
        let dirstr = spl.next().unwrap();
        let nr: i32 = spl.next().unwrap().parse::<i32>().unwrap();
        let dir: (i32, i32) = match dirstr {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => (0, 0),
        };
        for _n in 0..nr {
            res.push(Move { x: dir.0, y: dir.1 })
        }
    }
    return Ok(res);
}

fn construct_grid(nrows: usize, ncols: usize) -> Grid {
    let mut grid: Vec<Vec<bool>> = vec![vec![false; ncols]; nrows];
    grid[nrows / 2][ncols / 2] = true;
    let visited = grid.clone();
    return Grid {
        grid,
        pos: (nrows / 2, ncols / 2),
        visited,
        n_visited: 1,
    };
}

fn make_move(grid: &mut Grid, m: &Move) {
    let row = grid.pos.0;
    let col = grid.pos.1;

    // set current position to 0
    grid.grid[row][col] = false;
    // update moved position
    let new_row = (row as i32 + m.x) as usize;
    let new_col = (col as i32 + m.y) as usize;
    grid.grid[new_row][new_col] = true;
    if !grid.visited[new_row][new_col] {
        grid.n_visited += 1;
        grid.visited[new_row][new_col] = true;
    }
    grid.pos = (new_row, new_col);
}

fn determine_move(grid1: &Grid, grid2: &Grid) -> Option<Move> {
    let (x1, y1) = (grid1.pos.0 as i32, grid1.pos.1 as i32);
    let (x2, y2) = (grid2.pos.0 as i32, grid2.pos.1 as i32);
    if x1.abs_diff(x2) > 1 || y1.abs_diff(y2) > 1 {
        // need to make a move
        Some(Move {
            x: (x1 - x2).signum(),
            y: (y1 - y2).signum(),
        })
    } else {
        // no move required
        return None;
    }
}
