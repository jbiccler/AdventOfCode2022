#include <array>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

std::array<string, 2> split(std::string &str, std::string delimiter) {
  array<string, 2> v;
  // Find the index of the delimiter.
  int delimiter_index = str.find(delimiter);
  // If the delimiter was found, then split the string into two tokens.
  if (delimiter_index != -1) {
    v[0] = str.substr(0, delimiter_index);
    v[1] = str.substr(delimiter_index + delimiter.size());
  } else {
    // If the delimiter was not found, then the entire string is a single token.
    v[0] = str;
    v[1] = str;
  }
  return v;
}

array<int, 2> to_num(std::string &str) {
  array<string, 2> v = split(str, "-");
  array<int, 2> vint = {stoi(v[0]), stoi(v[1])};
  return vint;
}

bool full_overlap(array<int, 2> &left, array<int, 2> &right) {
  if ((left[0] >= right[0]) && (left[1] <= right[1])) {
    return true;
  } else if ((right[0] >= left[0]) && (right[1] <= left[1])) {
    return true;
  } else {
    return false;
  }
}

bool partial_overlap(array<int, 2> &left, array<int, 2> &right) {
  if ((left[0] <= right[1]) && (left[1] >= right[0])) {
    return true;
  } else if ((right[0] <= left[1]) && (right[1] >= left[0])) {
    return true;
  } else {
    return false;
  }
}

void parse_input(std::string path) {
  string line;
  ifstream inputFile(path);
  int sum_part1{0};
  int sum_part2{0};

  if (inputFile.is_open()) {
    while (getline(inputFile, line)) {
      array<string, 2> spl = split(line, ",");
      array<int, 2> left = to_num(spl[0]);
      array<int, 2> right = to_num(spl[1]);
      sum_part1 += full_overlap(left, right);
      sum_part2 += partial_overlap(left, right);
    }
  }
  cout << "Total for part 1: " << sum_part1 << endl;
  cout << "Total for part 2: " << sum_part2 << endl;
}

int main(int argc, char *argv[]) {
  if (argc <= 1) {
    return 1;
  } else {
    parse_input(argv[1]);
  }
  return 0;
}
