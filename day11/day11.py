import re

import numpy as np


class Monkey:
    """docstring for Monkey."""

    def __init__(
        self,
        id,
        items,
        op,
        div,
        true_monkey,
        false_monkey,
        reduce_worry=3,
        set_mod_worry=1,
    ):
        self.id = id
        self.items = items
        self.op = op
        self.div = div
        self.true_monkey = true_monkey
        self.false_monkey = false_monkey
        self.n_inspections = 0
        self.reduce_worry = reduce_worry
        self.mod_worry = None

    def __repr__(self):
        return f"monkey: {self.id}, items: {self.items} , op: {self.op} ,div: {self.div} ,true: {self.true_monkey} ,false: {self.false_monkey}, n_inspect: {self.n_inspections}"

    def test(self, x):
        return x % self.div == 0

    def operation(self, old):
        self.n_inspections += 1
        return eval(self.op)

    def bored(self, old):
        return old // self.reduce_worry

    def update_item(self):
        item = self.items.pop(0)
        wl = self.operation(item)
        br = self.bored(wl)
        if self.mod_worry is not None:
            br = br % self.mod_worry
        return br, self.test(br)

    def append_item(self, val):
        self.items.append(val)


with open("./data/day11.txt") as f:
    file = f.readlines()
monkeys = []

reduce_worry = 1  # 3 for part 1

for i, l in enumerate(file):
    if re.match("Monkey \d*", l):
        items = [
            int(item)
            for item in file[i + 1].lstrip("Starting items: ").strip().split(",")
        ]
        op = re.search("\= (.*)", file[i + 2]).groups()[0]
        div = int(re.search("divisible by (\d+)", file[i + 3]).groups()[0])
        true_monkey = int(re.search("If true.*(\d+)", file[i + 4]).groups()[0])
        false_monkey = int(re.search("If false.*(\d+)", file[i + 5]).groups()[0])
        monkeys.append(
            Monkey(
                len(monkeys),
                items,
                op,
                div,
                true_monkey,
                false_monkey,
                reduce_worry=reduce_worry,
            )
        )
mod = np.lcm.reduce([m.div for m in monkeys])
for m in monkeys:
    m.mod_worry = mod
# 20 rounds for part 1
for r in range(10000):
    for m in monkeys:
        while len(m.items) > 0:
            br, test = m.update_item()
            if test:
                monkeys[m.true_monkey].append_item(br)
            else:
                monkeys[m.false_monkey].append_item(br)

print(monkeys)
most_active = sorted([m.n_inspections for m in monkeys])[-2:]
print(most_active[0] * most_active[1])
