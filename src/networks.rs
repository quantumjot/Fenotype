// use core::num;
use rand;
use rand::seq::SliceRandom;
use std::cmp;


use crate::base::Node;
use crate::graph::Graph;
use crate::utils::unique_permutations;


// Implementation of networkx `complete_graph`
pub fn complete_graph(_n: u64, directed: bool) -> Graph {
    let graph: Graph = Graph::new(directed);
    return graph;
}


// Implementation of networkx `gnm_random_graph`
pub fn gnm_random_graph(n: u64, m: u64, directed: bool) -> Graph {
    let mut max_edges: u64 = n * (n - 1);

    if !directed {
        max_edges /= 2;
    }

    max_edges = cmp::min(max_edges, m);

    // if m >= max_edges {
    //     return complete_graph(n, directed);
    // }

    let mut graph: Graph = Graph::new(directed);
    for idx in 0..n {
        graph.add_node(Node::new(idx as i64));
    }

    // in the case where we only have one node, return the graph
    if n == 1 {
        return graph;
    }

    let perms = unique_permutations(n);

    // NOTE(arl): nice method to randomly select items from the vector of edges
    let sample: Vec<_> = perms
        .choose_multiple(&mut rand::thread_rng(), max_edges as usize)
        .collect();

    for edge in sample {
        graph.add_edge(edge[0] as i64, edge[1] as i64);
    }

    return graph;
}
