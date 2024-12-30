from functools import cache

with open("input.txt") as f: s = f.readlines()

options = [o.strip() for o in s[0].split(", ")]
examples = [e.strip() for e in s[2:]]

@cache
def dfs1(input: str, options: tuple[str]) -> bool:
    for i in range(len(options)):
        option = options[i]
        if input[:len(option)] == option:
            if len(input[len(option):]) == 0:
                return True
            x = dfs1(input[len(option):], options)
            if x:
                return x
    return False

@cache
def dfs(input: str, options: tuple[str]) -> int:
    result = 0
    for i in range(len(options)):
        new_result = 0
        option = options[i]
        if input[:len(option)] == option:
            if len(input[len(option):]) == 0:
                new_result = 1
            else:
                x = dfs(input[len(option):], options)
                if x != 0:
                    new_result = x
        result += new_result
    return result


counter = 0
for e in examples:
    if dfs1(e, tuple(options)):
        counter += 1
    # counter += dfs(e, tuple(options))

print(counter)