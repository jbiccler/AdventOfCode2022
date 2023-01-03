import copy

import numpy as np


def shift(nums, indx):
    # find the current num to change:
    tmp = copy.deepcopy(nums)
    current_indx = [t[1] for t in tmp].index(indx)
    target_num, target_indx = tmp[current_indx]
    tmp.pop(current_indx)
    sh = (current_indx + target_num) % len(tmp)
    tmp.insert(sh, (target_num, target_indx))
    return tmp


def grove_numbers(numbers):
    for indx, (n, i) in enumerate(numbers):
        if n == 0:
            zero_indx = indx
            break
    indxs = (zero_indx + np.array([1000, 2000, 3000])) % len(numbers)
    groves = [numbers[i][0] for i in indxs]
    return groves, np.sum(groves)


if __name__ == "__main__":
    decrypt_key = 811589153  # set to 1 for part 1
    times_to_mix = 10  # set to 1 for part 1

    with open("./data/day20.txt") as f:
        nums = []
        for i, line in enumerate(f.readlines()):
            nums.append((int(line[:-1]) * decrypt_key, i))

    for t in range(times_to_mix):
        for i in range(len(nums)):
            c = (t + 1) * i
            if c % 100 == 0:
                print(f"Iter: {c}, {c/len(nums)/times_to_mix*100:.2f}")
            nums = shift(nums, i)
    print(nums)
    print(grove_numbers(nums))
