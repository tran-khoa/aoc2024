from collections import defaultdict

import torch

enc = defaultdict(lambda: 0, {"M": 1, "A": 2, "S": 3})
base = 4

with open("../puzzle.txt") as f:
    arr = torch.tensor([[enc[c] for c in label.strip()] for label in f.readlines()])
kernel = torch.tensor([[base**0, 0, base**1], [0, base**2, 0], [base**3, 0, base**4]])
out = torch.nn.functional.conv2d(
    arr[None, None, :, :], kernel[None, None, :, :], padding=0
)
print(torch.isin(out, torch.tensor([997, 487, 877, 367])).sum().item())
