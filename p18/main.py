from heapq import *
import math

with open("input.txt") as f: s = f.readlines()

SIZE = 71
# BYTES = 1024

grid = [["." for _ in range(SIZE)] for _ in range(SIZE)]

flaws = []
for line in s:
    b = line.split(',')
    x = int(b[0])
    y = int(b[1])
    flaws.append((x, y))

# for i in range(BYTES):
#     cood = flaws[i]
#     grid[cood[1]][cood[0]] = "#"

def get_neighbors(grid, pos):
    positions = []
    for x, y in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
        new = (pos[0] + x, pos[1] + y)
        if new[0] < 0 or new[0] >= len(grid[0]) or new[1] < 0 or new[1] >= len(grid) or grid[new[1]][new[0]] == "#":
            continue
        positions.append(new)
    return positions

def get_distance(pos, end):
    # return math.sqrt((pos[0] - end[0])**2 + (pos[1] - end[1])**2)
    return abs(pos[0] - end[0]) + abs(pos[1] - end[1])

def a_star(grid, start, end):
    paths, dist = [(0, 0, start, ())], {start: 0}
    result = []
    while paths:
        _, cost, pos, path = heappop(paths)
        if cost > dist[pos]:
            continue
        path += (pos, )
        if pos == end:
            result = [cost, path]
            break
        for new_pos in get_neighbors(grid, pos):
            old_cost = dist.get(new_pos, 10**100)
            new_cost = cost + 1
            if new_cost < old_cost:
                distance = get_distance(new_pos, end)
                heappush(paths, (distance + new_cost, new_cost, new_pos, path))
                dist[new_pos] = new_cost
    return result



def compare(m):
    grid1 = [["." for _ in range(SIZE)] for _ in range(SIZE)]
    grid2 = [["." for _ in range(SIZE)] for _ in range(SIZE)]
    for j in range(m):
        cood = flaws[j]
        grid1[cood[1]][cood[0]] = "#"
    for j in range(m-1):
        cood = flaws[j]
        grid2[cood[1]][cood[0]] = "#"
    r1 = a_star(grid1, (0, 0), (SIZE-1, SIZE-1))
    r2 = a_star(grid2, (0, 0), (SIZE-1, SIZE-1))
    if r1 != [] and r2 != []:
        return -1
    if r1 == [] and r2 != []:
        return 0
    return 1




L = 0
R = len(flaws)
while L <= R:
    m = math.floor((L+R)/2)
    r_m = compare(m)
    if r_m == -1:
        L = m+1
        continue
    if r_m == 1:
        R = m-1
        continue
    print("successful: ", flaws[m-1], m)
    break





# r = a_star(grid, (0, 0), (SIZE-1, SIZE-1))
# print(r[0])

# for a in r[1]:
#     grid[a[1]][a[0]] = "0"

# for l in grid:
#     print(l)