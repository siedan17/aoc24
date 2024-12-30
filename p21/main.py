import itertools
from functools import cache

with open("input.txt") as f: s = f.readlines()

recursive_steps = 25
numbers = [x.strip() for x in s]
print("input, recursive steps: ", numbers, recursive_steps)

big_pad = {
    "7": (0, 0),
    "8": (1, 0),
    "9": (2, 0),
    "4": (0, 1),
    "5": (1, 1),
    "6": (2, 1),
    "1": (0, 2),
    "2": (1, 2),
    "3": (2, 2),
    "0": (1, 3),
    "A": (2, 3),
}

small_pad = {
    "^": (1, 0),
    "A": (2, 0),
    "<": (0, 1),
    "v": (1, 1),
    ">": (2, 1),
}

steps = {
    "<": (-1, 0),
    ">": (1, 0),
    "v": (0, 1),
    "^": (0, -1),
}

@cache
def check_valid_big_pad(pos, path) -> bool:
    for step in path:
        pos = (pos[0] + steps[step][0], pos[1] + steps[step][1])
        if pos[0] == 0 and pos[1] == 3:
            return False
    return True

@cache
def check_valid_small_pad(pos, path) -> bool:
    for step in path:
        pos = (pos[0] + steps[step][0], pos[1] + steps[step][1])
        if pos[0] == 0 and pos[1] == 0:
            return False
    return True

@cache
def get_all_moves(x_diff, y_diff):
    moves = []
    x_bool = x_diff < 0
    y_bool = y_diff < 0
    for _ in range(abs(x_diff)):
        if x_bool:
            moves.append("<")
        else:
            moves.append(">")
    for _ in range(abs(y_diff)):
        if y_bool:
            moves.append("^")
        else:
            moves.append("v")
    return list(set(itertools.permutations(moves)))

@cache
def small_pad_trajectories(input, depth):
    if depth == 0:
        return len(input) # base case
    depth -= 1
    result = 0 # best path
    current = "A"
    for letter in input:
        curr = small_pad[current]
        goal = small_pad[letter]
        x_diff = goal[0] - curr[0]
        y_diff = goal[1] - curr[1]
        trajectories = get_all_moves(x_diff, y_diff)
        best = -1
        for trajectory in trajectories:
            if not check_valid_small_pad(curr, trajectory):
                    continue
            trajectory = list(trajectory)
            trajectory.append("A")
            next_step = small_pad_trajectories(tuple(trajectory), depth)
            if best < 0 or next_step < best:
                best = next_step
        result += best
        current = letter
    return result


results = []
for word in numbers:
    best_path = 0
    current = "A"
    for letter in word:
        curr = big_pad[current]
        goal = big_pad[letter]
        current = letter
        x_diff = goal[0] - curr[0]
        y_diff = goal[1] - curr[1]
        trajectories = get_all_moves(x_diff, y_diff)
        best = -1
        for trajectory in trajectories:
            if not check_valid_big_pad(curr, trajectory):
                continue
            trajectory = list(trajectory)
            trajectory.append("A")

            state = small_pad_trajectories(tuple(trajectory), recursive_steps)
            if best < 0 or state < best:
                best = state

        best_path += best
    results.append(best_path)

sum = 0
for i in range(len(numbers)):
    sum += int(numbers[i][:-1]) * results[i]

print("sum: ", sum)









