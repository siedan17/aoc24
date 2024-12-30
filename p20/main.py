from dataclasses import dataclass

with open("input.txt") as f: s = f.readlines()

@dataclass
class grid:
    data: list[list[str]]
    start: tuple[int, int]
    end: tuple[int, int]
    path: list[tuple[int, int]]

    def in_path(self, point: tuple[int, int]) -> bool:
        return point in self.path

    def pretty_print(self):
        for line in self.data:
            s = ""
            for x in line:
                s += x
            print(s)

    def set_path(self):
        self.path = []
        pos = self.start
        while True:
            self.path.append(pos)
            news = [(pos[0] + 1, pos[1]), (pos[0] - 1, pos[1]), (pos[0], pos[1] + 1), (pos[0], pos[1] - 1)]
            for new in news:
                if not self.in_path(new) and self.data[new[0]][new[1]] == ".":
                    pos = new
                    break
                if not self.in_path(new) and self.data[new[0]][new[1]] == "E":
                    self.path.append(new)
                    return

data = []
start, end = (), ()

for i in range(len(s)):
    line = s[i].strip()
    row = []
    for j in range(len(line)):
        if line[j] == "S":
            start = (i, j)
        if line[j] == "E":
            end = (i, j)
        row.append(line[j])
    data.append(row)

board = grid(data, start, end, path=[])

board.set_path()
# board.pretty_print()

cheats = { # cheat_reduction: [((1), (2)), ... ]

}

def man_dist(a, b):
    return abs(a[0] - b[0]) + abs(a[1] - b[1])

for i in range(len(board.path)):
    for j in range(len(board.path)):
        if j <= i:
            continue
        d = man_dist(board.path[i], board.path[j])
        if d <= 20:
            if j - i - d > 0:
                if j - i - d in cheats.keys():
                    cheats[j - i - d].append((board.path[i], board.path[j]))
                else:
                    cheats[j - i - d] = [(board.path[i], board.path[j])]


counter = 0
for k, v in cheats.items():
    # if k > 0:
    #     print(k, len(v))

    if k >= 100:
        counter += len(v)

print(counter)



