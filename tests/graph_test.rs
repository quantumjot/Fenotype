#[cfg(test)]
mod tests {

    use fenotype::base::Node;
    use fenotype::graph::Graph;
    use fenotype::networks::gnm_random_graph;

    use std::collections::HashSet;

    #[test]
    fn test_new_graph() {
        let _graph: Graph = Graph::new(false);

        assert_eq!(_graph.num_nodes(), 0);
        assert_eq!(_graph.num_edges(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut _graph: Graph = Graph::new(false);
        _graph.add_node(Node::new(0));
        assert_eq!(_graph.num_nodes(), 1);
    }

    #[test]
    fn test_remove_node() {
        let mut _graph: Graph = Graph::new(false);
        _graph.add_node(Node::new(0));

        assert!(_graph.has_node(0));
        _graph.remove_node(0);
        assert!(!_graph.has_node(0));
    }

    #[test]
    #[should_panic]
    fn test_add_edge_without_nodes() {
        let mut _graph: Graph = Graph::new(false);
        assert_eq!(_graph.num_edges(), 0);
        _graph.add_edge(0, 1);
    }

    #[test]
    fn test_add_edge() {
        let mut _graph: Graph = Graph::new(false);
        _graph.add_node(Node::new(0));
        _graph.add_node(Node::new(1));
        _graph.add_edge(0, 1);

        assert_eq!(_graph.num_nodes(), 2);
        assert_eq!(_graph.num_edges(), 1);
    }

    #[test]
    fn test_has_edge() {
        let mut _graph: Graph = Graph::new(false);
        _graph.add_node(Node::new(0));
        _graph.add_node(Node::new(1));
        _graph.add_edge(0, 1);

        assert!(_graph.has_edge(0, 1));
    }

    #[test]
    #[should_panic]
    fn test_doesnt_have_edge() {
        let mut _graph: Graph = Graph::new(false);
        _graph.add_node(Node::new(0));
        _graph.add_node(Node::new(1));
        
        assert!(_graph.has_edge(0, 1));
    }

    #[test]
    fn test_remove_edge() {
        let mut _graph: Graph = Graph::new(false);
        _graph.add_node(Node::new(0));
        _graph.add_node(Node::new(1));
        _graph.add_edge(0, 1);

        assert!(_graph.has_edge(0, 1));
        assert_eq!(_graph.num_edges(), 1);

        _graph.remove_edge(0, 1);

        assert!(!_graph.has_edge(0, 1));
        assert_eq!(_graph.num_edges(), 0);
    }

    #[test]
    fn test_random_graph() {
        let num_nodes: u64 = 10;
        let num_edges: u64 = 5;

        let _graph: Graph = gnm_random_graph(num_nodes, num_edges, false);

        assert_eq!(
            _graph.num_nodes(),
            num_nodes as usize,
            "incorrect number of nodes"
        );
        assert_eq!(
            _graph.num_edges(),
            num_edges as usize,
            "incorrect number of edges"
        );
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
        assert_eq!(_graph.neighbors(0).unwrap(), &HashSet::from([1, 2]));

        // there should be no neighbors of node 3
        assert_eq!(_graph.neighbors(3), None,);
    }
}
