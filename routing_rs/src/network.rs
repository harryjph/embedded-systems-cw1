use crate::link::{Link, self}; 
use crate::node::{Node, self}; 
use crate::matching;
use std::cmp::min;
use std::collections::HashMap;
use std::hash::Hash;
use std::{path, vec};

struct Network{
    nodes: HashMap<u32, Node>,
    links: Vec<Link>,
    start_point: Node,
    max_cost: f64
}

impl Network{
    fn new(&mut self, links: Vec<Link>, nodes: Vec<Node>, start_point: Node){
        let mut nodes_map: HashMap<u32, Node> = HashMap::new();
        for node in nodes.iter(){
            nodes_map.insert(node.node_id, *node);
        }
        self.start_point = start_point; 
        let mut max_cost: f64 = 0.0;
        for link in &links {
            max_cost += link.cost;
        }
        self.max_cost = max_cost;
        self.links = links; 
    }
    
    fn link_is_active(&self, link:Link) -> bool {
        if self.nodes[&link.nodes[0]].needs_emptying && self.nodes[&link.nodes[1]].needs_emptying {
            return false
        } else { 
            return true
        }
    }
    
    fn get_active_links(&self) -> Vec<Link> {
        let mut active_links: Vec<Link> = Vec::new();
        for link in self.links.iter() {
            if self.link_is_active(*link){
                active_links.push(*link);
            }
        } 
        active_links 
    }

    fn get_cost(&self, node1: u32, node2: u32) -> f64 {
        for link in self.links.iter(){
            if link.is_link(node1, Some(node1)){
                return link.cost;
            }
        }
        return self.max_cost;
    }

    fn get_link(&self, node1: u32, node2: u32) -> Result<Link, &str>{
        for link in self.links.iter(){
            if link.is_link(node1, Some(node2)){
                return Ok(*link); 
            }
        }
        return Err("there is no link for the two nodes, should not happen"); 
    }

    fn check_status(&mut self) {
        for (_, node) in self.nodes.iter_mut(){
            node.update_fill_level(None);
        }
    }

    // TODO check if I need the original copy of Node to track the change
    // I don't think it should because the only change to the node concerns the 
    // fill level which should be updated at network level
    fn min_cost(&self, costs: &HashMap<u32, (f64, Link)>) -> Node{
        let mut min_cost: f64 = self.max_cost;
        let mut min_node: Node = self.start_point; 
        for (node, cost) in costs.iter(){
            if cost.0 < min_cost {
                min_node = self.nodes[node];
                min_cost = cost.0;
            }
        }
        return min_node;
    }

    fn update_costs(&self, node: u32, nodes: &Vec<u32>, costs: HashMap<u32, (f64, Link)>) -> HashMap<u32, (f64, Link)> {
        let mut new_costs: HashMap<u32, (f64,Link)> = HashMap::new();
        for new_node in nodes.iter(){
            let cost: f64 = self.get_cost(*new_node, node);
            if cost < costs[new_node].0 {
                let link = self.get_link(node,*new_node);
                new_costs.insert(*new_node, (cost, link.unwrap()));
            } else {
                new_costs.insert(*new_node, costs[new_node]);
            }
        }
        return new_costs;
    }

    fn init_costs(&self, active_nodes: &Vec<u32>) -> HashMap<u32, (f64, Link)>{
        let mut costs: Vec<(f64, Link)> = Vec::new();
        for node in active_nodes.iter(){
            let link = self.get_link(*node, self.start_point.node_id).unwrap();
            let cost = self.get_cost(*node, self.start_point.node_id);
            costs.push((cost, link));
        }
        let mut costs_map = HashMap::new();
        for (node, cost) in active_nodes.iter().zip(costs.iter()){
            costs_map.insert(*node, *cost);
        }
        return costs_map;
    }

    fn prims_mst(&mut self) -> (Vec<Link>, Vec<u32>){
        self.check_status();
        let mut active_nodes: Vec<u32> = Vec::new();
        for (node_id, node) in self.nodes.iter(){
            if node.needs_emptying{
                active_nodes.push(*node_id);
            }
        }
        let mut mst_links: Vec<Link> = Vec::new(); 
        let mut mst_nodes: Vec<u32> = Vec::new();
        let mut costs: HashMap<u32, (f64, Link)> = HashMap::new();
        while active_nodes.len() != 0 {
            if mst_nodes.len() == 0{
                mst_nodes.push(self.start_point.node_id);
                active_nodes.retain(|node| {*node != self.start_point.node_id});
                costs = self.init_costs(&active_nodes);
            } else {
                let min_cost_node = self.min_cost(&costs).node_id;
                let link = costs[&min_cost_node].1;
                mst_links.push(link);
                mst_nodes.push(link.nodes[0]);
                mst_nodes.push(link.nodes[1]);
                active_nodes.retain(|node| {*node != min_cost_node});
                costs = self.update_costs(min_cost_node, &active_nodes, costs);
            }
        }
        return (mst_links, mst_nodes);
    }

