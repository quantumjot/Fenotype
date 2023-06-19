use cell_graph::graph::Graph;
use cell_graph::networks::gnm_random_graph;

fn main() {
    let mut _graph: Graph = gnm_random_graph(10, 5, false);

    println!(
        "Graph: {} (nodes: {}, edges: {})",
        _graph.get_uid(),
        _graph.num_nodes(),
        _graph.num_edges()
    );

    println!("Has edge (0, 1): {}", _graph.has_edge(0, 1));
    println!("Has edge (0, 2): {}", _graph.has_edge(0, 2));

    _graph.remove_edge(0, 1);

    println!("Has edge (0, 1): {}", _graph.has_edge(0, 1));
}
