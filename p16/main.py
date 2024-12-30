from heapq import *
import copy

with open("input.txt") as f: lines = f.readlines()

board = []
for line in lines:
    board.append(line.strip())

# for b in board:
#     print(b)

# find Start and End
start = (-1, -1)
end = (-1, -1)
direction = ">"

for i in range(len(board)):
    for j in range(len(board[0])):
        if board[i][j] == "S":
            start = (i, j)
        if board[i][j] == "E":
            end = (i, j)


# Dijkstra (best first)

def get_neighbors_costs(board, pos, direction) -> tuple[tuple[tuple[int], int, str]]:
    result = ()
    new_pos = []
    new_directions = [direction]
    cost = 0
    if direction == ">":
        new_pos.append((pos[0], pos[1] + 1))
        new_pos.append((pos[0] - 1, pos[1]))
        new_pos.append((pos[0] + 1, pos[1]))
        new_directions.append("^")
        new_directions.append("v")
    if direction == "<":
        new_pos.append((pos[0], pos[1] - 1))
        new_pos.append((pos[0] - 1, pos[1]))
        new_pos.append((pos[0] + 1, pos[1]))
        new_directions.append("^")
        new_directions.append("v")
    if direction == "^":
        new_pos.append((pos[0] - 1, pos[1]))
        new_pos.append((pos[0], pos[1] - 1))
        new_pos.append((pos[0], pos[1] + 1))
        new_directions.append("<")
        new_directions.append(">")
    if direction == "v":
        new_pos.append((pos[0] + 1, pos[1]))
        new_pos.append((pos[0], pos[1] - 1))
        new_pos.append((pos[0], pos[1] + 1))
        new_directions.append("<")
        new_directions.append(">")
    for i in range(len(new_pos)):
        if board[new_pos[i][0]][new_pos[i][1]] != "#":
            if i == 0:
                cost = 1
            else:
                cost = 1001
            result += ((new_pos[i], cost, new_directions[i]), )
    return result

def print_board(board, path):
    board = copy.deepcopy(board)
    for p in path:
        l = list(board[p[0]])
        l[p[1]] = "X"
        board[p[0]] = "".join(l)
    for b in board:
        print(b)

def dijkstra(board, start, direction, end) -> tuple[int, tuple[tuple[int]]]:
    paths, dist = [(0, start, (), direction)], {start: 0}
    result = []
    best = 10**100
    while paths:
        cost, pos, path, dir = heappop(paths)
        if cost > best:
            break
        if cost > dist[pos] + 1000:
            continue
        path += (pos, )
        if pos == end:
            result.append([cost, path])
            best = cost
            continue
        turn = False
        for new_pos, add_cost, new_direction in get_neighbors_costs(board, pos, dir):
            old_cost = dist.get(new_pos, 10**100)
            if add_cost > 100:
                turn = True
            new_cost = cost + add_cost
            heappush(paths, (new_cost, new_pos, path, new_direction))
            if new_cost <= old_cost:
                dist[new_pos] = new_cost
        if turn:
            continue

    return result


result = dijkstra(board, start, direction, end)

print(len(result))
# for path in result:
#     print(path)
#     print_board(board, path[1])

positions = set()
for path in result:
    for p in path[1]:
        positions.add(p)

print(len(positions))






