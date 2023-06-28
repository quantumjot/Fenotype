// use core::num;
use rand;
use rand::seq::SliceRandom;
use std::cmp;

// use uuid::Uuid;

use crate::base::Node;
use crate::graph::Graph;
// use sets::Set;
// use std::collections::HashMap;

fn unique_permutations(n: u64) -> Vec<[u64; 2]> {
    let _capacity: usize = ((n*(n-1))/2) as usize;
    let mut _perm: Vec<[u64; 2]> = Vec::with_capacity(_capacity);
    for i in 0..n {
        for j in (i + 1)..n {
            _perm.push([i, j]);
        }
    }
    // println!("{} -> {} unique permutations", n, _perm.len());
    return _perm;
}

// fn random_node(id: i64) -> Node {
//     return Node::new(id);
// }

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
