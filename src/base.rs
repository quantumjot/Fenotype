use std::collections::HashMap;
use std::string::String;

use uuid::Uuid;

pub trait ID {
    fn get_uid(&self) -> Uuid;
}

#[derive(Copy, Clone)]
struct Coordinates<T> {
    x: Option<T>,
    y: Option<T>,
    z: Option<T>,
    t: Option<T>,
}


#[derive(Copy, Clone)]
pub struct Node {
    _uid: Uuid,
    pub id: i64,
    // pub coords: Option<Coordinates<T>>,
}

impl Node {
    pub fn new(id: i64) -> Self {
        let uid: Uuid = Uuid::new_v4();
        return Self {
            _uid: uid,
            id,
        };
    }

    // pub fn new_with_coords<T>(id: i64, coords: Option<Coordinates<T>>) -> Self {
    //     let uid: Uuid = Uuid::new_v4();
    //     return Self {
    //         _uid: uid,
    //         id,
    //         coords,
    //     };
    // }
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
    pub fn new(source_id: i64, target_id: i64) -> Self {
        let uid: Uuid = Uuid::new_v4();
        return Self {
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
