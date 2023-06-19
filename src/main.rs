mod network;


fn main() {

    let mut _graph: network::graph::Graph = network::networks::gnm_random_graph(10, 1, false);



    println!(
        "Graph: {} (nodes: {}, edges: {})",
        _graph.get_uid(),
        _graph.num_nodes(),
        _graph.num_edges()
    );

    println!("Has edge: {}", _graph.has_edge(0, 1));

    _graph.remove_edge(0, 1);

    println!("Has edge: {}", _graph.has_edge(0, 1));

}