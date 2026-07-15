# /// script
# requires-python = ">=3.14"
# dependencies = [
#   "networkx",
#   "pydot",
#   "matplotlib"
# ]
# ///

import json
import networkx
import matplotlib.pyplot as plt

def load_ir(path: str) -> networkx.MultiDiGraph:
    with open(path) as f:
        data = json.load(f)
    return networkx.node_link_graph(
        data, directed=True, multigraph=True, edges="links"
    )

def main() -> None:
    g = load_ir("test.json")

    print(f"{g.number_of_nodes()} ops, {g.number_of_edges()} dataflow edges")
    print(f"dialect: {g.graph['dialect']}, depth: {g.graph['depth']}")

    pos = networkx.nx_pydot.graphviz_layout(g, prog="dot")
    networkx.draw(g, pos, with_labels=True)
    plt.show()


if __name__ == "__main__":
    main()
