
with open("input.txt") as f: lines = f.readlines()
grid = []
moves = []
append_to_moves = False
for line in lines:
    if line.strip() == "":
        append_to_moves = True
        continue
    elif append_to_moves:
        moves += list(line.strip())
    else:
        grid.append(list(line.strip()))
# print(grid, moves)

def wider_map(grid):
    new_grid = []
    for line in grid:
        new_line = []
        for symbol in line:
            if symbol == "#":
                new_line += ["#", "#"]
            if symbol == "O":
                new_line += ["[", "]"]
            if symbol == ".":
                new_line += [".", "."]
            if symbol == "@":
                new_line += ["@", "."]
        new_grid.append(new_line)
    return new_grid

new_grid = wider_map(grid)

def find_start(grid) -> tuple[int, int]:
    for i in range(len(grid[0])):
        for j in range(len(grid)):
            if grid[j][i] == "@":
                return (i, j)
    return (-1, -1)

def next_trial_pos(current, symbol) -> tuple[int, int]:
    next_x, next_y = 0, 0
    if symbol == "^":
        next_x, next_y = current[0], current[1] - 1
    elif symbol == "v":
        next_x, next_y = current[0], current[1] + 1
    elif symbol == "<":
        next_x, next_y = current[0] - 1, current[1]
    elif symbol == ">":
        next_x, next_y = current[0] + 1, current[1]
    return (next_x, next_y)

def next_trial_positions(current, symbol) -> list[tuple[int, int]]:
    result = []
    for pos in current:
        new = next_trial_pos(pos, symbol)
        result.append(new)
    return result

def shift(grid, current, next, symbol) -> bool:
    if grid[next[1]][next[0]] == "#":
        return False
    if grid[next[1]][next[0]] == ".":
        grid[next[1]][next[0]] = grid[current[1]][current[0]]
        grid[current[1]][current[0]] = "."
        return True
    if grid[next[1]][next[0]] in ["[", "]"]:
        new_next = next_trial_pos(next, symbol)
        if shift(grid, next, new_next, symbol):
            grid[next[1]][next[0]] = grid[current[1]][current[0]]
            grid[current[1]][current[0]] = "."
            return True
        else:
            return False

def shift_extended(grid, current, next, symbol) -> bool:
    if symbol == "<" or symbol == ">":
        assert len(next) == 1
        assert len(current) == 1
        return shift(grid, current[0], next[0], symbol)

    for i in range(len(next)):
        if grid[next[i][1]][next[i][0]] == "#":
            return False

    for_all = True
    for i in range(len(current)):
        if grid[next[i][1]][next[i][0]] != ".":
            for_all = False
            break
    if for_all:
        for i in range(len(next)):
            grid[next[i][1]][next[i][0]] = grid[current[i][1]][current[i][0]]
            grid[current[i][1]][current[i][0]] = "."
        return True

    new_current = []
    for i in range(len(next)):
        if grid[current[i][1]][current[i][0]] == "[":
            if grid[next[i][1]][next[i][0]] == "[":
                new_current.append(current[i])
            if grid[next[i][1]][next[i][0]] == "]":
                new_current.append(current[i])
                new_current.append((current[i][0] - 1, current[i][1]))

        if grid[current[i][1]][current[i][0]] == "]":
            if grid[next[i][1]][next[i][0]] == "]":
                new_current.append(current[i])
            if grid[next[i][1]][next[i][0]] == "[":
                new_current.append(current[i])
                new_current.append((current[i][0] + 1, current[i][1]))

        if grid[current[i][1]][current[i][0]] == "@":
            if grid[next[i][1]][next[i][0]] == "]":
                new_current.append(current[i])
                new_current.append((current[i][0] - 1, current[i][1]))

            if grid[next[i][1]][next[i][0]] == "[":
                new_current.append(current[i])
                new_current.append((current[i][0] + 1, current[i][1]))

    new_current = list(set(new_current))
    new_next = next_trial_positions(new_current, symbol)
    new_current = new_next
    new_next = next_trial_positions(new_current, symbol)
    if shift_extended(grid, new_current, new_next, symbol):
        for i in range(len(current)):
            grid[next[i][1]][next[i][0]] = grid[current[i][1]][current[i][0]]
            grid[current[i][1]][current[i][0]] = "."
        return True
    else:
        return False

def calc_score(grid):
    result = 0
    for i in range(len(grid[0])):
        for j in range(len(grid)):
            if grid[j][i] == "[":
                result += i + 100*j
    return result

current = [find_start(new_grid)]

for move in moves:
    next_trials = next_trial_positions(current, move)
    if shift_extended(new_grid, current, next_trials, move):
        current = [find_start(new_grid)]

print("result: ", calc_score(new_grid))

for l in new_grid:
    n = ""
    for i in l:
        n += i
    print(n)
print("\n")
