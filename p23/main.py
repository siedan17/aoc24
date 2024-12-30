from copy import deepcopy
with open("input.txt") as f: s = f.readlines()

edges = set()
vertices = set()

for line in s:
    line = line.strip()
    v1 = line[0:2]
    v2 = line[3:]
    vertices.add(v1)
    vertices.add(v2)
    edges.add((v1, v2))

print(len(edges))
print(len(vertices))

# three_cliques = set()
# for i in vertices:
#     for j in vertices:
#         for k in vertices:
#             if ((i, j) in edges or (j, i) in edges) and ((k, i) in edges or (i, k) in edges) and ((k,j) in edges or (j, k) in edges):
#                 new = (i, j, k)
#                 if not new in three_cliques:
#                     three_cliques.add(new)

max_clique = set()
for v in vertices:
    vert = deepcopy(vertices)
    vert.remove(v)
    new_clique = set()
    old = len(new_clique)
    new_clique.add(v)
    new = len(new_clique)
    while new > old and len(vert) > 0:
        old = new
        v1 = ""
        for p in vert:
            a = True
            for e in new_clique:
                if not ((e, p) in edges or (p, e) in edges):
                    a = False
                    break
            if a:
                v1 = p
                break
        if v1 != "":
            new_clique.add(v1)
            vert.remove(v1)
        new = len(new_clique)

    if len(new_clique) > len(max_clique):
        max_clique = new_clique

print(max_clique)
a = list(max_clique)
a.sort()
l = ""
for x in a:
    l += x + ","
print(l)