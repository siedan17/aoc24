
with open("input.txt") as f: s = f.readlines()

numbers = []
for line in s:
    numbers.append(int(line))

# print(numbers)

MOD = 16777216

def new_secret_num(number: int) -> int:
    # first
    x = number * 64
    number = number ^ x
    number = number % MOD

    # second
    y = int(number / 32)
    number = number ^ y
    number = number % MOD

    # third
    z = number * 2048
    number = number ^ z
    number = number % MOD

    return number

result = {}
for n in numbers:
    x = n
    result[n] = [x]
    for _ in range(2000):
        x = new_secret_num(x)
        result[n].append(x)
    # result[n] = x

decimals = {}
for k, v in result.items():
    decimals[k] = [x % 10 for x in v]

differences = {}
for k, v in decimals.items():
    differences[k] = [0]
    for i in range(1, len(v)):
        differences[k].append(v[i] - v[i-1])

# print(result)
# print(decimals)
# print(differences)

dict_dicts = {}
for k, v in differences.items():
    new_dict = {}
    for i in range(1, len(v) - 4):
        seq = tuple(v[i:i+4])
        value = decimals[k][i+3]
        if seq not in new_dict.keys():
            new_dict[seq] = value
    dict_dicts[k] = new_dict

def update_dict(input, new_dict):
    for k, v in new_dict.items():
        if k in input.keys():
            input[k] += v
        else:
            input[k] = v
    return input


merged_dict = {}
for v in dict_dicts.values():
    merged_dict = update_dict(merged_dict, v)

print(max(merged_dict.values()))


# sum = 0
# for item in result.values():
#     sum += item
# print(sum)


