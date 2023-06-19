// use core::num;
use rand;
use std::cmp;
// use uuid::Uuid;

use crate::network::base::{Node, Edge};
use crate::network::graph::Graph;
// use sets::Set;
// use std::collections::HashMap;

fn random_node(id: i64, scale: Option<f64>) -> Node {
    let coords: [f64; 3] = rand::random();
    return Node::new(id, coords);
}

// fn random_edge(max_edges: i64) -> Edge {

// }


// Implementation of networkx `complete_graph`
pub fn complete_graph(n: u64, m: u64, directed: bool) -> Graph {
    let mut graph: Graph = Graph::new();
    return graph;
}

// Implementation of networkx `gnm_random_graph`
pub fn gnm_random_graph(n: u64, m: u64, directed: bool) -> Graph {
    let mut graph: Graph = Graph::new();

    for idx in 0..n {
        graph.add_node(random_node(idx as i64, Some(1.)));
    }

    // in the case where we only have one node, return the graph
    if n == 1 {
        return graph;
    }

    let mut max_edges: u64 = n * (n - 1);

    match directed {
        true => (),
        false => max_edges /= 2,
    }

    max_edges = cmp::min(max_edges, m);

    graph.add_edge(0, 1);

    return graph;
}