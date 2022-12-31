
import numpy as np


class Map:
    """docstring for mp."""

    def __init__(self, nrow, ncol):
        self.nrow = nrow
        self.ncol = ncol
        self.create_mp()

    def create_mp(self):
        """
        Create mp of nrow x ncol
        """
        mp = []
        for i in range(self.nrow):
            row = []
            for j in range(self.ncol):
                row.append(None)
            mp.append(row)
        self.mp = mp

    def update_mp(self, val, x, y):
        self.mp[y][x] = val

    def __repr__(self):
        self.mp_to_arr()
        return str(self.arr)

    def mp_to_arr(self):
        arr = np.ones(shape=(self.nrow, self.ncol), dtype=str)
        arr[:, :] = "."
        for i in range(self.nrow):
            for j in range(self.ncol):
                el = self.mp[i][j]
                if el is not None:
                    if isinstance(el, Rock):
                        arr[i, j] = "#"
                    elif isinstance(el, Sand):
                        arr[i, j] = "o"
        self.arr = arr
        return arr


class Coord:
    def __init__(self, x, y):
        self.x = x
        self.y = y


class Rock(Coord):
    def __init__(self, x, y):
        super(Rock, self).__init__(x, y)


class Sand(Coord):
    def __init__(self, x, y, rest, mp, fresh_spawn):
        super(Sand, self).__init__(x, y)
        self.rest = rest
        self.mp = mp
        self.fresh_spawn = fresh_spawn

    def next_move(self):
        if hasattr(self.mp, "max_y"):
            if self.y == self.mp.max_y:
                return -1
        if self.rest:
            return
        if self.fresh_spawn:
            # check if spawn empty:
            spawn = self.mp.mp[self.y][self.x]
            if spawn is None:
                self.mp.update_mp(self, self.x, self.y)
            else:
                return -2
            self.fresh_spawn = False
        x, y = self.x, self.y
        down = self.mp.mp[y + 1][x]

        next_coords = None
        if down is None:
            next_coords = (x, y + 1)
        else:
            left = self.mp.mp[y + 1][x - 1]
            right = self.mp.mp[y + 1][x + 1]
            if left is None:
                next_coords = (x - 1, y + 1)
            elif right is None:
                next_coords = (x + 1, y + 1)
            else:
                # come to rest
                self.rest = True
                return
        # update map and coordinates
        self.mp.update_mp(None, x, y)
        self.x, self.y = next_coords
        self.mp.update_mp(self, self.x, self.y)
        return

        self.mp.update_mp(None, x, y)
        self.mp.update_mp(self, x, y + 1)


def rock_parser(line, mp, x_coord_offset=0, y_coord_offset=0):
    spl = [s.strip() for s in line.split("->")]
    coords = []
    for coord in spl:
        x, y = coord.split(",")
        x, y = int(x) - x_coord_offset, int(y) - y_coord_offset
        coords.append((x, y))
    all_coords = []
    for i, coord in enumerate(coords):
        if i == 0:
            start = coords[i]
            continue
        else:
            prev = coords[i - 1]
            x_diff = coord[0] - prev[0]
            y_diff = coord[1] - prev[1]
            sign_x = np.sign(x_diff)
            sign_y = np.sign(y_diff)
            if sign_x == 0 and sign_y == 0:
                all_coords.append(coord)
                continue
            elif sign_x == 0:
                x_coords = [coord[0]] * (np.abs(y_diff) + 1)
                y_coords = list(range(prev[1], coord[1] + sign_y, sign_y))
            elif sign_y == 0:
                y_coords = [coord[1]] * (np.abs(x_diff) + 1)
                x_coords = list(range(prev[0], coord[0] + sign_x, sign_x))
            if len(all_coords) > 0:
                all_coords.extend(list(zip(x_coords, y_coords))[1:])
            else:
                all_coords.extend(list(zip(x_coords, y_coords)))
    for coord in all_coords:
        mp.update_mp(Rock(coord[0], coord[1]), coord[0], coord[1])
    return mp


# part 1
mp = Map(200, 200)
x_coord_offset = 400
y_coord_offset = 0


with open("./data/day14.txt") as f:
    for line in f.readlines():
        mp = rock_parser(
            line, mp, x_coord_offset=x_coord_offset, y_coord_offset=y_coord_offset
        )

max_y_before_abyss = np.max(np.argwhere(mp.mp_to_arr() == "#")[:, 0])
mp.max_y = max_y_before_abyss

spawn_point = 500 - x_coord_offset


def simulate_sand(n, spawn_point, mp):
    for i in range(n):
        s = Sand(spawn_point, y=0, rest=False, mp=mp, fresh_spawn=True)
        while not s.rest:
            res = s.next_move()
            if res == -1:
                print(f"First unit to fall endlessly, i: {i}")
                return
            elif res == -2:
                print(f"Spawn point full at i: {i}")
                return


simulate_sand(1000, spawn_point, mp)

# part 2
mp = Map(302, 1000)
x_coord_offset = 0
y_coord_offset = 0

with open("./data/day14.txt") as f:
    for line in f.readlines():
        mp = rock_parser(
            line, mp, x_coord_offset=x_coord_offset, y_coord_offset=y_coord_offset
        )

max_y = np.max(np.argwhere(mp.mp_to_arr() == "#")[:, 0]) + 2

mp = rock_parser(
    f"100,{max_y} -> 900,{max_y}",
    mp,
    x_coord_offset,
    y_coord_offset,
)
mp.max_y = max_y

spawn_point = 500 - x_coord_offset

simulate_sand(100000, spawn_point, mp)
