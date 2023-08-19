import pandas as pd

file = pd.read_csv("./data/day2.csv", header=None)
file.columns = ["first", "second"]

# part 1
def calc_result(f, s):
    value_map = {"A": 1, "B": 2, "C": 3, "X": 1, "Y": 2, "Z": 3}

    if value_map[f] == value_map[s]:
        return value_map[s] + 3
    elif value_map[s] - value_map[f] == 1 or value_map[s] - value_map[f] == -2:
        return value_map[s] + 6
    else:
        return value_map[s]


file["res"] = file.apply(lambda x: calc_result(x[0], x[1]), axis=1)
print(file.res.sum())

# part 2
def calc_result2(f, s):
    value_map = {"A": 1, "B": 2, "C": 3, "X": 0, "Y": 3, "Z": 6}
    l = ["A", "B", "C"]
    if s == "X":
        return value_map[l[l.index(f) - 1]] + value_map[s]
    elif s == "Y":
        return value_map[f] + value_map[s]
    else:
        return value_map[l[(l.index(f) + 1) % 3]] + value_map[s]


file["res2"] = file.apply(lambda x: calc_result2(x[0], x[1]), axis=1)
print(file.res2.sum())
