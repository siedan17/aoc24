
with open("input.txt") as f: s = f.readlines()

wires = {}
gates = []

append_wires = True
for line in s:
    if line == "\n":
        append_wires = False
        continue
    l = line.split()
    if append_wires:
        wires[l[0][:-1]] = l[1] == "1"
        continue
    else:
        gates.append((l[0], l[1], l[2], l[4]))

def calc_decimal(dict: dict, letter: str) -> int:
    result = 0
    for k in dict.keys():
        if k[0] == letter and dict[k]:
            result += 2**(int(k[1:]))
    return result

def calc_gate(first: bool, second: bool, gate: str) -> bool:
    match gate:
        case "AND":
            return first and second
        case "OR":
            return first or second
        case "XOR":
            return first != second
    raise ValueError

def update_wires(wires: dict, gates: list[tuple]):
    keys = wires.keys()
    all = False
    while not all:
        all = True
        for gate in gates:
            if not gate[0] in keys or not gate[2] in keys:
                all = False
                continue
            wires[gate[3]] = calc_gate(wires[gate[0]], wires[gate[2]], gate[1])
        keys = wires.keys()

update_wires(wires, gates)

result_to_gate = {}
for gate in gates:
    result_to_gate[gate[3]] = gate

def xor_check(gate):
    # x,y then not OR or z and from XOR and OR
    input1 = gate[0]
    input2 = gate[2]
    if input1[0] in ["x", "y"] and input2[0] in ["x", "y"]:
        if gate[3][0] == "z":
            print("Error x, y, z", gate)
            return
        return

    if gate[3][0] == "z":
        a = result_to_gate[input1]
        b = result_to_gate[input2]
        if not a[1] in ["XOR", "OR"] or not b[1] in ["XOR", "OR"]:
            print("error xor, or", gate, a, b)
            return
        return

    print("something wrong ", gate)
    return

def and_check(gate):
    if gate[3][0] == "z":
        print("and Z", gate)
    return


def or_check(gate):
    if "z" in gate[3]:
        print("False Z:", gate)
        return
    r1 = result_to_gate[gate[0]]
    r2 = result_to_gate[gate[2]]
    if not (r1[1] == "AND" and r2[1] == "AND"):
        print("FALSE, up: ", gate, r1, r2)
    return


for gate in gates:
    match gate[1]:
        case "XOR":
            xor_check(gate)
        case "AND":
            and_check(gate)
        case "OR":
            or_check(gate)


