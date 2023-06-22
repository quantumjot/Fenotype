use crate::base::*;
use std::collections::hash_map::IntoValues;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use uuid::Uuid;
// use log::{info, warn};

pub trait GraphBuilder {
    fn from_edges(edges: Vec<[i64; 2]>, directed: bool) -> Self;
    fn from_file(file_path: &Path, directed: bool) -> Self;
}

/// Graph
pub struct Graph {
    _uid: Uuid,
    _directed: bool,
    _node_map: HashMap<i64, Node>,
    _edge_map: HashMap<i64, HashSet<i64>>,
}

impl GraphBuilder for Graph {
    // Create a new graph from an edge list
    fn from_edges(edges: Vec<[i64; 2]>, directed: bool) -> Self {
        let mut graph = Self::new(directed);

        for edge in edges {
            let source_id = edge[0];
            let target_id = edge[1];
            graph.add_node(Node::new(source_id));
            graph.add_node(Node::new(target_id));
            graph.add_edge(source_id, target_id);
        }

        return graph;
    }

    // Create a new graph from an edge list file
    fn from_file(file_path: &Path, directed: bool) -> Self {
        let contents = fs::read_to_string(file_path.as_os_str()).expect("Cannot parse file.");

        let rows = contents.lines();
        let mut edges: Vec<[i64; 2]> = Vec::new();

        for row in rows {
            let edge_indices: Vec<i64> =
                row.split_whitespace().map(|s| s.parse().unwrap()).collect();

            if edge_indices.len() != 2 {
                panic!("Edge list doesn't contain two entries per row.");
            }

            edges.push([edge_indices[0], edge_indices[1]]);
        }

        return Self::from_edges(edges, directed);
    }
}

impl Graph {
    pub fn new(directed: bool) -> Self {
        let uid: Uuid = Uuid::new_v4();

        return Self {
            _uid: uid,
            _directed: directed,
            _node_map: HashMap::new(),
            _edge_map: HashMap::new(),
        };
    }

    pub fn new_directed() -> Self {
        return Self::new(true);
    }

    pub fn new_undirected() -> Self {
        return Self::new(false);
    }

    pub fn get_uid(&self) -> Uuid {
        return self._uid;
    }

    pub fn nodes(&self) -> IntoValues<i64, Node> {
        let nodes = self._node_map.clone().into_values();

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

    // add a new edge to the graph
    pub fn add_edge(&mut self, source_id: i64, target_id: i64) {
        println! {"Adding {} -> {}", source_id, target_id};

        if source_id == target_id {
            panic!("Source and target id cannot be the same: {}", source_id);
        }

        // check that the nodes exist already
        self._node_map
            .get(&source_id)
            .expect("Source node doesn't exist");

        self._node_map
            .get(&target_id)
            .expect("Target node doesn't exist");

        self._edge_map
            .entry(source_id)
            .and_modify(|targets| {
                targets.insert(target_id);
            })
            .or_insert(HashSet::from([target_id]));

        if !self._directed {
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
        if self._directed {
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
