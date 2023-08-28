#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

struct State {
  int val;
  int op;
  unsigned int time;
  State(int val, int op, unsigned int time) : val(val), op(op), time(time){};
};

struct Operation {
  bool addx;
  int val;
  Operation(bool addx, int val) : addx(addx), val(val){};
};

vector<Operation> parse_input(string path) {
  std::ifstream file(path);
  std::string line;
  vector<Operation> ops;
  if (file.is_open()) {
    // read per line and split on delimiter
    while (getline(file, line)) {
      int idx = line.find(",");
      auto op = line.substr(0, idx);
      // match on types of operation, addx or noop
      // append corresponding operation to ops
      if (op == "addx") {
        auto val = stoi(line.substr(idx + 1));
        ops.push_back(Operation(true, val));
      } else if (op == "noop") {
        ops.push_back(Operation(false, 0));
      }
    }
  }
  return ops;
}

vector<State> generate_states(vector<Operation> &ops) {
  vector<State> states;
  int current = 1;
  // for each operation check which type it is and append the corresponding
  // state to states also keep depleting the time parameter until all time has
  // passed for that operation and CPU is free to move to next op
  for (auto op : ops) {
    auto state = op.addx ? State(current, op.val, 2) : State(current, 0, 1);
    while (state.time > 0) {
      auto copy = state;
      states.push_back(copy);
      state.time -= 1;
    }
    current += state.op;
  }
  return states;
}

int score(vector<State> &states) {
  // calculate score as per the puzzle description
  int i = 20;
  int score = 0;
  while (i < states.size() - 1) {
    score += i * states[i - 1].val;
    i += 40;
  }
  return score;
}

vector<vector<bool>> generate_crt(vector<State> &states, unsigned int width,
                                  unsigned int height) {
  // generate vector of vector of booleans that represent the pixels on the CRT
  // screen True if drawn, fals otherwise.
  vector<vector<bool>> screen{height, vector<bool>(width, false)};
  for (int r = 0; r < height; r++) {
    for (int c = 0; c < width; c++) {
      auto state = states[r * width + c];
      if ((state.val - 1) <= c && c <= (state.val + 1)) {
        screen[r][c] = true;
      }
    }
  }
  return screen;
}

void print_screen(vector<vector<bool>> &screen) {
  // print the screen to stdout
  for (int r = 0; r < screen.size(); r++) {
    for (int c = 0; c < screen[0].size(); c++) {
      if (screen[r][c]) {
        cout << "#";
      } else {
        cout << ".";
      }
    }
    cout << endl;
  }
}

int main(int argc, char *argv[]) {
  if (argc > 1) {
    // get operations
    auto ops = parse_input(argv[1]);
    // get states from operations
    auto states = generate_states(ops);

    // score operations
    cout << "Score for part 1: " << score(states) << endl;
    cout << endl;
    cout << "Part 2: " << endl;

    // check if pixel is drawn on CRT screen and print
    auto crt = generate_crt(states, 40, 6);
    print_screen(crt);
  }
}
