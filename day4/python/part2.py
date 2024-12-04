from collections import defaultdict

import torch
import torch.nn.functional as F


def enc(letter):
    d = defaultdict(lambda: 0)
    d.update({"M": 1, "A": 2, "S": 3})
    return d[letter]


base = 4

with open("puzzle.txt") as f:
    arr = torch.tensor([[enc(c) for c in label.strip()] for label in f.readlines()])

kernel = torch.tensor([[base**0, 0, base**1], [0, base**2, 0], [base**3, 0, base**4]])
out = F.conv2d(arr[None, None, :, :], kernel[None, None, :, :], padding=0)

# possible_products = torch.tensor(
#     [
#         enc("A") * base**2
#         + (enc("M") * base**0 + enc("S") * base**4)
#         + (enc("M") * base**1 + enc("S") * base**3),
#         enc("A") * base**2
#         + (enc("S") * base**0 + enc("M") * base**4)
#         + (enc("M") * base**1 + enc("S") * base**3),
#         enc("A") * base**2
#         + (enc("M") * base**0 + enc("S") * base**4)
#         + (enc("S") * base**1 + enc("M") * base**3),
#         enc("A") * base**2
#         + (enc("S") * base**0 + enc("M") * base**4)
#         + (enc("S") * base**1 + enc("M") * base**3),
#     ]
# )
possible_products = torch.tensor([997, 487, 877, 367])
result = torch.isin(out, possible_products).sum().item()
print(result)
assert result == 1880
