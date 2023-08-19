import string

import numpy as np
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import shortest_path

with open("./data/day12.txt") as f:
    file = f.readlines()

alpha_map = {
    **{s: i for i, s in enumerate(string.ascii_lowercase)},
    **{"S": 0, "E": 25},
}
alpha = []
arr = []
for line in file:
    if line == "\n":
        break
    letters = [*line][:-1]
    alpha.append(letters)
    num = []
    for letter in letters:
        num.append(alpha_map[letter])
    arr.append(num)
alpha = np.array(alpha)
arr = np.array(arr)

nrows, ncols = arr.shape
start = np.where(alpha == "S")
start = (start[0][0], start[1][0])
end = np.where(alpha == "E")
end = (end[0][0], end[1][0])
print(start, end)
# construct graph
G = np.zeros(shape=(nrows * ncols, nrows * ncols), dtype=int)


def update_graph(G, arr, fr, to):
    nrows, ncols = arr.shape
    diff = arr[to] - arr[fr]
    if diff <= 1:
        G_fr, G_to = ncols * fr[0] + fr[1], ncols * to[0] + to[1]
        G[G_fr, G_to] = 1  # distance of 1
    return G


for r in range(nrows):
    for c in range(ncols):
        if c + 1 < ncols:
            G = update_graph(G, arr, (r, c), (r, c + 1))
        if c - 1 >= 0:
            G = update_graph(G, arr, (r, c), (r, c - 1))
        if r + 1 < nrows:
            G = update_graph(G, arr, (r, c), (r + 1, c))
        if r - 1 >= 0:
            G = update_graph(G, arr, (r, c), (r - 1, c))

# Use Dijksta...
print(csr_matrix(G))
G_start, G_end = ncols * start[0] + start[1], ncols * end[0] + end[1]
print(G_start, G_end)
sp = shortest_path(csr_matrix(G), method="D", indices=G_start)
# print(sp[G_end], sp)
print(sp[G_end])

# part 2
# find all indices with a as elevation
a_indxs = np.where(arr == 0)
# convert to nodes
G_a_indxs = ncols * a_indxs[0] + a_indxs[1]
sp2 = shortest_path(csr_matrix(G), method="D", indices=G_a_indxs)
# print(sp[G_end], sp)
print(np.min(sp2[:, G_end]))
