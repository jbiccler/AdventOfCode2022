#include <array>
#include <cmath>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <optional>
#include <string>
#include <vector>

using namespace std;

struct Move {
  int x;
  int y;
  Move(int x, int y) : x(x), y(y){};
};

struct Grid {
  vector<vector<bool>> grid;
  array<int, 2> pos;
  vector<vector<bool>> visited;
  unsigned int n_visited;
  Grid(vector<vector<bool>> grid, array<int, 2> pos,
       vector<vector<bool>> visited, unsigned int n_visited)
      : grid(grid), pos(pos), visited(visited), n_visited(n_visited){};

  unsigned int get_n_visited() { return this->n_visited; };
};

vector<Move> parse_input(string path) {
  vector<Move> lines;
  std::ifstream file(path);
  string line;
  if (file.is_open()) {
    while (getline(file, line)) {
      int idx = line.find(",");
      string dirstr = line.substr(0, idx);
      int nr = stoi(line.substr(idx + 1));
      array<int, 2> dir{0, 0};
      if (dirstr == "R") {
        dir[0] = 1;
        dir[1] = 0;
      } else if (dirstr == "L") {
        dir[0] = -1;
        dir[1] = 0;
      } else if (dirstr == "U") {
        dir[0] = 0;
        dir[1] = -1;
      } else if (dirstr == "D") {
        dir[0] = 0;
        dir[1] = 1;
      }
      if (dir[0] == 0 && dir[1] == 0) {
        cout << "Didn't match direction" << endl;
      }
      for (int i = 0; i < nr; i++) {
        lines.push_back(Move(dir[0], dir[1]));
      }
    }
  }
  return lines;
}

Grid construct_grid(int nrows, int ncols) {
  vector<bool> row(ncols, false);
  vector<vector<bool>> res(nrows, row);
  res[nrows / 2][ncols / 2] = true;
  // copy vector
  vector<vector<bool>> visited = res;
  return Grid(res, array<int, 2>{nrows / 2, ncols / 2}, visited, 1);
}

void make_move(Grid &grid, Move &m) {
  int row = grid.pos[0];
  int col = grid.pos[1];

  // set current position to 0
  grid.grid[row][col] = false;
  // update moved position
  int new_row = (row + m.x);
  int new_col = (col + m.y);
  grid.grid[new_row][new_col] = true;
  if (!grid.visited[new_row][new_col]) {
    grid.n_visited += 1;
    grid.visited[new_row][new_col] = true;
  }
  grid.pos[0] = new_row;
  grid.pos[1] = new_col;
}

optional<Move> determine_move(Grid &grid1, Grid &grid2) {
  auto [x1, y1] = grid1.pos;
  auto [x2, y2] = grid2.pos;

  if ((std::abs(x1 - x2) > 1) || (std::abs(y2 - y1) > 1)) {
    // need to make a move
    int x{0};
    int y{0};
    if (x1 > x2) {
      x = 1;
    } else if (x1 < x2) {
      x = -1;
    }
    if (y1 > y2) {
      y = 1;
    } else if (y1 < y2) {
      y = -1;
    }
    return Move(x, y);
  } else {
    // no move required
    return std::nullopt;
  }
}

int run(vector<Move> &moves, unsigned int n, unsigned int nrows,
        unsigned int ncols) {
  vector<Grid> grids;
  for (int i = 0; i < n; i++) {
    grids.push_back(construct_grid(nrows, ncols));
  }
  for (auto m : moves) {
    make_move(grids[0], m);
    for (int i = 1; i < n; i++) {
      auto newmove = determine_move(grids[i - 1], grids[i]);
      if (newmove) {
        make_move(grids[i], newmove.value());
      }
    }
  }
  return grids[n - 1].get_n_visited();
}

int main(int argc, char *argv[]) {
  if (argc <= 1) {
    return -1;
  } else {
    auto moves = parse_input(argv[1]);
    // TODO set these dynamically based on input or pass as arg
    unsigned int ncols = 1000;
    unsigned int nrows = 1000;
    cout << "Number of visited squares by tail of part1: "
         << run(moves, 2, nrows, ncols) << endl;
    cout << "Number of visited squares by tail of part2: "
         << run(moves, 10, nrows, ncols) << endl;
  }
  return 0;
};
