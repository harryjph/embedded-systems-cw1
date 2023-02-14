use super::node::Node;
use std::hash::{Hash, Hasher};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Link {
    pub nodes: [usize; 2], //just save the node ids here so link can be copied and does not have to be mutable
    pub cost: f64,
}

impl Hash for Link {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.nodes.hash(state);
    }
}
impl Eq for Link {}

impl Link {
    pub fn new(node1: &Node, node2: &Node) -> Self {
        use std::f64::consts::PI;
        const EARTH_RADIUS_M: f64 = 6371.0 * 1000.0;

        let a = 0.5 - ((node2.y_coord-node1.y_coord)*PI/180.0).cos()/2.0 + (node1.y_coord*PI/180.0).cos()*(node2.y_coord*PI/180.0).cos() * (1.0-((node2.x_coord-node1.x_coord)*PI/180.0).cos()) / 2.0;
        
        let cost = 2.0 * EARTH_RADIUS_M * a.sqrt().asin();

        Self {
            nodes: [node1.node_id, node2.node_id],
            cost: cost,
        }
    }

    pub fn is_link(&self, node1: usize, node2: Option<usize>) -> bool {
        match node2 {
            Some(node) => {
                if self.nodes.contains(&node1) && self.nodes.contains(&node) {
                    true
                } else {
                    false
                }
            }
            None => {
                if self.nodes.contains(&node1) {
                    true
                } else {
                    false
                }
            }
        }
    }
    pub fn other_node(&self, node: usize) -> usize {
        if node == self.nodes[1] {
            self.nodes[0]
        } else {
            self.nodes[1]
        }
    }
}
