use uuid::Uuid;

pub trait ID {
    fn get_uid(&self) -> Uuid;
}

#[derive(Copy, Clone)]
pub struct Node {
    _uid: Uuid,
    pub id: i64,
    pub coords: Option<[f64; 3]>,
}

impl Node {
    pub fn new(id: i64) -> Node {
        let uid: Uuid = Uuid::new_v4();
        return Node {
            _uid: uid,
            id,
            coords: None,
        };
    }

    pub fn new_with_coords(id: i64, coords: Option<[f64; 3]>) -> Node {
        let uid: Uuid = Uuid::new_v4();
        return Node {
            _uid: uid,
            id,
            coords,
        };
    }
}

impl ID for Node {
    fn get_uid(&self) -> Uuid {
        return self._uid;
    }
}

#[derive(Copy, Clone)]
pub struct Edge {
    _uid: Uuid,
    pub source_id: i64,
    pub target_id: i64,
}

impl Edge {
    pub fn new(source_id: i64, target_id: i64) -> Edge {
        let uid: Uuid = Uuid::new_v4();
        return Edge {
            _uid: uid,
            source_id,
            target_id,
        };
    }
}

impl ID for Edge {
    fn get_uid(&self) -> Uuid {
        return self._uid;
    }
}
