import numpy as np
import pandas as pd

file = pd.read_csv("./data/day9.csv", header=None)
file.columns = ["direction", "moves"]


visited = np.zeros((10000, 10000))


def is_adjacent(hx, hy, tx, ty):
    return (np.abs(hx - tx) <= 1) * (np.abs(hy - ty) <= 1)


def next_move(hx, hy, tx, ty):
    adj = is_adjacent(hx, hy, tx, ty)
    if adj:
        return (tx, ty)
    return (tx + np.sign(hx - tx), ty + np.sign(hy - ty))


def next_moves(visited, x, y, dir, nmoves):
    map = {"D": (-1, 0), "U": (1, 0), "L": (0, -1), "R": (0, 1)}
    dir_t = map[dir]
    for m in range(nmoves):
        x[0] = x[0] + dir_t[0]
        y[0] = y[0] + dir_t[1]
        for k in range(1, len(x)):
            x[k], y[k] = next_move(x[k - 1], y[k - 1], x[k], y[k])
        visited[x[-1], y[-1]] += 1
    return visited, x, y


n = 10
x, y = [0] * n, [0] * n
visited[x[-1], y[-1]] = 1

for i, r in file.iterrows():
    visited, x, y = next_moves(visited, x, y, r["direction"], r["moves"])

print(np.sum(visited > 0))
