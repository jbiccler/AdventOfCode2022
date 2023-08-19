import numpy as np

trees = []
with open("./data/day8.txt") as f:
    for line in f.readlines():
        if line == "\n":
            break
        trees.append([l for l in [*line] if l != "\n"])


trees = np.array(trees, dtype=int)

print(trees)

max_val = np.max(trees)
nrows, ncols = trees.shape
counter = nrows * 2 + ncols * 2 - 4
visible = []
for i in range(max_val + 1):
    map = trees >= i
    rows, cols = np.where(trees == i)

    for r, c in list(zip(rows, cols)):
        # if edge then 1
        if r == 0 or r == nrows - 1 or c == 0 or c == ncols - 1:
            continue
        else:
            # not on edge
            sums = np.array(
                (
                    np.sum(map[:r, c]),
                    np.sum(map[r + 1 :, c]),
                    np.sum(map[r, :c]),
                    np.sum(map[r, c + 1 :]),
                )
            )
            if np.any(sums == 0):
                counter += 1
                visible.append((r, c))
# view which trees are supposedly visible for debugging
visible_trees = np.copy(trees)
for r, c in visible:
    visible_trees[r, c] = -99

print(counter)
# print(visible_trees)
# part 2
scenic = np.zeros_like(trees)
edges = np.zeros_like(trees)
edges[0, :] = True
edges[:, 0] = True
edges[-1, :] = True
edges[:, -1] = True
# print(edges)
for i in range(max_val + 1):
    map = (trees >= i) | edges
    rows, cols = np.where(trees == i)
    for r, c in list(zip(rows, cols)):
        num_trees = lambda x: np.argmax(x) + 1
        left, right, up, down = (
            (map[:r, c]),
            (map[r + 1 :, c]),
            (map[r, :c]),
            (map[r, c + 1 :]),
        )
        res = 1
        if len(left) > 0:
            res *= num_trees(left.reshape(-1)[::-1])
        if len(right) > 0:
            res *= num_trees(right.reshape(-1)[::1])
        if len(up) > 0:
            res *= num_trees(up.reshape(-1)[::-1])
        if len(down) > 0:
            res *= num_trees(down.reshape(-1)[::1])
        scenic[r, c] = res

print(np.max(scenic))
