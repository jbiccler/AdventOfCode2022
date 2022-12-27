import pandas as pd
import string

alpha = list(string.ascii_lowercase)
alpha_map = {
    **{a: (i + 1) for i, a in enumerate(alpha)},
    **{a.upper(): len(alpha) + (i + 1) for i, a in enumerate(alpha)},
}
file = pd.read_csv("./data/day3.csv", header=None)


class Rucksack:
    def __init__(self, string):
        self.string = string
        self.n = len(string)
        self.n2 = self.n // 2
        self._populate_compartments()
        self._find_common()

    def _populate_compartments(self):
        self.comp1 = [*self.string[: self.n2]]
        self.comp2 = [*self.string[self.n2 :]]
        self.total_set = set(self.comp1 + self.comp2)

    def _find_common(self):
        (self.common,) = set(self.comp1).intersection(set(self.comp2))
        self.common_val = alpha_map[self.common]

    def find_group_badge(self, other1, other2):
        (badge,) = self.total_set.intersection(other1.total_set).intersection(
            other2.total_set
        )
        badge_val = alpha_map[badge]
        self.badge, other1.badge, other2.badge = [badge] * 3
        self.badge_val, other1.badge_val, other2.badge_val = [badge_val] * 3


rucks = []

for i in range(file.shape[0]):
    rucks.append(Rucksack(file.iloc[i].values[0]))

print(sum([r.common_val for r in rucks]))

badges = []

for i in range(0, file.shape[0], 3):
    rucks[i].find_group_badge(rucks[i + 1], rucks[i + 2])
    badges.append(rucks[i].badge_val)

print(sum(badges))
