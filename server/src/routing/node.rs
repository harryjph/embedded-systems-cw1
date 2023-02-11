use std::hash::{Hash, Hasher};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Node {
    pub x_coord: f64,
    pub y_coord: f64,
    fill_level: f64,
    pub node_id: usize,
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
    pub fn new(x_coord: f64, y_coord: f64, node_id: usize, capacity: f64) -> Node {
        Self {
            x_coord,
            y_coord,
            node_id,
            capacity,
            fill_level: 0.0,
            needs_emptying: false,
        }
    }
    pub fn update_fill_level(&mut self, new_level: Option<f64>) {
        match new_level {
            Some(lev) => self.fill_level = lev,
            _ => {}
        }
        if self.fill_level > self.capacity {
            self.needs_emptying = true;
        }
    }
}
