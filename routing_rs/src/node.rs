use std::hash::{Hash, Hasher};

#[derive(PartialEq)]
pub struct Node {
    pub x_coord: f64,
    pub y_coord: f64,
    fill_level: f64,
    node_id: u32,
    capacity: f64,
    pub needs_emptying: bool, // potentially add fields to keep track of how much time has passed
                              // since it was last emptied
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node_id.hash(state);
    }
}

impl Eq for Node {}

impl Node {
    fn update_fill_level(&mut self, new_level: f64) {
        self.fill_level = new_level
    }
}
