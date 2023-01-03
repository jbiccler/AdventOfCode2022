import re

import numpy as np
import sympy


def recursive_find(d, path, recent):
    next_deps = []
    for r in recent:
        next_deps += d[r]["deps"]
    if len(next_deps) == 0:
        return path + recent
    else:
        return recursive_find(d, path + recent, next_deps)


if __name__ == "__main__":
    d = {}
    part1 = False
    with open("./data/day21.txt") as f:
        for line in f.readlines():
            reg = list(re.match("(\w+): (.*)", line).groups())
            sp = re.split(" * | + | / | - ", reg[-1])
            if not part1:
                if reg[0] == "root":
                    reg[-1] = reg[-1][:5] + "-" + reg[-1][6:]
                    sp[1] = "-"
            tmp = []
            deps = []
            nums = []
            for s in sp:
                if s not in ["*", "+", "/", "-"]:
                    try:
                        tmp.append(int(s))
                        nums.append(int(s))
                    except:
                        tmp.append(s)
                        deps.append(s)
            d[reg[0]] = {"formula": reg[-1], "num": nums, "deps": deps}
    max_prio = 0
    for k, v in d.items():
        d[k]["path"] = recursive_find(d, [], v["deps"])
        d[k]["priority"] = len(d[k]["path"])
        if d[k]["priority"] > max_prio:
            max_prio = d[k]["priority"]
    if part1:
        for i in range(max_prio + 1):
            for k, v in d.items():
                prio = v["priority"]
                if prio == i:
                    formula = f'{k} = {v["formula"]}'
                    exec(formula)
        print(root)
    else:
        # part 2
        # write as full equation by iteratively substituting the formulas
        # then solve with sympy
        equation = d["root"]["formula"]
        for i in range(max_prio, -1, -1):
            for k, v in d.items():
                prio = v["priority"]
                if prio == i and k != "humn":
                    equation = equation.replace(k, f"({v['formula']})")
                    if prio == 0:
                        formula = f'{k} = {v["formula"]}'
                        exec(formula)

        humn = sympy.symbols("humn")
        solved = sympy.solve(sympy.sympify(equation))
        print(solved)
