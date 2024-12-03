import re

if __name__ == "__main__":
    with open("instructions.txt") as f:
        instructions = f.read().replace("\n", "")
    matches = re.findall(r".*?mul\(([0-9]+),([0-9]+)\).*?", instructions)
    print(sum(map(lambda tup: int(tup[0]) * int(tup[1]), matches)))
# 159892596
