#[cfg(test)]
mod tests {

    use cell_graph::graph::Graph;
    use cell_graph::networks::gnm_random_graph;

    #[test]
    fn test_random_graph() {

        let num_nodes: u64 = 10;
        let num_edges: u64 = 5;

        let _graph: Graph = gnm_random_graph(num_nodes, num_edges, false);

        assert_eq!(_graph.num_nodes(), num_nodes as usize, "incorrect number of nodes");
        assert_eq!(_graph.num_edges(), num_edges as usize, "incorrect number of edges");
    }
}