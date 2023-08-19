import re
import time

import numpy as np

blues = []


class Blueprint:
    def __init__(self, indx, robots):
        self.indx = indx
        self.robots = robots


class Robot:
    def __init__(self, mineral, cost, name):
        self.mineral = mineral
        self.cost = cost
        self.name = name


def search(blues, nr_minutes=24, to_keep=1000, prune_from=10000):
    res = {i: {} for i in range(1, len(blues) + 1)}

    for key in res.keys():
        # start
        blue = [blue for blue in blues if blue.indx == key][0]
        nr_robots = np.array([1, 0, 0, 0])
        nr_minerals = np.array([0, 0, 0, 0])
        res[key][1] = {
            "O": {"nr_robots": nr_robots, "nr_minerals": nr_minerals + nr_robots}
        }
        for m in range(1, nr_minutes):
            current_paths = res[key][m]
            print(f"key:{key}, minute:{m}, len: {len(current_paths.values())}")
            # possible builds
            next_paths = {}
            for path, values in current_paths.items():
                nr_minerals = values["nr_minerals"]
                nr_robots = values["nr_robots"]
                for robot in blue.robots:
                    if np.all(nr_minerals >= robot.cost):
                        new_robots = nr_robots + robot.mineral
                        new_minerals = nr_minerals - robot.cost + nr_robots
                        next_paths[path + robot.name] = {
                            "nr_robots": new_robots,
                            "nr_minerals": new_minerals,
                        }
            # prune
            if len(current_paths.values()) >= prune_from:
                srtd = sorted(
                    list(next_paths.items()),
                    key=lambda x: (
                        x[-1]["nr_minerals"][-1],
                        x[-1]["nr_robots"][-1],
                        x[-1]["nr_minerals"][-2],
                        x[-1]["nr_robots"][-2],
                    ),
                )[-to_keep:]
                srtd = set([k for k, v in srtd])
                # print(f"current best: {srtd[-1]}")
                new_next_paths = {}
                for it, vals in next_paths.items():
                    if it in srtd:
                        new_next_paths[it] = vals
            else:
                new_next_paths = next_paths
            res[key][m + 1] = new_next_paths

    return res


with open("./data/day19.txt") as f:
    for line in f.readlines():
        ind, o, c, oo, oc, gore, gob = map(
            lambda x: int(x), re.findall("(\d+)", line))
        # ore,clay,obsidian,geode
        robots = [
            Robot(np.array([1, 0, 0, 0]), np.array([o, 0, 0, 0]), "Or"),
            Robot(np.array([0, 1, 0, 0]), np.array([c, 0, 0, 0]), "C"),
            Robot(np.array([0, 0, 1, 0]), np.array([oo, oc, 0, 0]), "Ob"),
            Robot(np.array([0, 0, 0, 1]), np.array([gore, 0, gob, 0]), "G"),
            Robot(np.array([0, 0, 0, 0]), np.array([0, 0, 0, 0]), "X"),
        ]
        blues.append(Blueprint(ind, robots))
###
# part 1
###
start_time = time.time()
res = search(blues, nr_minutes=24, to_keep=5000, prune_from=100000)
print(f"Elapsed time {time.time() - start_time}")

aggr = {}
for key, val in res.items():
    aggr[key] = {}
    for m, paths in val.items():
        for path, nrs in paths.items():
            aggr[key][m] = max(aggr[key].get(m, 0), nrs["nr_minerals"][-1])
print(aggr)

quality_levels = []
total_ql = 0
for key, val in aggr.items():
    ql = val[24] * key
    quality_levels.append(ql)
    total_ql += ql
print(quality_levels)
print(total_ql)

###
# part 2
###
start_time = time.time()
res = search(blues[:3], nr_minutes=32, to_keep=5000, prune_from=100000)
print(f"Elapsed time {time.time() - start_time}")

aggr = {}
for key, val in res.items():
    aggr[key] = {}
    for m, paths in val.items():
        for path, nrs in paths.items():
            aggr[key][m] = max(aggr[key].get(m, 0), nrs["nr_minerals"][-1])
print(aggr)

geodes = [val[32] for val in aggr.values()]
print(geodes)
print(geodes[0] * geodes[1] * geodes[2])
