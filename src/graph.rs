use crate::base::*;
use std::collections::HashMap;
use std::collections::HashSet;
use uuid::Uuid;
// use log::{info, warn};

/// Graph
pub struct Graph {
    _uid: Uuid,
    directed: bool,
    _node_map: HashMap<i64, Node>,
    _edge_map: HashMap<i64, HashSet<i64>>,
}

impl Graph {
    pub fn new(directed: bool) -> Graph {
        let uid: Uuid = Uuid::new_v4();

        return Graph {
            _uid: uid,
            directed: directed,
            _node_map: HashMap::new(),
            _edge_map: HashMap::new(),
        };
    }

    pub fn get_uid(&self) -> Uuid {
        return self._uid;
    }

    pub fn nodes(&self) -> Vec<Node> {
        let nodes: Vec<Node> = Vec::new();
        return nodes;
    }

    pub fn edges(&self) -> Vec<Edge> {
        let edges: Vec<Edge> = Vec::new();
        return edges;
    }

    pub fn add_node(&mut self, node: Node) {
        self._node_map.entry(node.id).or_insert(node);
    }

    pub fn remove_node(&mut self, node: Node) {
        if self._node_map.contains_key(&node.id) {
            self._node_map.remove(&node.id);
        }
    }

    pub fn num_nodes(&self) -> usize {
        return self._node_map.len();
    }

    pub fn add_edge(&mut self, source_id: i64, target_id: i64) {
        println! {"Adding {} -> {}", source_id, target_id};

        if source_id == target_id {
            panic!("Source and target id cannot be the same: {}", source_id);
        }

        self._edge_map
            .entry(source_id)
            .and_modify(|targets| {
                targets.insert(target_id);
            })
            .or_insert(HashSet::from([target_id]));

        if !self.directed {
            self._edge_map
                .entry(target_id)
                .and_modify(|sources| {
                    sources.insert(source_id);
                })
                .or_insert(HashSet::from([source_id]));
        }
    }

    // Remove an edge from the graph
    pub fn remove_edge(&mut self, source_id: i64, target_id: i64) {
        println! {"Removing {} -> {}", source_id, target_id};

        self._edge_map.entry(source_id).and_modify(|targets| {
            targets.remove(&target_id);
        });
    }

    // Iterate over the stored graph and return the number of edges
    pub fn num_edges(&self) -> usize {
        let mut num_edges: usize = 0;
        for key in self._edge_map.keys() {
            let edges = self._edge_map.get(key).unwrap().len();
            num_edges += edges;
        }

        // TODO(arl): I'm not sure I like this
        if self.directed {
            return num_edges;
        } else {
            return num_edges / 2;
        }
    }

    // Return a bool to indicate whether the edges is found in the map
    pub fn has_edge(&self, source_id: i64, target_id: i64) -> bool {
        match self._edge_map.get(&source_id) {
            Some(edges) => return edges.contains(&target_id),
            _ => return false,
        }
    }

    pub fn neighbors(&self, source_id: i64) -> Option<&HashSet<i64>> {
        return self._edge_map.get(&source_id);
    }
}
