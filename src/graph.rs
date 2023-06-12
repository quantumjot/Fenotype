use uuid::Uuid;
// use std::collections::HashMap;


#[allow(dead_code)]
pub struct Node {
    uid: Uuid,
    coords: [f64; 3],
    // features: Option<HashMap<String, f64>>,
}

pub struct Edge {
    pub uid: Uuid,
    source: Node,
    target: Node,
}


#[allow(dead_code)]
pub struct Graph {
    pub uid: Uuid,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}


impl Node {
    pub fn new(coords: [f64; 3]) -> Node {
        let uid: Uuid = Uuid::new_v4();
        return Node { uid, coords };
    }
}


impl Edge {
    pub fn new(source: Node, target: Node) -> Edge {
        let uid: Uuid = Uuid::new_v4();
        return Edge { uid, source, target };
    }

    pub fn length(&self) -> f64 {
        let mut len: f64 = 0.0;
        let ndim: usize = self.source.coords.len();

        for idx in 0..ndim {
            len += (self.target.coords[idx] - self.source.coords[idx]).powi(2);
        }

        return len.sqrt();
    }
}