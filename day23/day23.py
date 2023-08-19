import numpy as np


def parse_input(path):
    data = []
    with open(path) as f:
        for line in f.readlines():
            data.append(list(map(lambda x: {".": 0, "#": 1}[x], line[:-1])))
    return np.array(data)


def do_round(mp, dirs):
    indxs = np.argwhere(mp == 1)
    next_moves = []
    for elf in indxs:
        x, y = elf
        neighbouring = mp[(x - 1): (x + 2), (y - 1): (y + 2)]
        if np.sum(neighbouring) == 1:
            # dont move
            next_moves.append(((x, y), (x, y)))
            continue
        else:
            for direction in dirs:
                xrange = (
                    range((x - 1), (x + 2)
                          ) if direction[0] == 0 else x + direction[0]
                )
                yrange = (
                    range((y - 1), (y + 2)
                          ) if direction[1] == 0 else y + direction[1]
                )
                potential_pos = mp[xrange, yrange]
                if np.sum(potential_pos) == 0:
                    next_moves.append(
                        ((x, y), (x + direction[0], y + direction[1]))
                    )  # from to
                    break
    froms = [m[0] for m in next_moves]
    tos = [m[1] for m in next_moves]
    atleast_one_moved = False
    for fr, to in next_moves:
        if to != fr:
            if tos.count(to) > 1:
                continue
            else:
                # make move
                mp[fr[0], fr[1]] = 0
                mp[to[0], to[1]] = 1
                atleast_one_moved = True
    return mp, dirs[1:] + [dirs[0]], atleast_one_moved


def smallest_rectange(mp):
    ones = np.argwhere(mp == 1)
    minx, maxx = np.min(ones[:, 0]), np.max(ones[:, 0])
    miny, maxy = np.min(ones[:, 1]), np.max(ones[:, 1])
    rect = mp[minx: (maxx + 1), miny: (maxy + 1)]
    return rect, np.sum(rect == 0)


if __name__ == "__main__":
    mp = np.pad(parse_input("./data/day23.txt"), ((100, 100), (100, 100)))
    rect, _ = smallest_rectange(mp)
    dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    round_counter = 0
    atleast_one_moved = True
    # for r in range(10): # for part 1
    while atleast_one_moved:
        round_counter += 1
        print(f"Round: {round_counter}")
        mp, dirs, atleast_one_moved = do_round(mp, dirs)
    rect, summed = smallest_rectange(mp)
    print(summed, round_counter)
