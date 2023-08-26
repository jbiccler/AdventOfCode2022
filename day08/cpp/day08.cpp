#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

vector<vector<unsigned int>> parse_input(string path) {
  vector<vector<unsigned int>> lines;
  std::ifstream file(path);
  if (file.is_open()) {
    string line;
    while (std::getline(file, line)) {
      vector<unsigned int> tmp;
      for (auto c : line) {
        tmp.push_back(c - '0');
      }
      lines.push_back(tmp);
    }
    file.close();
  } else {
    std::cout << "Could not open file: " << path << std::endl;
  }
  return lines;
}

bool visible(vector<vector<unsigned int>> &input, unsigned int r,
             unsigned int c) {
  int nrows = input.size();
  int ncols = input[0].size();
  if (r <= 0 || c <= 0 || nrows <= r + 1 || ncols <= c + 1) {
    return true;
  } else {
    // left
    for (int j = c - 1; j >= 0; j--) {
      if (input[r][j] >= input[r][c]) {
        break;
      } else if (j == 0) {
        return true;
      }
    }
    // right
    for (int j = c + 1; j < ncols; j++) {
      if (input[r][j] >= input[r][c]) {
        break;
      } else if (j == ncols - 1) {
        return true;
      }
    }
    // up
    for (int j = r - 1; j >= 0; j--) {
      if (input[j][c] >= input[r][c]) {
        break;
      } else if (j == 0) {
        return true;
      }
    }
    // down
    for (int j = r + 1; j < nrows; j++) {
      if (input[j][c] >= input[r][c]) {
        break;
      } else if (j == nrows - 1) {
        return true;
      }
    }
  }
  return false;
}

unsigned int view_score(vector<vector<unsigned int>> &input, unsigned int r,
                        unsigned int c) {
  int nrows = input.size();
  int ncols = input[0].size();
  if (r <= 0 || c <= 0 || nrows <= r + 1 || ncols <= c + 1) {
    return 0;
  } else {
    unsigned int score = 1;
    unsigned int sum = 0;
    // left
    for (int j = c - 1; j >= 0; j--) {
      sum += 1;
      if (input[r][j] >= input[r][c]) {
        break;
      }
    }
    score *= sum;
    sum = 0;
    // right
    for (int j = c + 1; j < ncols; j++) {
      sum += 1;
      if (input[r][j] >= input[r][c]) {
        break;
      }
    }
    score *= sum;
    sum = 0;
    // up
    for (int j = r - 1; j >= 0; j--) {
      sum += 1;
      if (input[j][c] >= input[r][c]) {
        break;
      }
    }
    score *= sum;
    sum = 0;
    // down
    for (int j = r + 1; j < nrows; j++) {
      sum += 1;
      if (input[j][c] >= input[r][c]) {
        break;
      }
    }
    score *= sum;
    return score;
  }
}
int main(int argc, char *argv[]) {
  if (argc == 1) {
    return -1;
  } else {
    auto input = parse_input(argv[1]);
    unsigned int sum1{0};
    unsigned int best{0};
    unsigned int best_r{0};
    unsigned int best_c{0};
    for (int r = 0; r < input.size(); r++) {
      for (int c = 0; c < input[0].size(); c++) {
        if (visible(input, r, c)) {
          sum1 += 1;
        }
        auto curr = view_score(input, r, c);
        if (curr > best) {
          best = curr;
          best_r = r;
          best_c = c;
        }
      }
    }
    cout << "Number of trees visible from outside of the grid " << sum1 << endl;
    cout << "Tree at index " << best_r << "," << best_c
         << " has the best score: " << best << endl;
  }
  return 0;
}
