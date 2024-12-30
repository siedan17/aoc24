
program = "2,4,1,3,7,5,4,0,1,3,0,3,5,5,3,0"
program = program.split(",")
program = [int(x) for x in program]

reg_A = 51342988
reg_B = 0
reg_C = 0

combos = {
    0: 0,
    1: 1,
    2: 2,
    3: 3,
    4: reg_A,
    5: reg_B,
    6: reg_C,
    7: False, # error
}

def first(v):
    x = combos[4] / (2**combos[v])
    combos[4] = int(x)
    return

def second(v):
    combos[5] = combos[5] ^ v
    return

def third(v):
    combos[5] = combos[v] % 8
    return

def fourth(v):
    if combos[4] == 0:
        return
    return v

def fifth(v):
    combos[5] = combos[5] ^ combos[6]
    return

def sixth(v):
    print("here: ", combos[5], combos[5] % 8)
    return combos[v] % 8

def seventh(v):
    x = combos[4] / (2**combos[v])
    combos[5] = int(x)
    return

def eigth(v):
    x = combos[4] / (2**combos[v])
    combos[6] = int(x)
    return


def run_instruction(n, v):
    result = None
    match n:
        case 0:
            result = first(v)
        case 1:
            result = second(v)
        case 2:
            result = third(v)
        case 3:
            result = fourth(v)
        case 4:
            result = fifth(v)
        case 5:
            result = sixth(v)
        case 6:
            result = seventh(v)
        case 7:
            result = eigth(v)
    return result


def run_program(program):
    pointer = 0
    output = []
    while pointer < len(program)-1:
        p = program[pointer]
        v = program[pointer+1]
        result = run_instruction(p, v)
        if p == 3 and type(result) == int:
            pointer = result
            continue
        if p == 5:
            output.append(result)
        pointer += 2
    return output

a = run_program(program)
a = ",".join([str(x) for x in a])
print(a)
print(combos)