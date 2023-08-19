with open("./data/day6.txt") as f:
    string = f.readlines()[0]


def find_n_consecutive(n):
    for i, s in enumerate(string):
        sub = string[i : i + n]
        if len(set([*sub])) == n:
            return i


answer1 = find_n_consecutive(4)
answer2 = find_n_consecutive(14)

print(answer1 + 4, string[answer1 : answer1 + 4])
print(answer2 + 14, string[answer2 : answer2 + 14])
