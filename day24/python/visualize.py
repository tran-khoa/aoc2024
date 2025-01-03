import graphviz
import re

with open("../inputs.txt") as f:
    line_str = f.read()

regex = re.compile(r"(.{3}) (XOR|AND|OR) (.{3}) -> (.{3})")
statements = regex.findall(line_str)

dot = graphviz.Digraph()
dot.attr(rankdir="LR")
for statement in statements:
    x = statement[0]
    y = statement[2]
    z = statement[3]
    op = statement[1]
    op_id = f"{x}-{y}-{op}"

    dot.node(op_id, label=f"{op}", shape="circle")
    dot.edge(x, op_id)
    dot.edge(y, op_id)
    dot.edge(op_id, z)

dot.render("circuit.gv", view=True)
print(statements)