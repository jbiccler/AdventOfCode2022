import copy

with open("./data/day13.txt") as f:
    file = f.readlines()

pairs = {}

for i, j in enumerate(range(0, len(file), 3)):
    pairs[i] = {"l": eval(file[j]), "r": eval(file[j + 1])}


def compare_lists(left, right):
    # reference: https://galaxyinferno.com/how-to-solve-advent-of-code-2022-day-13-with-python/
    while len(left) > 0 and len(right) > 0:
        l = left.pop(0)
        r = right.pop(0)

        if isinstance(l, list) and isinstance(r, list):
            recursive = compare_lists(l, r)
            if recursive is not None:
                return recursive
        elif isinstance(l, int) and isinstance(r, int):
            if l < r:
                return True
            elif l > r:
                return False
        elif isinstance(l, int) and isinstance(r, list):
            recursive = compare_lists([l], r)
            if recursive is not None:
                return recursive
        elif isinstance(l, list) and isinstance(r, int):
            recursive = compare_lists(l, [r])
            if recursive is not None:
                return recursive
    if len(left) > len(right):
        return False
    elif len(left) < len(right):
        return True
    else:
        return None


# part 1
indxs = []
pairs_copy = copy.deepcopy(pairs)
for k, v in pairs_copy.items():
    cl = compare_lists(v["l"], v["r"])
    # print(f"{k+1}: {cl}")
    if cl:
        indxs.append(k + 1)
print(sum(indxs))

# part 2
all_lists = (
    [v["l"] for v in pairs.values()]
    + [v["r"] for v in pairs.values()]
    + [[[2]]]
    + [[[6]]]
)
srtd = []
for j, l in enumerate(all_lists):
    if j == 0:
        srtd.append(l)
        continue
    for i, sr in enumerate(srtd):
        cl = compare_lists(copy.deepcopy(sr), copy.deepcopy(l))
        if not cl:
            srtd.insert(i, all_lists[j])
            break
    else:
        srtd.append(all_lists[j])


# print results
res = []
for i, sr in enumerate(srtd):
    print(f"{sr}\n")
    if sr == [[6]] or sr == [[2]]:
        res.append(i + 1)
print(res, res[0] * res[1])
