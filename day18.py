import time

import numpy as np

coords = []

with open("./data/day18.csv") as f:
    for line in f.readlines():
        split = line[:-1].split(",")
        coords.append(split)
coords = np.array(coords).astype(int) + 1

start_time = time.time()
grid = np.zeros(shape=(30, 30, 30))
for row in coords:
    grid[row[0], row[1], row[2]] = 1

non_shared = 0

for coord in coords:
    x, y, z = coord
    horz = grid[(x - 1): (x + 2), y, z]
    vert = grid[x, (y - 1): (y + 2), z]
    depth = grid[x, y, (z - 1): (z + 2)]
    shared = np.sum(horz) + np.sum(vert) + np.sum(depth) - \
        3  # counts itself 3 times
    non_shared += 6 - int(shared)

print(non_shared)

# part 2

for direction in (+1, -1):
    # go up and down through the gird
    xdim, ydim, zdim = grid.shape
    if direction == 1:
        start, end = 0, zdim
    else:
        start, end = zdim - 1, -1
    for z in range(start, end, direction):
        row = grid[:, :, z]
        if (direction == 1 and z == 0) or (direction == -1 and z == zdim - 1):
            row[row == 0] = 2
        else:
            # chose row above or below current one
            other_row = grid[:, :, z - direction]
            row[(row == 0) * (other_row == 2)] = 2
            changed = 1
            while changed:
                for indx in np.argwhere(row == 0):
                    x, y = indx
                    horz = row[(x - 1): (x + 2), y]
                    vert = row[x, (y - 1): (y + 2)]
                    if np.any(horz == 2) or np.any(vert == 2):
                        row[x, y] = 2
                        break
                else:
                    changed = 0


reachable = 0
for coord in coords:
    x, y, z = coord
    horz = grid[(x - 1): (x + 2), y, z]
    vert = grid[x, (y - 1): (y + 2), z]
    depth = grid[x, y, (z - 1): (z + 2)]
    reachable += np.sum(horz == 2) + np.sum(vert == 2) + np.sum(depth == 2)

print(reachable)
print(f"Elapsed time {time.time()-start_time}")
