import itertools

import numpy as np

with open('../map.txt') as f:
    height_map = [[int(c) for c in line.strip()] for line in f.readlines()]

complex_map = {complex(row, col): height_map[row][col] for row in range(len(height_map)) for col in
               range(len(height_map[0]))}

num_nodes = len(height_map) * len(height_map[0])
adjacency_matrix = np.zeros((num_nodes, num_nodes))


def idx(c: complex) -> int:
    return int(c.real * len(height_map[0]) + c.imag)


zero_indices = set()
nine_indices = set()


def dfs_adj(pos: complex):
    value = complex_map[pos]
    if value == 0:
        zero_indices.add(idx(pos))
    elif value == 9:
        nine_indices.add(idx(pos))
        return
    for delta in [1, -1, 1j, -1j]:
        new_pos = pos + delta
        if new_pos not in complex_map:
            continue
        if adjacency_matrix[idx(pos), idx(new_pos)] == 1:
            continue
        if complex_map[new_pos] == value + 1:
            adjacency_matrix[idx(pos), idx(new_pos)] = 1
            dfs_adj(new_pos)


[dfs_adj(complex(k)) for k, v in complex_map.items() if v == 0]
paths = np.linalg.matrix_power(adjacency_matrix, 9)
num_paths = sum(paths[i, j] for i, j in itertools.product(zero_indices, nine_indices))
print(num_paths)
