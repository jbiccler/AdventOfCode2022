#include <array>
#include <fstream>
#include <iostream>
#include <optional>
#include <string>
#include <vector>

using namespace std;

struct State
{
    int val;
    int op;
    unsigned int time;
    State(int val, int op, unsigned int time) : val(val), op(op), time(time){};
};

struct Operation
{
    bool addx;
    int val;
    Operation(bool addx, int val) : addx(addx), val(val){};
};

vector<Operation> parse_input(string path)
{
    std::ifstream file(path);
    std::string line;
    // init vector with capacity 100
    vector<Operation> ops;
    if (file.is_open())
    {
        while (getline(file, line))
        {
            int idx = line.find(",");
            auto op = line.substr(0, idx);
            auto valstr = line.substr(idx + 1);
            int val = 0;
            if (valstr.size() > 0)
            {
                val = stoi(valstr);
            }
            if (op == "addx")
            {
                ops.push_back(Operation(true, val));
            }
            else if (op == "noop")
            {
                ops.push_back(Operation(false, val));
            }
        }
    }
    return ops;
}

vector<State> generate_states(vector<Operation> &ops)
{
    vector<State> states;
}

int main(int argc, char *argv[])
{
    if (argc > 1)
    {
        auto ops = parse_input(argv[1]);
        // auto states = generate_states(ops);

        // cout << "Score for part 1: " << score(states) << endl;
        // cout << endl;
        // cout << "Part 2: " << endl;

        // auto crt = generate_crt(states, 40, 6);
        // print_screen(crt);
    }
}