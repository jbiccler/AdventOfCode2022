#include <fstream>
#include <iostream>
#include <string>
using namespace std;

int score_part1(std::string &str) {
  char first{str[0]};
  char snd{str[2]};
  // 0 for loss, 3 for draw, 6 for win
  int winDrawLose{0};
  switch (first) {
  case 'A':
    winDrawLose = (snd == 'X') * 3 + (snd == 'Y') * 6;
    break;
  case 'B':
    winDrawLose = (snd == 'Y') * 3 + (snd == 'Z') * 6;
    break;
  case 'C':
    winDrawLose = (snd == 'Z') * 3 + (snd == 'X') * 6;
    break;
  default:
    break;
  };
  // add score based on choice to winDrawLose
  return winDrawLose + (snd == 'X') * 1 + (snd == 'Y') * 2 + (snd == 'Z') * 3;
}

int score_part2(std::string &str) {
  char first{str[0]};
  char snd{str[2]};
  // 0 for loss, 3 for draw, 6 for win
  int winDrawLose{(snd == 'X') * 0 + (snd == 'Y') * 3 + (snd == 'Z') * 6};
  switch (first) {
  case 'A':
    return winDrawLose + (snd == 'X') * 3 + (snd == 'Y') * 1 + (snd == 'Z') * 2;
  case 'B':
    return winDrawLose + (snd == 'X') * 1 + (snd == 'Y') * 2 + (snd == 'Z') * 3;
  case 'C':
    return winDrawLose + (snd == 'X') * 2 + (snd == 'Y') * 3 + (snd == 'Z') * 1;
  default:
    return 0;
  };
}

void parse_input(std::string path) {
  string line;
  int sum_part1{0};
  int sum_part2{0};
  unsigned int c{0};
  ifstream inputFile(path);
  // open file and call scoring function on each line
  if (inputFile.is_open()) {
    while (getline(inputFile, line)) {
      sum_part1 += score_part1(line);
      sum_part2 += score_part2(line);
    }
    inputFile.close();
    cout << "Part 1 total: " << sum_part1 << endl;
    cout << "Part 2 total: " << sum_part2 << endl;
  } else
    cout << "Unable to open file";
}

int main(int argc, char* argv[]) {
  if (argc<=1){
    return 1;
  }
  parse_input(argv[1]);
  return 0;
}