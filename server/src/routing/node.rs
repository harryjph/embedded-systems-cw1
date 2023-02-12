use std::hash::{Hash, Hasher};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Node {
    pub x_coord: f64,
    pub y_coord: f64,
    pub node_id: usize,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_id.hash(state);
    }
}

impl Eq for Node {}

impl Node {
    pub fn new(x_coord: f64, y_coord: f64, node_id: usize, dummy: f64) -> Node {
        Self {
            x_coord,
            y_coord,
            node_id,
        }
    }
}