    fn is_relaxed(&self, node: u32, relaxed_links: Vec<Link>) -> bool {
        let mut node_links: Vec<Link> = Vec::new();
        for link in relaxed_links.iter() {
            if link.nodes.contains(&node) {
                node_links.push(*link);
            }
        }
        if node_links.len() == 2 {return true} 
        else {return false}
    }

    fn links_per_node(&self, links: &Vec<Link>, nodes: &Vec<u32>, node_rel: Option<u32>) -> HashMap<u32, Vec<Link>> {
        let mut links_by_node: HashMap<u32, Vec<Link>> = HashMap::new();
        for node in nodes.iter(){
            let mut all_links = links.clone(); 
            all_links.retain(|link| {link.is_link(*node, None)});
            links_by_node.insert(*node, all_links);
        }
        match node_rel{
            Some(node) => {
                let mut all_links = links.clone(); 
                all_links.retain(|link| {link.is_link(node, None)});
                links_by_node.insert(node, all_links);
            }
            _ => {}
        }
        return links_by_node;
    }

    fn get_odd(&self, links: &Vec<Link>, nodes: Vec<u32>) -> (Vec<u32>, Vec<Link>){
        let mst_links_by_node: HashMap<u32, Vec<Link>> = self.links_per_node(&links, &nodes, None);
        let mut odd_nodes: Vec<u32> = Vec::new(); 
        for node in nodes.iter(){
            if mst_links_by_node[node].len() % 2 != 0 {
                odd_nodes.push(*node);
            }
        }
        let mut all_links_by_node: HashMap<u32, Vec<Link>> = self.links_per_node(&self.links, &odd_nodes, None);
        let mut all_odd_links: Vec<Link> = Vec::new(); 
        for (_, odd_links) in all_links_by_node.iter_mut(){
            all_odd_links.append(odd_links);
        }
        return (odd_nodes, all_odd_links)
    }

    fn is_entered(&self, node: u32, relaxed_links: &Vec<Link>) -> bool {
        let mut node_links: Vec<Link> = Vec::new();
        for link in relaxed_links.iter(){
            if link.nodes.contains(&node){
                node_links.push(*link);
            }
        }
        debug_assert!(node_links.len() <= 2);
        if node_links.len() == 1 {return true}
        else{return false}
    }

    fn remove_links_relaxed(&self, node: u32, relaxed_nodes: Vec<u32>, remaining_links: &mut Vec<Link>){
        let mut node_links = Vec::new(); 
        for link in remaining_links.iter(){
            if link.nodes.contains(&node){
                node_links.push(*link);
            }
        }
        node_links.retain(|link|{ relaxed_nodes.contains(&link.other_node(node))});
        remaining_links.retain(|link| { !node_links.contains(link)})
    }

    fn is_closing_link(&self, link: Link, relaxed_links: Vec<Link> ) -> bool {
        if self.is_entered( link.nodes[0], &relaxed_links) && self.is_entered(link.nodes[1], &relaxed_links){
            return true;
        } else {
            return false; 
        }
    }

    fn links_of_node(&self, node: u32, links: &Vec<Link>) -> Vec<Link>{
        let mut nodes_links = Vec::new();
        for link in links.iter() {
            if link.nodes.contains(&node) {
                nodes_links.push(*link);
            }
        }
        return nodes_links;
    }

    // this is probably not needed....
    fn is_dead_end(&self, links: Vec<Link>, start_point: u32, node: u32, link: Link) -> bool {
        let other = link.other_node(node);
        let start_node_links= self.links_of_node(start_point, &links);
        if start_node_links.len() == 1 {
            let final_link = start_node_links[1]; 
            let mut other_links = self.links_of_node(other, &links);
            other_links.retain(|other_link| {*other_link != link});
            if other_links.len() == 1 && final_link == other_links[0]{
                return true; 
            }
        }
        return false;
    }

    fn beginning_of_end(&self, start_point: u32, links: &Vec<Link>) -> (u32, Vec<Link>) {
        let mut forced_path = true; 
        let mut path: Vec<Link> = Vec::new(); 
        let mut next_node = start_point; 
        let mut available_links = links.clone(); 
        let mut boe = start_point;
        while forced_path && available_links.len() != 0 {
            let next_node_links = self.links_of_node(next_node, &available_links);
            if next_node_links.len() > 1 {
                forced_path = false;
            } else {
                let link = next_node_links[0];
                path.push(link);
                available_links.retain(|av_link| {*av_link != link});
                boe = link.other_node(next_node);
                next_node = boe;
            }
        }
        return (boe, path)
    }

