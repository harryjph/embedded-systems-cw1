use std::hash::{Hash, Hasher};
use crate::node::Node; 

#[derive (PartialEq)]
pub
struct Link{
    nodes: [Node; 2],
    pub cost: f64
}

impl Hash for Link{
    fn hash<H: Hasher> (&self, state: &mut H) {
        self.nodes.hash(state);
    }
}
impl Eq for Link{}

impl Link{
    fn new(&mut self, node1: Node, node2: Node) {
        let cost: f64 = ((node1.x_coord - node2.x_coord).powf(2.0) + (node1.y_coord - node2.y_coord).powf(2.0)).sqrt();
        self.cost = cost;
        self.nodes = [node1, node2];
    }

    fn is_active(&self) -> bool {
        if self.nodes[0].needs_emptying && self.nodes[1].needs_emptying {
            return false
        } else { 
            return true
        }
    }

    fn is_link(&self, node1: Node, node2: Option<Node>) -> bool {
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
    fn other_node(&self, node: Node) -> &Node{
        if node == self.nodes[1]{
            &self.nodes[0]
        } else {
            &self.nodes[1]
        }
    }
}
