with open('input.txt') as f: s = f.read()

def stretch(s: str):
    streched = []
    even = 0
    for i in range(len(s)):
        if i % 2 == 0:
            for _ in range(int(s[i])):
                streched.append(even)
            even += 1
        else:
            for _ in range(int(s[i])):
                streched.append(".")
    return streched

# def get_point_idx(list_char):
#     for i in range(len(list_char)):
#         if list_char[i] == ".":
#             return i
#     return 0

def get_next_point_idx(list_char, start_idx):
    for i in range(start_idx, len(list_char)):
        if list_char[i] == ".":
            counter = 1
            j = i + 1
            while type(list_char[j]) == str and list_char[j] == ".":
                counter+=1
                j += 1
            return i, counter
    return 0, 0

def get_last_num(list_char, current_id):
    found = False
    value = 0
    counter = 0
    for i in range(len(list_char)):
        v = list_char[len(list_char) - (i + 1)]
        if (type(v) == str or v > current_id) and not found:
            continue
        if (type(v) == str and v == ".") or (found and v != value):
            return (list_char[len(list_char) - (i)], len(list_char) - i, counter)
        found = True
        value = v
        counter += 1
    return 0, 0, 0

# def resort(list_char):
#     last_char = len(list_char) - 1
#     first_point = get_point_idx(list_char)
#     while last_char > first_point:
#         if type(list_char[last_char]) != str:
#             list_char[first_point] = list_char[last_char]
#             list_char[last_char] = "."
#             first_point = get_point_idx(list_char)
#         last_char -= 1
#     return list_char

def resort(list_char):
    value_to_sort, value_idx, count = get_last_num(list_char, len(list_char))
    open_position, open_count = get_next_point_idx(list_char, 0)

    while value_to_sort > 0:
        while value_idx > open_position + 1:
            if open_count >= count:
                for k in range(count):
                    list_char[open_position+k] = value_to_sort
                for j in range(count):
                    list_char[value_idx+j] = "."
                open_position, open_count = get_next_point_idx(list_char, open_position+open_count)
                break
            open_position, open_count = get_next_point_idx(list_char, open_position+open_count)

        value_to_sort, value_idx, count = get_last_num(list_char, value_to_sort - 1)
        open_position, open_count = get_next_point_idx(list_char, 0)

    return list_char

def checksum(list_char):
    result = 0
    for i in range(len(list_char)):
        if type(list_char[i]) == str:
            continue
        result += i * list_char[i]
    return result

print("checksum: ", checksum(resort(stretch(s))))