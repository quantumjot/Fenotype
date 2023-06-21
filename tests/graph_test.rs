#[cfg(test)]
mod tests {

    use fenotype::base::Node;
    use fenotype::graph::Graph;
    use fenotype::networks::gnm_random_graph;

    use std::collections::HashSet;

    #[test]
    fn test_random_graph() {

        let num_nodes: u64 = 10;
        let num_edges: u64 = 5;

        let _graph: Graph = gnm_random_graph(num_nodes, num_edges, false);

        assert_eq!(_graph.num_nodes(), num_nodes as usize, "incorrect number of nodes");
        assert_eq!(_graph.num_edges(), num_edges as usize, "incorrect number of edges");
    }

    #[test]
    fn test_neighbors() {
    
        let mut _graph: Graph = Graph::new_undirected();

        _graph.add_node(Node::new(0));
        _graph.add_node(Node::new(1));
        _graph.add_node(Node::new(2));
        _graph.add_node(Node::new(3));

        _graph.add_edge(0, 1);
        _graph.add_edge(0, 2);

        // there should be 2 neighbors of node 0
        assert_eq!(
            _graph.neighbors(0).unwrap(), 
            &HashSet::from([1, 2])
        );

        // there should be no neighbors of node 3
        assert_eq!(
            _graph.neighbors(3), 
            None,
        );    
    }
}