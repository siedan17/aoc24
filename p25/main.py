

with open("input.txt") as f: s = f.readlines()

keys = []
locks = []

new = []
data = []
for line in s:
    line = line.strip()
    if line == "":
        data.append(new)
        new = []
        continue
    new.append([l for l in line])
data.append(new)

for block in data:
    key = False
    for b in block[0]:
        if b != "#":
            key = True
            break

    if key:
        block = block[::-1]

    heights = [-1 for _ in range(len(block[0]))]
    for i in range(len(block)):
        for j in range(len(block[0])):
            if block[i][j] == "#":
                heights[j] += 1

    if key:
        keys.append(heights)
    else:
        locks.append(heights)

# print(keys)
# print(locks)

def check(key, lock):
    sum = []
    assert len(key) == len(lock)
    for i in range(len(key)):
        sum.append(key[i] + lock[i])
    if max(sum) > 5:
        return False
    return True


counter = 0
for key in keys:
    for lock in locks:
        if check(key, lock):
            counter += 1

print(counter)



