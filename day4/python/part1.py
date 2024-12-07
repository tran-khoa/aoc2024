with open("../puzzle.txt") as f:
    pz = [[c for c in label.strip()] for label in f.readlines()]

count = 0
for r in range(len(pz)):
    for c in range(len(pz[0])):
        if pz[r][c] not in {"X", "S"}:
            continue
        if pz[r][c] == "X":
            target = "XMAS"
        else:
            target = list(reversed("XMAS"))

        # row
        if c + len(target) - 1 < len(pz[0]):
            for i in range(len(target)):
                if pz[r][c + i] != target[i]:
                    break
            else:
                count += 1

        # column
        if r + len(target) - 1 < len(pz):
            for i in range(len(target)):
                if pz[r + i][c] != target[i]:
                    break
            else:
                count += 1

        # diagonal
        if c + len(target) - 1 < len(pz[0]) and r + len(target) - 1 < len(pz):
            for i in range(len(target)):
                if pz[r + i][c + i] != target[i]:
                    break
            else:
                count += 1

        if c >= len(target) - 1 and r + len(target) - 1 < len(pz):
            for i in range(len(target)):
                if pz[r + i][c - i] != target[i]:
                    break
            else:
                count += 1
print(count)
