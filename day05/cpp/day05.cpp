#include <array>
#include <fstream>
#include <iostream>
#include <iterator>
#include <string>
#include <vector>
#include <regex>
#include <utility>

using namespace std;

// start_row should be first row with instructions
pair<vector<vector<char>>, vector<array<int, 3>>> parse_input(string path, int start_row, int num_cols)
{
    // vector of lines from our text file
    vector<string> lines;
    std::ifstream file(path);
    if (file.is_open())
    {
        std::string line;
        while (std::getline(file, line))
        {
            lines.push_back(line);
        }
        file.close();
    }
    else
    {
        std::cout << "Could not open file: " << path << std::endl;
    }

    vector<vector<char>> res;
    // add the elements in reverse order to the vector
    // such that the last element of the vector is the first to be popped off in a move
    for (int i = 0; i < num_cols; i++)
    {
        vector<char> tmp;
        for (int r = start_row - 3; r >= 0; r--)
        {
            if (lines[r][1 + i * 4] != ' ')
            {
                tmp.push_back(lines[r][(1 + i * 4)]);
            }
        }
        res.push_back(tmp);
    }

    std::regex pattern(".*move (\\d+) from (\\d+) to (\\d+).*");
    std::smatch matches;
    vector<array<int, 3>> moves;
    for (int r = start_row; r < lines.size(); r++)
    {
        if (std::regex_search(lines[r], matches, pattern))
        {
            if (matches.size() == 4)
            { // Make sure we have 4 elements including the full match
                array<int, 3> groups;

                for (size_t i = 1; i < matches.size(); ++i)
                {
                    groups[i - 1] = stoi(matches[i]);
                }

                // std::cout << "Extracted groups:" << std::endl;
                // for (int &group : groups)
                // {
                //     std::cout << group << std::endl;
                // }
                moves.push_back(groups);
            }
            else
            {
                std::cout << "Invalid number of groups found." << std::endl;
            }
        }
        else
        {
            std::cout << "No match found." << std::endl;
        }
    }
    return make_pair(res, moves);
}

vector<vector<char>> make_moves(vector<vector<char>> current, vector<array<int, 3>> &moves, bool move_all_at_once = false)
{
    for (int m = 0; m < moves.size(); m++)
    {
        int n = moves[m][0];
        int fr = moves[m][1] - 1;
        int to = moves[m][2] - 1;
        if (move_all_at_once)
        {
            vector<char> tmp;
            for (int j = 0; j < n; j++)
            {
                if (current[fr].size() > 0)
                {
                    // Pop the last element from the first vector
                    char last = current[fr].back();
                    current[fr].pop_back();
                    // Push the last element to the second vector
                    tmp.push_back(last);
                }
            }
            std::reverse(tmp.begin(), tmp.end());
            for (auto j : tmp)
            {
                current[to].push_back(j);
            }
        }
        else
        {

            for (int j = 0; j < n; j++)
            {
                if (current[fr].size() > 0)
                {
                    // Pop the last element from the first vector
                    char last = current[fr].back();
                    current[fr].pop_back();
                    // Push the last element to the second vector
                    current[to].push_back(last);
                }
            }
        }
    }
    return current;
}

void print_result(vector<vector<char>> &result)
{
    for (auto l : result)
    {
        for (auto c : l)
        {
            cout << c << " ";
        }
        cout << endl;
    }
}

void print_answer(vector<vector<char>> &result)
{
    for (auto l : result)
    {
        cout << l[l.size() - 1];
    }
    cout << endl;
}

int main(int argc, char *argv[])
{
    if (argc == 1)
    {
        return 1;
    }
    else
    {
        auto start = parse_input(argv[1], 10, 9);
        cout << "-------Start configuration---------" << endl;
        print_result(start.first);
        cout << "------------Part 1 configuration---------------" << endl;
        auto result_part1 = make_moves(start.first, start.second, false);
        print_result(result_part1);
        cout << "------------Part 1 answer---------------" << endl;
        print_answer(result_part1);
        cout << "------------Part 2 configuration---------------" << endl;
        auto result_part2 = make_moves(start.first, start.second, true);
        print_result(result_part2);
        cout << "------------Part 2 answer---------------" << endl;
        print_answer(result_part2);
    }
    return 0;
}