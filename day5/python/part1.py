import itertools

rules: list[tuple[int, int]] = []
updates: list[list[int]] = []
with open('../inputs.txt') as f:
    update_section = False
    for line in f.readlines():
        if line.strip() == '':
            update_section = True
            continue

        if not update_section:
            rules.append(tuple(map(int, line.strip().split('|'))))
        else:
            updates.append(list(map(int, line.strip().split(','))))

# O(r*log(r))
rules_dict = {k: set(x[0] for x in g) for k, g in itertools.groupby(
    sorted(rules, key=lambda x: x[1]), key=lambda x: x[1]
)}

safe_middle_sum = 0
# O(u*len(u)*r)
for u in updates:
    all_numbers = set(u)
    seen_numbers = set()
    for n in u:
        if n in rules_dict and any(
            (x in all_numbers and x not in seen_numbers) for x in rules_dict[n]
        ):
            break

        seen_numbers.add(n)
    else:
        safe_middle_sum += u[len(u) // 2]
print(safe_middle_sum)