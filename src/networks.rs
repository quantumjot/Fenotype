// use core::num;
use rand;
use std::cmp;
use rand::seq::SliceRandom;

// use uuid::Uuid;

use crate::base::Node;
use crate::graph::Graph;
// use sets::Set;
// use std::collections::HashMap;

fn permutations(n: u64, m: u64) -> Vec<[u64; 2]> {
    let mut _perm: Vec<[u64; 2]> = Vec::new();
    for i in 0..n {
        for j in (i+1)..m {
            _perm.push([i, j]);
        }
    }
    return _perm;
}

fn random_node(id: i64, _scale: Option<f64>) -> Node {
    let coords: [f64; 3] = rand::random();
    return Node::new(id, coords);
}

// fn random_edge(max_edges: i64) -> Edge {

// }

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

    println!("Max edges: {}", max_edges);

    // if m >= max_edges {
    //     return complete_graph(n, directed);
    // }

    let mut graph: Graph = Graph::new(directed);
    for idx in 0..n {
        graph.add_node(random_node(idx as i64, Some(1.)));
    }

    // in the case where we only have one node, return the graph
    if n == 1 {
        return graph;
    }

    let perms = permutations(n, max_edges);

    // NOTE(arl): nice method to randomly select items from the vector of edges
    let sample: Vec<_> = perms
        .choose_multiple(&mut rand::thread_rng(), max_edges as usize)
        .collect();

    for edge in sample {
        graph.add_edge(edge[0] as i64, edge[1] as i64);
    }

    return graph;
}
