import itertools

rules: list[tuple[int, int]] = []
updates: list[tuple[int, ...]] = []
with open('inputs.txt') as f:
    update_section = False
    for line in f.readlines():
        if line.strip() == '':
            update_section = True
            continue

        if not update_section:
            rules.append(tuple(map(int, line.strip().split('|'))))
        else:
            updates.append(tuple(map(int, line.strip().split(','))))


rules_dict = {k: tuple(x[1] for x in g) for k, g in itertools.groupby(
    sorted(rules, key=lambda x: x[0]), key=lambda x: x[0]
)}
has_rules = set(x for r in rules for x in r)

safe_middle_sum = 0
for u in updates:
    states = ['u' for _ in u]
    wip = []
    def dfs_visit(idx: int):
        number_left = u[idx]
        if states[idx] == 'p':
            return
        if states[idx] == 't':
            raise ValueError('Cycle detected')
        states[idx] = 't'

        neighbors = list(rules_dict.get(number_left, []))
        neighbors = [n for n in neighbors if n in u]
        neighbors.extend([n for n in u if n not in has_rules])
        for n in neighbors:
            dfs_visit(u.index(n))

        states[idx] = 'p'
        wip.append(u[idx])

    while any(s == 'u' for s in states):
        for idx, s in enumerate(states):
            if s == 'u':
                dfs_visit(idx)
                break
    wip = tuple(reversed(wip))
    if u != wip:
        safe_middle_sum += wip[len(u) // 2]
print(safe_middle_sum)


