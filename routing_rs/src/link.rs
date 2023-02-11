use std::hash::{Hash, Hasher};
use crate::node::Node; 

#[derive (PartialEq, Copy, Clone, Debug)]
pub
struct Link{
    pub nodes: [u32; 2], //just save the node ids here so link can be copied and does not have to be mutable
    pub cost: f64
}

impl Hash for Link{
    fn hash<H: Hasher> (&self, state: &mut H) {
        self.nodes.hash(state);
    }
}
impl Eq for Link{}

impl Link{
    pub fn new(node1: Node, node2: Node) -> Self {
        let cost: f64 = ((node1.x_coord - node2.x_coord).powf(2.0) + (node1.y_coord - node2.y_coord).powf(2.0)).sqrt();
        Self { 
            nodes: [node1.node_id, node2.node_id],
            cost: cost
        }
        
    }

    pub fn is_link(&self, node1: u32, node2: Option<u32>) -> bool {
        match node2{
            Some(node) => {
                if self.nodes.contains(&node1) && self.nodes.contains(&node){
                    true
                } else {false}
            }
            None => {
                if self.nodes.contains(&node1){true}
                else{false}
            }
        }
    }
    pub fn other_node(&self, node: u32) -> u32{
        if node== self.nodes[1]{
            self.nodes[0]
        } else {
            self.nodes[1]
        }
    }
}
