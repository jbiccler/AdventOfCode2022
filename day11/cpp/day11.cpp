#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <regex>
#include <string>
#include <vector>

using namespace std;

typedef unsigned long int u64;
typedef unsigned int u32;

enum OperationType { Addition = 0, Multiplication = 1 };

struct Operation {
  OperationType op;
  u32 val;
  bool op_self;
  Operation(OperationType op, u32 val, bool op_self)
      : op(op), val(val), op_self(op_self){};
};

struct Monkey {
  vector<u64> items;
  Operation op;
  u32 div;
  u32 next_true;
  u32 next_false;
  u32 n_inspected;
  Monkey(vector<u64> items, Operation op, u32 div, u32 next_true,
         u32 next_false, u32 n_inspected)
      : items(items),
        op(op),
        div(div),
        next_true(next_true),
        next_false(next_false),
        n_inspected(n_inspected){};
};

vector<Monkey> parse_input(string path) {
  // get file
  std::ifstream file(path);
  string line;
  // output vector
  vector<Monkey> monkeys;
  // define a regex pattern per line of expected input
  std::regex starting_pattern("Starting items: (.*)");
  std::regex operation_pattern("Operation: new = old (\\*|\\+) (\\d+|old)");
  std::regex test_pattern("Test: divisible by (\\d+)");
  std::regex true_pattern("If true: .*(\\d+)");
  std::regex false_pattern("If false: .*(\\d+)");
  std::smatch matches;
  // placeholder variables for coming loop
  vector<u64> items;
  Operation op = Operation(OperationType::Addition, 0, false);
  u32 div{1};
  u32 true_monkey{0};
  u32 false_monkey{0};

  if (file.is_open()) {
    // iterate over each line and parse to monkey
    while (getline(file, line)) {
      if (regex_search(line, matches, starting_pattern)) {
        for (int i = 1; i < matches.size(); i++) {
          std::string remainder{matches[1]};
          auto idx = remainder.find(", ");
          while (idx != string::npos) {
            items.push_back(stoi(remainder.substr(0, idx)));
            remainder = remainder.substr(idx + 2);
            idx = remainder.find(", ");
          }
          if (remainder.size() > 0) {
            items.push_back(stoi(remainder));
          }
        }
      } else if (regex_search(line, matches, operation_pattern)) {
        string oper = matches[1];
        OperationType oper_type;
        if (oper == "*") {
          oper_type = OperationType::Multiplication;
        } else if (oper == "+") {
          oper_type = OperationType::Addition;
        } else {
          cout << "Failed to parse operator: " << oper << endl;
        }
        string target = matches[2];
        if (target == "old") {
          op = Operation(oper_type, 0, true);
        } else {
          op = Operation(oper_type, stoi(target), false);
        }
      } else if (regex_search(line, matches, test_pattern)) {
        div = stoi(matches[1]);
      } else if (regex_search(line, matches, true_pattern)) {
        true_monkey = stoi(matches[1]);
      } else if (regex_search(line, matches, false_pattern)) {
        false_monkey = stoi(matches[1]);
        // assume this is the last line of input per monkey
        monkeys.push_back(Monkey(items, op, div, true_monkey, false_monkey, 0));
        items.clear();
      }
    };
  }
  return monkeys;
}

void execute_round(vector<Monkey> &monkeys, bool whole_divide_3, u64 lcm) {
  for (int m = 0; m < monkeys.size(); m++) {
    // while there are still items for this monkey
    // process them, pass them along and clear the item from the list
    while (monkeys[m].items.size() > 0) {
      // remove this item from the current monkey's queue
      auto item = monkeys[m].items.front();
      monkeys[m].items.erase(monkeys[m].items.begin());
      // do worry level operation
      if (monkeys[m].op.op_self) {
        if (monkeys[m].op.op == OperationType::Addition) {
          item += item;
        } else if (monkeys[m].op.op == OperationType::Multiplication) {
          item *= item;
        }
      } else {
        if (monkeys[m].op.op == OperationType::Addition) {
          item += monkeys[m].op.val;
        } else if (monkeys[m].op.op == OperationType::Multiplication) {
          item *= monkeys[m].op.val;
        }
      }
      if (whole_divide_3) {
        // monkey gets bored -> whole division by 3
        item /= 3;
      } else if (item > lcm) {
        // avoid integer overflow by modula LCM operation
        item = item % lcm;
      }
      // test divisibility
      if (item % monkeys[m].div == 0) {
        // pass to monkey in true case
        monkeys[monkeys[m].next_true].items.push_back(item);
      } else {
        // pass to monkey in false case
        monkeys[monkeys[m].next_false].items.push_back(item);
      }
      monkeys[m].n_inspected += 1;
    }
  }
}

u64 score(vector<Monkey> &monkeys) {
  // score is the multiplication of the number of inspections of the two monkeys
  // with the most inpsections.
  if (monkeys.size() < 2) {
    cout << "insufficient monkeys to score" << endl;
    return 0;
  }
  vector<u64> res;
  for (auto m : monkeys) {
    res.push_back(m.n_inspected);
  }
  // sort and multiply the two most frequent ones
  std::sort(res.begin(), res.end(), std::greater<u64>());
  return res[0] * res[1];
}

u64 lcm(u64 a, u64 b) {
  // find lowest common multiple based on stdlib greatest common divisor.
  return (a * b) / std::__gcd(a, b);
}

int main(int argc, char *argv[]) {
  if (argc > 1) {
    auto monkeys = parse_input(argv[1]);
    // copy for part 2 as to avoid parsing again
    auto monkeys2{monkeys};
    cout << "Part 1, 20 round" << endl;
    for (int i = 0; i < 20; i++) {
      execute_round(monkeys, true, 0);
    }
    cout << "Monkey business score of " << score(monkeys) << endl;
    cout << "Part 2, 10000 round" << endl;
    auto lcm_val = monkeys2[0].div;
    for (int m = 1; m < monkeys2.size(); m++) {
      lcm_val = lcm(lcm_val, monkeys2[m].div);
    }
    for (int i = 0; i < 10000; i++) {
      execute_round(monkeys2, false, lcm_val);
    }
    cout << "Monkey business score of " << score(monkeys2) << endl;
  }
}