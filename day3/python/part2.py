import re

with open("instructions.txt") as f:
    instructions = f.read().replace("\n", "")
valid_sections = re.findall(r"(?:^|do\(\))(.*?)(?:don't\(\)|$)", instructions)
p = re.compile(r".*?mul\(([0-9]+),([0-9]+)\).*?")
print(sum([int(x) * int(y) for s in valid_sections for x, y in p.findall(s)]))
# 92626942
