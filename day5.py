import pandas as pd
import numpy as np
import copy
import re

# find dimensions of crates
with open("./data/day5.txt") as f:
    for i, line in enumerate(f.readlines()):
        if line[:2] == " 1":
            nrow = i
            ncol = int(line[-4:].strip())

crates = np.zeros(shape=(nrow, ncol), dtype=str)
moves = pd.DataFrame(
    {
        "move": np.array([], dtype=str),
        "from": np.array([], dtype=str),
        "to": np.array([], dtype=str),
    }
)
with open("./data/day5.txt") as f:
    for i, line in enumerate(f.readlines()):
        if i < nrow:
            for j, x in enumerate(range(0, len(line), 4)):
                crate = line[x : x + 4]
                crates[i][j] = crate.strip().lstrip("[").rstrip("]")
        elif i > nrow + 1:
            res = re.match("move (\d+) from (\d+) to (\d+)", line)
            n, fr, to = res.groups()
            moves = pd.concat(
                [
                    moves,
                    pd.DataFrame(
                        {
                            "move": [n],
                            "from": [fr],
                            "to": [to],
                        }
                    ),
                ],
                axis=0,
                ignore_index=True,
            )
moves = moves.astype(int)
# turn to list so we can have varying lengths
crates_l = [[]] * ncol

for i in range(crates.shape[1]):
    crates_l[i] = [c for c in crates[::-1, i] if c != ""]


def make_moves(data, n, fr, to, keep_order=False):
    """
    keep_order = True for second part, False for first.
    """
    tmp = []
    for i in range(n):
        tmp.append(data[fr - 1].pop(-1))
    if keep_order:
        tmp = tmp[::-1]
    data[to - 1].extend(tmp)
    return data


crates_l1 = copy.deepcopy(crates_l)
crates_l2 = copy.deepcopy(crates_l)

for i, x in moves.iterrows():
    crates_l1 = make_moves(crates_l1, x["move"], x["from"], x["to"], keep_order=False)
    crates_l2 = make_moves(crates_l2, x["move"], x["from"], x["to"], keep_order=True)

print("".join([row[-1] for row in crates_l1]), "".join([row[-1] for row in crates_l2]))
