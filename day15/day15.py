import re
import time

import numpy as np
import pandas as pd
from scipy.spatial.distance import cdist

tmp = []
with open("./data/day15.txt") as f:
    for line in f.readlines():
        coords = re.search(
            "=(-?\d+).*=(-?\d+).*=(-?\d+).*=(-?\d+)", line).groups()
        tmp.append(list(map(lambda x: int(x), coords)))

df = pd.DataFrame(tmp)
df.columns = ["sx", "sy", "bx", "by"]

# offset coords
offset = 0
df = df + offset


def pair_dist(sx, sy, bx, by):
    return cdist(np.array([[sx, sy]]), np.array([[bx, by]]), "cityblock")[0][0]


start_time = time.time()
df["dist"] = df.apply(lambda x: pair_dist(*x), axis=1)

# target_y = 2000000
# target_x = 10000000
# target_arr = np.zeros(shape=(2 * target_x, 2), dtype=float)
# target_arr[:, 0] = np.arange(-target_x, target_x)
# target_arr[:, 1] = target_y + offset
#
# dist = cdist(df[["sx", "sy"]].values, target_arr, "cityblock")
# covered = dist <= df["dist"].values.reshape(df.shape[0], -1)
# merged = np.logical_or.reduce(covered, axis=0)
#
# print(np.sum(covered, axis=1))
# print(np.sum(merged) - 1)
# print(f"Time elapsed: {time.time()-start_time}")

'''
Part 2:
    There is only one point in the 4e6 x 4e6 square that's outside of the range of all sensor to beacon squares.
    Hence this has to be on a line just outside of these squares.
    For each square find 4 lines just outside of it and intersect them.
    The single point that is outside of all squares has to be the point where all lines intersect.
'''


def construct_lines(sx, sy, dist, limit=int(4e6)):

    left_cross = (sx - dist - 1, sy)
    right_cross = (sx + dist + 1, sy)
    up_cross = (sx, sy - dist - 1)
    down_cross = (sx, sy + dist + 1)

    top_left = np.arange()
    top_right =
