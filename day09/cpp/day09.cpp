#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <array>

using namespace std;

struct Move {
    int x;
    int y;
};

struct Grid {
    vector<vector<bool>> grid;
    array<int, 2> pos;
    vector<vector<bool>> visited;
}

vector<Move> input_parse(String &path){
    vector<Move> lines;
    std::ifstream file(path);
    if (file.is_open()){
        while (getline(file, line)){
            int idx = line.find(",")
            string dirstr = line.substr(0, idx);
            int nr = stoi(line.substr(idx+1));
            array<int,2> dir{0,0};
            if (dirstr == "R"){dir[0]=1; dir[1]=0;}
            else if (dirstr == "L") { dir[0] = -1; dir[1] = 0;}
            else if (dirstr == "U") { dir[0] = 0; dir[1]=-1;}
            else if (dirstr == "D") { dir[0] = 0; dir[1]=1;}
        }
        for (int i = 0; i<nr; i++){
            lines.push(Move(dir[0],dir[1]));
        }
    }
    return lines;
}

int main(int argc, char *argv[]){
    if (argc <= 1 ){
        return -1;
    } else {
        parse_input(argv[1])
    }
}