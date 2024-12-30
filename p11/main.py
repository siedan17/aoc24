from functools import cache

with open('input.txt') as f: s = f.read()
numbers = [int(x) for x in s.split(" ")]

# def multiply(nums: list[int], n: int):
#     for i in range(n):
#         new_nums = []
#         for num in nums:
#             x = apply(num)
#             for y in x:
#                 new_nums.append(int(y))
#         nums = new_nums
#     return nums

@cache
def new(num: int, height: int):
    new_nums = []
    x = apply(num)
    for y in x:
        new_nums.append(int(y))
    if height == 1:
        return len(new_nums)
    return sum(new(s, height - 1) for s in new_nums)

def apply(num: int):
    if num == 0:
        return [1]
    if len(str(num)) % 2 == 0:
        return [num // (10**(len(str(num))/2)), num % (10**(len(str(num))/2))]
    else:
        return [num * 2024]

print(sum(new(x, 75) for x in numbers))