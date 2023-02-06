use crate::link::Link; 
use crate::node::Node; 

struct Network{
    nodes: Vec<Node>,
    links: Vec<Link>,
    start_point: Node,
    max_cost: f64
}

impl Network{
    fn new(&mut self, links: Vec<Link>, nodes: Vec<Node>, start_point: Node){
        self.nodes = nodes; 
        self.start_point = start_point; 
        let mut max_cost: f64 = 0.0;
        for link in &links {
            max_cost += link.cost;
        }
        self.max_cost = max_cost;
        self.links = links; 
    }
    
}