use crate::network::base::*;
use sets::Set;
use std::collections::HashMap;
use uuid::Uuid;

struct EdgeMap {
    pub map: HashMap<i64, Set<i64>>,
}

/// EdgeMap
///
/// A container to store edges for quick lookup of neighbors
///
impl EdgeMap {
    pub fn new() -> EdgeMap {
        return EdgeMap {
            map: HashMap::new(),
        };
    }

    // insert an edge into the edge map
    pub fn insert(&mut self, source_id: i64, target_id: i64) {
        self.map
            .entry(source_id)
            .or_insert_with(Set::new_empty)
            .insert(target_id);
    }

    // remove an edge if it exists
    pub fn remove(&mut self, source_id: i64, target_id: i64) {
        self.map
            .entry(source_id)
            .and_modify(|targets: &mut Set<i64>| {
                targets.delete(target_id);
            });
    }

    // Iterate over the stored graph and return the number of edges
    pub fn num_edges(&self) -> usize {
        let mut num_edges: usize = 0;
        for key in self.map.keys() {
            let edges = self.map.get(key).unwrap().data.len();
            num_edges += edges;
        }
        return num_edges;
    }

    pub fn neighbors(&self, source_id: i64) -> Option<&Set<i64>> {
        return self.map.get(&source_id);
    }
}

/// Graph
pub struct Graph {
    _uid: Uuid,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    directed: bool,
    _map: EdgeMap,
}

impl Graph {
    pub fn new() -> Graph {
        let uid: Uuid = Uuid::new_v4();

        let mut _nodes: Vec<Node> = Vec::new();
        let mut _edges: Vec<Edge> = Vec::new();

        let _directed: bool = false;
        let _map: EdgeMap = EdgeMap::new();

        return Graph {
            _uid: uid,
            nodes: _nodes,
            edges: _edges,
            directed: _directed,
            _map: _map,
        };
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_edge(&mut self, source_id: i64, target_id: i64) {
        if source_id == target_id {
            panic!("Source and target id cannot be the same: {}", source_id);
        }

        let edge = Edge::new(source_id, target_id);
        self.edges.push(edge);

        self._map.insert(source_id, target_id);
    }

    // Remove an edge from the graph
    pub fn remove_edge(&mut self, source_id: i64, target_id: i64) {
        let mut iter = self.edges.iter();

        let idx = iter.position(|&x| x.source_id == source_id && x.target_id == target_id);

        match idx {
            Some(val) => self.edges.remove(val),
            _ => panic!("Cannot find edge {} -> {}.", source_id, target_id),
        };

        self._map.remove(source_id, target_id);
    }

    // Check whether an edge exists
    pub fn has_edge(&self, source_id: i64, target_id: i64) -> bool {
        let mut iter = self.edges.iter();
        let _has_edge = iter
            .find(|&x| x.source_id == source_id && x.target_id == target_id)
            .is_some();
        return _has_edge;
    }

    pub fn num_nodes(&self) -> usize {
        return self.nodes.len();
    }

    pub fn num_edges(&self) -> usize {
        return self.edges.len();
    }

    pub fn get_uid(&self) -> Uuid {
        return self._uid;
    }

    pub fn neighbors(&self, node_id: i64) -> Vec<Node> {
        let neighbors: Vec<Node> = Vec::new();
        return neighbors;
    }
}




