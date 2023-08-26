#include <array>
#include <fstream>
#include <iostream>
#include <string>
#include <string_view>
#include <vector>

using namespace std;

string parse_input(string path) {
  std::ifstream file(path);
  std::string line;
  getline(file, line);
  return line;
}

bool has_unique_characters(std::string_view input) {
  // Assuming ASCII encoding
  if (input.size() > 128) {
    return false;
  }
  array<bool, 128> seen_chars{};
  for (char ch : input) {
    if (seen_chars[ch]) {
      return false; // Character already seen, not unique
    }
    seen_chars[ch] = true;
  }
  return true;
}

int find_mark(string &line, int len) {
  for (int i = 0; i < line.size() - len; i++) {
    if (has_unique_characters(string_view(line.data() + i, len))) {
      return i + len;
    }
  }
  return -1;
}

int main(int argc, char *argv[]) {
  if (argc == 1) {
    return -1;
  } else {
    string line = parse_input(argv[1]);
    int mark1 = find_mark(line, 4);
    cout << "Mark of length 4 found at: " << mark1 << endl;
    int mark2 = find_mark(line, 14);
    cout << "Mark of length 14 found at: " << mark2 << endl;
  }
  return 0;
}
