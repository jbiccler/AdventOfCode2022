#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <set>
#include <string>
#include <vector>
using namespace std;

set<char> find_common(vector<string> &v) {
  if (v.size() < 2) {
    return set<char>{};
  }
  vector<set<char>> sep;
  for (int i = 0; i < v.size(); i++) {
    set<char> s;
    for (int j = 0; j < v[i].size(); j++) {
      s.insert(v[i][j]);
    }
    sep.push_back(s);
  }
  set<char> result;
  vector<set<char>>::iterator iter = sep.begin();
  if (iter != sep.end()) {
    result = *iter;
    for (++iter; iter != sep.end(); ++iter) {
      set<char> temp;
      // intersect *iter and result, put into temp
      set_intersection(iter->begin(), iter->end(), result.begin(), result.end(),
                       std::inserter(temp, temp.end()));
      result = temp;
    }
  }
  return result;
}

vector<string> split_half(std::string &str) {
  auto n{str.size()};
  vector<string> res;
  res.push_back(str.substr(0, n / 2));
  res.push_back(str.substr(n / 2));
  return res;
}

int score(set<char> s) {
  set<char>::iterator itr;
  int sum{0};
  string alphabet{"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"};
  for (itr = s.begin(); itr != s.end(); itr++) {
    sum += alphabet.find(*itr) + 1;
  }
  return sum;
}

void parse_input_part1(std::string path) {
  string line;
  ifstream inputFile(path);
  int sum{0};
  if (inputFile.is_open()) {
    while (getline(inputFile, line)) {
      vector<string> halves = split_half(line);
      set<char> common_part1 = find_common(halves);
      sum += score(common_part1);
    }
  }
  cout << "Total sum is " << sum << endl;
}

void parse_input_part2(std::string path) {
  string line;
  int sum{0};
  unsigned int c{0};
  // vector of lines from our text file
  std::ifstream is(path);
  std::istream_iterator<string> start(is), end;
  std::vector<string> bp(start, end);
  // create temporary vectors of size 3 each
  vector<string> vec1;
  for (int i = 0; i < bp.size(); i++) {
    if (i % 3 == 0) {
      vec1.clear();
    }
    vec1.push_back(bp[i]);
    if (vec1.size() == 3) {
      set<char> common_part2 = find_common(vec1);
      sum += score(common_part2);
    }
  }
  cout << "Total sum is " << sum << endl;
}

int main(int argc, char *argv[]) {
  if (argc <= 1) {
    return 1;
  } else {
    parse_input_part1(argv[1]);
    parse_input_part2(argv[1]);
  }
  return 0;
}
