import numpy as np
import pandas as pd

elves = [[]]
with open("./data/day1.txt") as f:
    c = 0
    for line in f.readlines():
        strip = line.rstrip("\n")
        if strip == "":
            c += 1
            elves.append([])
        else:
            elves[c].append(int(strip))

summed = [sum(elf) for elf in elves]
srtd = sorted(summed, reverse=True)
print(srtd)
print(sum(srtd[:3]))
