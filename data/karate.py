import networkx as nx

from pathlib import Path


def write_karate_network(filename: Path) -> None:
    G = nx.karate_club_graph()
    edges = G.edges()

    with open(filename.with_suffix(".tsv"), "w") as edge_file:
        edge_file.writelines(f"{source} \t {target} \n" for source, target in edges)


if __name__ == "__main__":
    write_karate_network(Path(__file__).parent / "karate.tsv")
