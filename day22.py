import re

import numpy as np


def parse_data(path):
    step_bool = False
    mp = []
    max_length = 0
    steps = []
    with open(path) as f:
        for line in f.readlines():
            if not step_bool:
                if line == "\n":
                    step_bool = True
                    continue
                mp.append([*line][:-1])
                if len(mp[-1]) > max_length:
                    max_length = len(mp[-1])
            else:
                tmp = line
                while len(tmp) > 0:
                    digit = re.match("(\d+)", tmp)
                    word = re.match("([A-Z]+)", tmp)
                    if digit:
                        dig = digit.groups()[0]
                        steps.append(int(dig))
                        tmp = tmp[len(dig):]
                    elif word:
                        w = word.groups()[0]
                        steps.append(w)
                        tmp = tmp[len(w):]
                    else:
                        tmp = tmp[1:]
    for i in range(len(mp)):
        mp[i] = mp[i] + (max_length - len(mp[i])) * [" "]
    return np.array(mp), steps


def map_to_int(mp):
    res = np.zeros_like(mp)
    res[mp == " "] = 0
    res[mp == "."] = 1
    res[mp == "#"] = 2
    return res.astype(int)


def next_move(x, y, mp, steps, dir):
    dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # clockwise
    next_step = steps[0]
    indx = dirs.index(dir)
    if next_step == "R":
        return x, y, mp, steps[1:], dirs[(indx + 1) % len(dirs)]
    elif next_step == "L":
        return x, y, mp, steps[1:], dirs[(indx - 1)]
    counter = 0
    xdim, ydim = mp.shape

    def increment_step(x, y):
        x1, y1 = (x + dir[0]) % xdim, (y + dir[1]) % ydim
        return x1, y1

    x1, y1 = x, y
    while counter < next_step:
        counter += 1
        x1, y1 = increment_step(x1, y1)
        # print(x1, y1, mp[x1, y1])
        # continue to step until we reach a 1 or 2
        while mp[x1, y1] == 0:
            x1, y1 = increment_step(x1, y1)
        if mp[x1, y1] == 1:
            x, y = x1, y1  # free to move
            # print(f"Updated x,y to {x,y}")
        elif mp[x1, y1] == 2:
            break  # hit a wall
    return x, y, mp, steps[1:], dir


if __name__ == "__main__":
    mp, steps = parse_data("./data/day22.txt")
    print(steps)
    mapped = map_to_int(mp)
    print(mapped)
    x, y = np.argwhere(mapped == 1)[0, :]
    dir = (0, 1)
    while len(steps) > 0:
        x, y, mp, steps, dir = next_move(x, y, mapped, steps, dir)
        # print(f"After inter {x,y}")
    print(f"final: {x, y,dir}")
    dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]  # clockwise
    print(1000 * (x + 1) + 4 * (y + 1) + dirs.index(dir))
