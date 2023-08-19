import pprint

import numpy as np
import pandas as pd

df = pd.read_csv("./data/day10.csv", header=None)
df.columns = ["act", "add"]
end = [1]
for i, x in df.iterrows():
    if x["act"] == "noop":
        end.append(end[-1])
    else:
        end.extend([end[-1], end[-1] + x["add"]])
end = np.array(end, dtype=int)

cycles = np.array(range(20, 221, 40))
print(np.sum(end[cycles - 1] * cycles))

# part 2


def sprite(during):
    return (during - 1, during, during + 1)


res = ""

for c in range(1, 240):
    # iterate over cycles (c)
    spr = sprite(end[c - 1])  # during
    res = res + "#" if (c - 1) % 40 in spr else res + "."
res_split = [res[i : i + 40] for i in range(0, len(res), 40)]
pprint.pprint(res_split)
