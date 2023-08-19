#include <algorithm>
#include <fstream>
#include <functional>
#include <iostream>
#include <string>
#include <vector>
using namespace std;

void parse_input(std::string path) {
  string line;
  int sum{0};
  unsigned int c{0};

  vector<int> elves;
  ifstream inputFile(path);
  if (inputFile.is_open()) {
    while (getline(inputFile, line)) {
      if (line == "\n" or line == "") {
        sum = 0;
        c++;
      } else {
        sum += stoi(line);
        if (c < elves.size()) {
          elves[c] = sum;
        } else {
          elves.push_back(sum);
        }
      }
    }
    inputFile.close();
    sort(elves.begin(), elves.end(), greater<int>());
    sum = 0;
    for (int i = 0; i < 3; i++) {
      cout << "Sorted elf " << i << " carries " << elves[i] << endl;
      sum += elves[i];
    }
    cout << "Total of first 3 elves " << sum << endl;
  } else
    cout << "Unable to open file";
}

int main() {
  parse_input("./day1.txt");
  return 0;
}