    fn euler_tour(&self, links_all: Vec<Link>) -> Vec<Link>{
        let mut links: Vec<Link> = Vec::new(); 
        // handle double edges: if there is one just remove it completely
        for link in links_all.iter() {
            if links.contains(link){
                links.retain(|double_link| {*double_link != *link});
            } else {
                links.push(*link);
            }
        }

        let mut sorted_links: Vec<Link> = Vec::new(); 
        sorted_links.push(links[0]);
        let first_link = links[0];
        let mut last_node = first_link.nodes[0];
        let start_node = last_node; 
        links.retain(|used_link| {*used_link == first_link});
        
        while links.len() != 0 {
            let (_, mut path_to_end) = self.beginning_of_end(start_node, &links);
            let other = sorted_links[sorted_links.len()-1].other_node(last_node);
            let mut possible_links: Vec<Link> = Vec::new();
            for link in links.iter(){
                if link.nodes.contains(&other) && !link.nodes.contains(&last_node) && !path_to_end.contains(link){
                    possible_links.push(*link);
                }
            }
            // no possible links, close path
            if possible_links.len() == 0{
                path_to_end.reverse(); 
                for last_link in path_to_end.iter(){
                    sorted_links.push(*last_link);
                }
                break;
            }
            let next_link = possible_links[0];
            sorted_links.push(next_link);
            links.retain(|link| {*link != next_link});
            last_node = other; 
        }
        return sorted_links;
    }

    fn get_common_node(&self, link1: Link, link2: Link) -> Option<u32> {
        let node11 = link1.nodes[0]; 
        let node12 = link1.nodes[1]; 
        if link2.nodes.contains(&node11){return Some(node11)}
        else if link2.nodes.contains(&node12){return Some(node12)}
        else {return None}
    }

    fn hamiltonian_cycle(&self, links: &mut Vec<Link>) -> Vec<Link>{
        let mut visited_nodes: Vec<u32> = Vec::new();
        let first_link = links[0];
        let mut relaxed_links: Vec<Link> = Vec::new(); 
        relaxed_links.push(first_link);
        links.retain(|link| {*link != first_link});
        let start_node = self.get_common_node(first_link, links[1]).unwrap();
        visited_nodes.push(start_node);
        let mut prev_node = first_link.other_node(start_node);
        visited_nodes.push(prev_node);
        let mut needs_relaxation = false; 
        let mut orphan = self.start_point.node_id; 
        for link in links.iter(){
            let other = link.other_node(prev_node);
            if visited_nodes.contains(&other){
                if !needs_relaxation{
                    orphan = prev_node; 
                }
                prev_node = other; 
            } else {
                if needs_relaxation {
                    // do relaxation
                    let new_link = self.get_link(orphan, other).unwrap();
                    relaxed_links.push(new_link);
                    visited_nodes.push(other);
                    prev_node = other; 
                    needs_relaxation = false; 
                } else {
                    // add link normally
                    relaxed_links.push(*link);
                    visited_nodes.push(other);
                    prev_node = other; 
                }
            }
        }
        let by_node = self.links_per_node(&relaxed_links, &visited_nodes, None);
        let mut incomplete_nodes = Vec::new(); 
        for (node, links) in by_node.iter(){
            if links.len() < 2{
                incomplete_nodes.push(*node);
                debug_assert!(links.len() == 1);
            }
        }
        debug_assert!(incomplete_nodes.len() == 2);
        let closing_link = self.get_link(incomplete_nodes[0], incomplete_nodes[1]).unwrap();
        relaxed_links.push(closing_link);
        return relaxed_links;
    }
    
    fn christofides(&mut self) -> Vec<u32> {
        // prims to get mst 
        let (mut mst_links, mst_nodes)= self.prims_mst();
        // get all vertices with odd number of connections 
        let (nodes_odd, links_odd) = self.get_odd(&mst_links, self.nodes.keys().cloned().collect());
        // get mwpf 
        let min_w_tup: Vec<(u32,u32)> = matching::mwmatching(&links_odd);
        let min_w: Vec<Link>= min_w_tup.into_iter().map(|(node1, node2)| self.get_link(node1, node2).unwrap()).collect();
        // add nodes and links from mst nodes and links
        mst_links.extend(min_w);
        // get eulerian tour 
        let mut eulerian = self.euler_tour(mst_links);
        let hamilton = self.hamiltonian_cycle(&mut eulerian);
        let mut final_ids: Vec<u32> = Vec::new();
        final_ids.push(hamilton[0].nodes[0]);
        let mut prev_node = final_ids[0];
        for link in hamilton.iter(){
            let other = link.other_node(prev_node);
            final_ids.push(other);
            prev_node = other; 
        }
        return final_ids;
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_min_cost(){
        todo!()
    }
}