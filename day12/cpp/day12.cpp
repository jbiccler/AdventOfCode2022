#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <limits>
#include <string>
#include <vector>

using namespace std;

struct Edge {
  // store from and to as indices to nodes to keep them on the stack as grid
  // doesnt change once initialized
  array<unsigned int, 2> from;
  array<unsigned int, 2> to;
  int weight;
  Edge(array<unsigned int, 2> from, array<unsigned int, 2> to, int weight)
      : from(from), to(to), weight(weight){};
};

struct Node {
  vector<Edge> edges;
  int height;
  char c;
  array<unsigned int, 2> idx;
  Node(vector<Edge> edges, int height, char c, array<unsigned int, 2> idx)
      : edges(edges), height(height), c(c), idx(idx){};
};

struct Grid {
  vector<vector<Node>> nodes;
  array<unsigned int, 2> start;
  array<unsigned int, 2> end;
  Grid(vector<vector<Node>> nodes, array<unsigned int, 2> start,
       array<unsigned int, 2> end)
      : nodes(nodes), start(start), end(end){};
};

void check_construct_edge(Grid& grid, array<unsigned int, 2> from,
                          array<unsigned int, 2> to, bool reverse) {
  auto fr{from[0]};
  auto fc{from[1]};
  auto tr{to[0]};
  auto tc{to[1]};
  if (reverse) {
    if (grid.nodes[tr][tc].height - grid.nodes[fr][fc].height <= 1) {
      grid.nodes[tr][tc].edges.push_back(Edge(to, from, 1));
    }
  } else {
    if (grid.nodes[tr][tc].height - grid.nodes[fr][fc].height <= 1) {
      grid.nodes[fr][fc].edges.push_back(Edge(from, to, 1));
    }
  }
};

Grid parse_input(string path, bool reverse) {
  ifstream file(path);
  string line;
  string alphabet = "abcdefghijklmnopqrstuvwxyz";
  vector<vector<Node>> nodes;
  array<unsigned int, 2> start{0, 0};
  array<unsigned int, 2> end{0, 0};
  unsigned int i{0};
  if (file.is_open()) {
    while (getline(file, line)) {
      vector<Node> tmp;
      if (line.size() > 0) {
        for (unsigned int j = 0; j < line.size(); j++) {
          auto height = alphabet.find(line[j]);
          array<unsigned int, 2> idx{i, j};
          if (height >= 0) {
            tmp.push_back(Node(vector<Edge>{}, height, line[j], idx));
          } else if (line[j] == 'S') {
            // start node with height a  = 0
            tmp.push_back(Node(vector<Edge>{}, 0, line[j], idx));
            start = idx;
          } else if (line[j] == 'E') {
            tmp.push_back(Node(vector<Edge>{}, 25, line[j], idx));
            end = idx;
          }
        }
        i++;
        nodes.push_back(tmp);
      }
    }
  }
  auto nrows = nodes.size();
  auto ncols = nodes[0].size();
  Grid grid{nodes, start, end};

  // set edges
  for (unsigned int r; r < nrows; r++) {
    for (unsigned int c; c < ncols; c++) {
      array<unsigned int, 2> from{r, c};
      if (r > 0) {
        // up
        check_construct_edge(grid, from, array<unsigned int, 2>{r - 1, c},
                             reverse);
      }
      if (r < nrows - 1) {
        // down
        check_construct_edge(grid, from, array<unsigned int, 2>{r + 1, c},
                             reverse);
      }
      if (c > 0) {
        // left
        check_construct_edge(grid, from, array<unsigned int, 2>{r, c - 1},
                             reverse);
      }
      if (c < ncols - 1) {
        // right
        check_construct_edge(grid, from, array<unsigned int, 2>{r, c + 1},
                             reverse);
      }
    }
  }
  return grid;
}

int dijkstra(Grid& grid, array<unsigned int, 2> start_idx,
             array<unsigned int, 2> end_idx, int target_height) {
  // set target_height to -1 for default exhaustive search
  // distance matrix -> same structure as nodes
  vector<vector<int>> dist(
      grid.nodes.size(),
      vector<int>(grid.nodes[0].size(), std::numeric_limits<int>::max()));
  // setup queue of to be visited nodes
  vector<Node> queue;
  for (int r = 0; r < grid.nodes.size(); r++) {
    for (int c = 0; c < grid.nodes[0].size(); c++) {
      queue.push_back(grid.nodes[r][c]);
    }
  };
  // set distance of start position to 0;
  dist[start_idx[0]][start_idx[1]] = 0;
  array<unsigned int, 2> target_idx{0, 0};
  while (queue.size() > 0) {
    // find node with current minimum distance
    auto min_dist = std::numeric_limits<int>::max();
    auto min_idx = 0;
    for (int i = 0; i < queue.size(); i++) {
      if (dist[queue[i].idx[0]][queue[i].idx[1]] < min_dist) {
        min_idx = i;
        min_dist = dist[queue[i].idx[0]][queue[i].idx[1]];
      }
    }
    // pop the node with current min distance from the queue and update the
    // distances to the nodes that are reachable from this visited node
    if (min_idx >= 0 && min_idx < queue.size()) {
      // remove element and store in variable
      auto visited = queue[min_idx];
      cout << endl;
      cout << "visited " << visited.idx[0] << visited.idx[1] << endl;
      queue.erase(queue.begin() + min_idx);
      if (target_height >= 0 && visited.height == target_height) {
        // visited target end node -> stop
        target_idx = visited.idx;
        cout << "visited target_height" << visited.idx[0] << visited.idx[1]
             << endl;
        break;
      } else if (visited.idx == end_idx) {
        // visited end node -> stop
        target_idx = visited.idx;
        cout << "visited end_idx" << visited.idx[0] << visited.idx[1] << endl;
        break;
      }
      auto current_dist = dist[visited.idx[0]][visited.idx[1]];
      if (current_dist == std::numeric_limits<int>::max()) {
        // node that can't be visited
        // as only nodes with MAX distance are left...
        break;
      }
      for (auto e : visited.edges) {
        if (current_dist + e.weight < dist[e.to[0]][e.to[1]]) {
          dist[e.to[0]][e.to[1]] = current_dist + e.weight;
        }
      }
    }
  };
  cout << target_idx[0] << " " << target_idx[1] << endl;
  return dist[target_idx[0]][target_idx[1]];
};

void print_grid(Grid& grid, bool print_chars) {
  for (auto r = 0; r < grid.nodes.size(); r++) {
    for (auto c = 0; c < grid.nodes[0].size(); c++) {
      if (print_chars) {
        cout << grid.nodes[r][c].c;
      } else {
        cout << grid.nodes[r][c].height;
      }
    }
    cout << endl;
  }
};

int main(int argc, char* argv[]) {
  if (argc > 1) {
    auto grid1 = parse_input(argv[1], false);
    print_grid(grid1, true);
    cout << "Part 1 -- number of steps required: "
         << dijkstra(grid1, grid1.start, grid1.end, -1) << endl;
    auto grid2 = parse_input(argv[1], true);
    cout << "Part 2 -- number of steps required: "
         << dijkstra(grid2, grid2.end, grid2.end, 0) << endl;
  }
}
