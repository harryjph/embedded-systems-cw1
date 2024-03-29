mod link;
mod matching;
mod node;

use itertools::Itertools;
use link::Link;
pub use node::Node;
use std::cmp::min;
use std::collections::HashMap;
use std::hash::Hash;
use std::{path, vec};

pub struct Network {
    nodes: HashMap<usize, Node>,
    links: Vec<Link>,
    start_point: Node,
    max_cost: f64,
}

impl Network {
    pub fn new(links: Vec<Link>, nodes: Vec<Node>, start_point: Node, max_cost: f64) -> Self {
        Self {
            nodes: nodes.into_iter().map(|n| (n.node_id, n)).collect(),
            links,
            start_point,
            max_cost: f64::MAX,
        }
    }

    pub fn new_euclidean(nodes: Vec<Node>, start_point: Node, max_cost: f64) -> Self {
        let links = nodes
            .iter()
            .combinations(2)
            .map(|ns| Link::new(ns[0], ns[1]))
            .collect();
        Self::new(links, nodes, start_point, max_cost)
    }

    fn get_cost(&self, node1: usize, node2: usize) -> f64 {
        for link in self.links.iter() {
            if link.is_link(node1, Some(node2)) {
                return link.cost;
            }
        }
        return self.max_cost;
    }

    fn get_link(&self, node1: usize, node2: usize) -> Result<Link, &str> {
        for link in self.links.iter() {
            if link.is_link(node1, Some(node2)) {
                return Ok(*link);
            }
        }
        return Err("there is no link for the two nodes, should not happen");
    }

    // TODO check if I need the original copy of Node to track the change
    // I don't think it should because the only change to the node concerns the
    // fill level which should be updated at network level
    fn min_cost(&self, costs: &HashMap<usize, (f64, Link)>) -> usize {
        //let mut min_cost = self.max_cost;
        //let mut min_node = self.start_point.node_id;
        //for (node, cost) in costs.iter() {
        //    if cost.0 < min_cost {
        //        min_node = *node;
        //        min_cost = cost.0;
        //    }
        //}
        costs.iter().reduce(|a, b| if a.1.0 < b.1.0 {a} else {b}).unwrap().0.clone()
    }

    fn update_costs(
        &self,
        node: usize,
        nodes: &Vec<usize>,
        costs: HashMap<usize, (f64, Link)>,
    ) -> HashMap<usize, (f64, Link)> {
        let mut new_costs: HashMap<usize, (f64, Link)> = HashMap::new();
        for new_node in nodes.iter() {
            if *new_node != node {
                let cost: f64 = self.get_cost(*new_node, node);
                if cost < costs[new_node].0 {
                    let link = self.get_link(node, *new_node);
                    new_costs.insert(*new_node, (cost, link.unwrap()));
                } else {
                    new_costs.insert(*new_node, costs[new_node]);
                }
            }
        }
        return new_costs;
    }

    fn init_costs(&self, active_nodes: &Vec<usize>) -> HashMap<usize, (f64, Link)> {
        let mut costs: Vec<(f64, Link)> = Vec::new();
        for node in active_nodes.iter() {
            let link = self.get_link(*node, self.start_point.node_id).unwrap();
            let cost = self.get_cost(*node, self.start_point.node_id);
            costs.push((cost, link));
        }
        let mut costs_map = HashMap::new();
        for (node, cost) in active_nodes.iter().zip(costs.iter()) {
            costs_map.insert(*node, *cost);
        }
        return costs_map;
    }

    fn prims_mst(&mut self) -> (Vec<Link>, Vec<usize>) {
        let mut active_nodes: Vec<usize> = Vec::new();
        for (node_id, node) in self.nodes.iter() {
            active_nodes.push(*node_id);
        }
        let mut mst_links: Vec<Link> = Vec::new();
        let mut mst_nodes: Vec<usize> = Vec::new();
        let mut costs: HashMap<usize, (f64, Link)> = HashMap::new();
        while active_nodes.len() != 0 {
            if mst_nodes.len() == 0 {
                mst_nodes.push(self.start_point.node_id);
                active_nodes.retain(|node| *node != self.start_point.node_id);
                costs = self.init_costs(&active_nodes);
            } else {
                let min_cost_node = self.min_cost(&costs);
                let link = costs[&min_cost_node].1;
                mst_links.push(link);
                mst_nodes.push(link.nodes[0]);
                mst_nodes.push(link.nodes[1]);
                active_nodes.retain(|node| *node != min_cost_node);
                costs = self.update_costs(min_cost_node, &active_nodes, costs);
            }
        }
        return (mst_links, mst_nodes);
    }

    fn is_relaxed(&self, node: usize, relaxed_links: Vec<Link>) -> bool {
        let mut node_links: Vec<Link> = Vec::new();
        for link in relaxed_links.iter() {
            if link.nodes.contains(&node) {
                node_links.push(*link);
            }
        }
        if node_links.len() == 2 {
            return true;
        } else {
            return false;
        }
    }

    fn links_per_node(
        &self,
        links: &Vec<Link>,
        nodes: &Vec<usize>,
        node_rel: Option<usize>,
    ) -> HashMap<usize, Vec<Link>> {
        let mut links_by_node: HashMap<usize, Vec<Link>> = HashMap::new();
        for node in nodes.iter() {
            let mut all_links = links.clone();
            all_links.retain(|link| link.is_link(*node, None));
            links_by_node.insert(*node, all_links);
        }
        match node_rel {
            Some(node) => {
                let mut all_links = links.clone();
                all_links.retain(|link| link.is_link(node, None));
                links_by_node.insert(node, all_links);
            }
            _ => {}
        }
        return links_by_node;
    }

    fn get_odd(&self, links: &Vec<Link>, nodes: Vec<usize>) -> (Vec<usize>, Vec<Link>) {
        let mst_links_by_node: HashMap<usize, Vec<Link>> =
            self.links_per_node(&links, &nodes, None);
        let mut odd_nodes: Vec<usize> = Vec::new();
        for node in nodes.iter() {
            if mst_links_by_node[node].len() % 2 != 0 {
                odd_nodes.push(*node);
            }
        }
        let mut all_links_by_node: HashMap<usize, Vec<Link>> =
            self.links_per_node(&self.links, &odd_nodes, None);
        let mut all_odd_links: Vec<Link> = Vec::new();
        for (_, odd_links) in all_links_by_node.iter_mut() {
            all_odd_links.append(odd_links);
            all_odd_links.retain(|link| {
                odd_nodes.contains(&link.nodes[0]) && odd_nodes.contains(&link.nodes[1])
            });
        }
        return (odd_nodes, all_odd_links);
    }

    fn is_entered(&self, node: usize, relaxed_links: &Vec<Link>) -> bool {
        let mut node_links: Vec<Link> = Vec::new();
        for link in relaxed_links.iter() {
            if link.nodes.contains(&node) {
                node_links.push(*link);
            }
        }
        debug_assert!(node_links.len() <= 2);
        if node_links.len() == 1 {
            return true;
        } else {
            return false;
        }
    }

//    fn remove_links_relaxed(
//        &self,
//        node: usize,
//        relaxed_nodes: Vec<usize>,
//        remaining_links: &mut Vec<Link>,
//    ) {
//        let mut node_links = Vec::new();
//        for link in remaining_links.iter() {
//            if link.nodes.contains(&node) {
//                node_links.push(*link);
//            }
//        }
//        node_links.retain(|link| relaxed_nodes.contains(&link.other_node(node)));
//        remaining_links.retain(|link| !node_links.contains(link))
//    }
//
    fn is_closing_link(&self, link: Link, relaxed_links: Vec<Link>) -> bool {
        if self.is_entered(link.nodes[0], &relaxed_links)
            && self.is_entered(link.nodes[1], &relaxed_links)
        {
            return true;
        } else {
            return false;
        }
    }

    fn links_of_node(&self, node: usize, links: &Vec<Link>) -> Vec<Link> {
        let mut nodes_links = Vec::new();
        for link in links.iter() {
            if link.nodes.contains(&node) {
                nodes_links.push(*link);
            }
        }
        return nodes_links;
    }

    // this is probably not needed....
    fn is_dead_end(&self, links: Vec<Link>, start_point: usize, node: usize, link: Link) -> bool {
        let other = link.other_node(node);
        let start_node_links = self.links_of_node(start_point, &links);
        if start_node_links.len() == 1 {
            let final_link = start_node_links[1];
            let mut other_links = self.links_of_node(other, &links);
            other_links.retain(|other_link| *other_link != link);
            if other_links.len() == 1 && final_link == other_links[0] {
                return true;
            }
        }
        return false;
    }

    fn beginning_of_end(&self, start_point: usize, links: &Vec<Link>) -> (usize, Vec<Link>) {
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
                if let Some(pos) = available_links.iter().position(|x| *x == link) {
                    available_links.remove(pos);
                }
                boe = link.other_node(next_node);
                next_node = boe;
            }
        }
        return (boe, path);
    }

    fn euler_tour(&self, links_all: Vec<Link>) -> Vec<Link> {
        let mut links: Vec<Link> = Vec::new();
        for link in links_all.iter() {
            links.push(*link);
        }
        let start_node_links = self.links_of_node(self.start_point.node_id, &links);
        let first_link = start_node_links[0];
        let mut sorted_links: Vec<Link> = Vec::new();
        sorted_links.push(first_link);
        let mut last_node = self.start_point.node_id;
        let start_node = last_node;
        if let Some(pos) = links.iter().position(|x| *x == first_link) {
            links.remove(pos);
        }

        while links.len() != 0 {
            let (_, mut path_to_end) = self.beginning_of_end(start_node, &links);
            let other = sorted_links[sorted_links.len() - 1].other_node(last_node);
            let mut possible_links: Vec<Link> = Vec::new();
            for link in links.iter() {
                if link.nodes.contains(&other)
                    && !path_to_end.contains(link)
                {
                    possible_links.push(*link);
                }
            }
            // no possible links, close path
            if possible_links.len() == 0 {
                path_to_end.reverse();
                for last_link in path_to_end.iter() {
                    sorted_links.push(*last_link);
                }
                break;
            }
            let next_link = possible_links[0];
            sorted_links.push(next_link);
            if let Some(pos) = links.iter().position(|x| *x == next_link) {
                links.remove(pos);
            }
            last_node = other;
        }
        return sorted_links;
    }

    fn get_common_node(&self, link1: Link, link2: Link) -> Option<usize> {
        let node11 = link1.nodes[0];
        let node12 = link1.nodes[1];
        if link2.nodes.contains(&node11) {
            return Some(node11);
        } else if link2.nodes.contains(&node12) {
            return Some(node12);
        } else {
            return None;
        }
    }

    fn hamiltonian_cycle(&self, links: &mut Vec<Link>) -> Vec<Link> {
        if links.len() == 1 {
            let link = links[0];
            let mut there_and_back = Vec::new();
            there_and_back.push(link);
            there_and_back.push(link);
            return there_and_back;
        }
        let mut visited_nodes: Vec<usize> = Vec::new();
        let first_link = links[0];
        let second_link = links[1];
        let mut relaxed_links: Vec<Link> = Vec::new();
        relaxed_links.push(first_link);
        if let Some(pos) = links.iter().position(|x| *x == first_link) {
            links.remove(pos);
        }
        let mut prev_node = self.get_common_node(first_link, second_link).unwrap();
        let start_node = first_link.other_node(prev_node);
        visited_nodes.push(start_node);
        visited_nodes.push(prev_node);
        let mut needs_relaxation = false;
        let mut orphan = self.start_point.node_id;
        for link in links.iter() {
            let other = link.other_node(prev_node);
            if visited_nodes.contains(&other) {
                if !needs_relaxation {
                    orphan = prev_node;
                }
                prev_node = other;
                needs_relaxation = true;
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
        for (node, links) in by_node.iter() {
            if links.len() < 2 {
                incomplete_nodes.push(*node);
                debug_assert!(links.len() == 1);
            }
        }
        debug_assert!(incomplete_nodes.len() == 2);
        let closing_link = self
            .get_link(incomplete_nodes[0], incomplete_nodes[1])
            .unwrap();
        relaxed_links.push(closing_link);
        return relaxed_links;
    }

    pub fn christofides(&mut self) -> Vec<usize> {
        // prims to get mst
        let (mut mst_links, mst_nodes) = self.prims_mst();
        // get all vertices with odd number of connections
        let (nodes_odd, links_odd) = self.get_odd(&mst_links, self.nodes.keys().cloned().collect());
        // get mwpf
        let min_w_tup: Vec<(usize, usize)> = matching::mwmatching(&nodes_odd, &links_odd);
        let min_w: Vec<Link> = min_w_tup
            .into_iter()
            .map(|(node1, node2)| self.get_link(node1, node2).unwrap())
            .collect();
        // add nodes and links from mst nodes and links
        mst_links.extend(min_w);
        // get eulerian tour
        let mut eulerian = self.euler_tour(mst_links);
        let hamilton = self.hamiltonian_cycle(&mut eulerian);
        let mut final_ids: Vec<usize> = Vec::new();
        final_ids.push(self.start_point.node_id);
        let mut prev_node = self.start_point.node_id;
        for link in hamilton.iter() {
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
    fn test_min_cost() {
        let node_id: Vec<usize> = (1..=6).collect();
        let nodes = vec![
            Node::new(0.0, 1.0, 0, 0.7),
            Node::new(0.0, 1.0, 0, 0.7),
            Node::new(0.0, 1.0, 0, 0.7),
            Node::new(0.0, 1.0, 0, 0.7),
            Node::new(0.0, 1.0, 0, 0.7),
            Node::new(0.0, 1.0, 0, 0.7),
        ];
        let mut links: Vec<Link> = Vec::new();
        let link12 = Link {
            nodes: [0, 1],
            cost: 10.0,
        };
        let link13 = Link {
            nodes: [0, 2],
            cost: 15.0,
        };
        let link14 = Link {
            nodes: [0, 3],
            cost: 9.0,
        };
        let link15 = Link {
            nodes: [0, 4],
            cost: 5.0,
        };
        let link23 = Link {
            nodes: [2, 1],
            cost: 17.0,
        };
        let link24 = Link {
            nodes: [3, 1],
            cost: 10.0,
        };
        let link25 = Link {
            nodes: [4, 1],
            cost: 11.0,
        };
        let link34 = Link {
            nodes: [2, 3],
            cost: 1.0,
        };
        let link35 = Link {
            nodes: [4, 2],
            cost: 4.0,
        };
        let link45 = Link {
            nodes: [4, 3],
            cost: 20.0,
        };
        let link16 = Link {
            nodes: [0, 5],
            cost: 21.0,
        };
        let link26 = Link {
            nodes: [1, 5],
            cost: 15.0,
        };
        let link36 = Link {
            nodes: [2, 5],
            cost: 13.0,
        };
        let link46 = Link {
            nodes: [3, 5],
            cost: 2.0,
        };
        let link56 = Link {
            nodes: [4, 5],
            cost: 25.0,
        };
        links.push(link12);
        links.push(link13);
        links.push(link14);
        links.push(link15);
        links.push(link23);
        links.push(link24);
        links.push(link25);
        links.push(link34);
        links.push(link35);
        links.push(link45);
        links.push(link16);
        links.push(link26);
        links.push(link36);
        links.push(link46);
        links.push(link56);
        let costs = HashMap::from([(0, (5.0, link12)), (1, (4.0, link23)), (2, (6.0, link34))]);
        let nw = Network::new(links, nodes, Node::new(0.0, 0.0, 0, 0.8), 100.0);
        let min_node = nw.min_cost(&costs);
        assert_eq!(min_node, 1);
    }

    #[test]
    fn test_update_cost() {
        let nodes = HashMap::from([
            (1, Node::new(0.0, 1.0, 0, 0.7)),
            (2, Node::new(0.0, 1.0, 0, 0.7)),
            (3, Node::new(0.0, 1.0, 0, 0.7)),
            (4, Node::new(0.0, 1.0, 0, 0.7)),
            (5, Node::new(0.0, 1.0, 0, 0.7)),
            (6, Node::new(0.0, 1.0, 0, 0.7)),
        ]);
        let mut links: Vec<Link> = Vec::new();
        let link12 = Link {
            nodes: [1, 2],
            cost: 10.0,
        };
        let link13 = Link {
            nodes: [1, 3],
            cost: 15.0,
        };
        let link14 = Link {
            nodes: [1, 4],
            cost: 9.0,
        };
        let link15 = Link {
            nodes: [1, 5],
            cost: 5.0,
        };
        let link23 = Link {
            nodes: [3, 2],
            cost: 17.0,
        };
        let link24 = Link {
            nodes: [4, 2],
            cost: 10.0,
        };
        let link25 = Link {
            nodes: [5, 2],
            cost: 11.0,
        };
        let link34 = Link {
            nodes: [3, 4],
            cost: 1.0,
        };
        let link35 = Link {
            nodes: [5, 3],
            cost: 4.0,
        };
        let link45 = Link {
            nodes: [5, 4],
            cost: 20.0,
        };
        let link16 = Link {
            nodes: [1, 6],
            cost: 21.0,
        };
        let link26 = Link {
            nodes: [2, 6],
            cost: 15.0,
        };
        let link36 = Link {
            nodes: [3, 6],
            cost: 13.0,
        };
        let link46 = Link {
            nodes: [4, 6],
            cost: 2.0,
        };
        let link56 = Link {
            nodes: [5, 6],
            cost: 25.0,
        };
        links.push(link12);
        links.push(link13);
        links.push(link14);
        links.push(link15);
        links.push(link23);
        links.push(link24);
        links.push(link25);
        links.push(link34);
        links.push(link35);
        links.push(link45);
        links.push(link16);
        links.push(link26);
        links.push(link36);
        links.push(link46);
        links.push(link56);
        let costs = HashMap::from([(1, (5.0, link12)), (2, (4.0, link23)), (3, (6.0, link34))]);
        let mut new_nodes: Vec<usize> = Vec::new();
        new_nodes.push(1);
        new_nodes.push(2);
        new_nodes.push(3);
        let nw = Network {
            nodes,
            links,
            start_point: Node::new(0.0, 0.0, 0, 0.8),
            max_cost: 100.0,
        };
        let new_costs = nw.update_costs(4, &new_nodes, costs);
        let min_node = nw.min_cost(&new_costs);
        assert_eq!(min_node, 3)
    }

    #[test]
    fn test_get_odd() {
        let nodes = HashMap::from([
            (1, Node::new(0.0, 1.0, 0, 0.7)),
            (2, Node::new(0.0, 1.0, 0, 0.7)),
            (3, Node::new(0.0, 1.0, 0, 0.7)),
            (4, Node::new(0.0, 1.0, 0, 0.7)),
            (5, Node::new(0.0, 1.0, 0, 0.7)),
            (6, Node::new(0.0, 1.0, 0, 0.7)),
        ]);
        let mut links = Vec::new();
        links.push(Link {
            nodes: [1, 2],
            cost: 1.0,
        });
        links.push(Link {
            nodes: [1, 3],
            cost: 2.0,
        });
        links.push(Link {
            nodes: [1, 4],
            cost: 3.0,
        });
        links.push(Link {
            nodes: [4, 5],
            cost: 4.0,
        });
        links.push(Link {
            nodes: [3, 4],
            cost: 5.0,
        });
        let nodes_id = (1..6).collect();
        let mut links_2 = Vec::new();
        links_2.push(Link {
            nodes: [1, 2],
            cost: 1.0,
        });
        links_2.push(Link {
            nodes: [1, 3],
            cost: 2.0,
        });
        links_2.push(Link {
            nodes: [1, 4],
            cost: 3.0,
        });
        links_2.push(Link {
            nodes: [4, 5],
            cost: 4.0,
        });
        links_2.push(Link {
            nodes: [3, 4],
            cost: 5.0,
        });
        let nw = Network {
            nodes,
            links: links_2,
            start_point: Node::new(0.0, 0.0, 0, 0.8),
            max_cost: 100.0,
        };
        let (odd_nodes, odd_links) = nw.get_odd(&links, nodes_id);
        assert_eq!(odd_nodes.len(), 4);
        assert!(odd_links.contains(&Link {
            nodes: [1, 2],
            cost: 1.0
        }));
        assert!(odd_links.contains(&Link {
            nodes: [1, 4],
            cost: 3.0
        }));
        assert!(odd_links.contains(&Link {
            nodes: [4, 5],
            cost: 4.0
        }));
        assert!(!odd_links.contains(&Link {
            nodes: [3, 4],
            cost: 5.0
        }));
        assert!(!odd_links.contains(&Link {
            nodes: [1, 3],
            cost: 2.0
        }));
        assert!(!odd_nodes.contains(&3));
    }

    #[test]
    fn test_mst() {
        let nodes = HashMap::from([
            (1, Node::new(0.0, 1.0, 0, 0.7)),
            (2, Node::new(0.0, 1.0, 0, 0.7)),
            (3, Node::new(0.0, 1.0, 0, 0.7)),
            (4, Node::new(0.0, 1.0, 0, 0.7)),
            (5, Node::new(0.0, 1.0, 0, 0.7)),
        ]);
        let mut links: Vec<Link> = Vec::new();
        let link12 = Link {
            nodes: [1, 2],
            cost: 3.0,
        };
        let link13 = Link {
            nodes: [1, 3],
            cost: 1.0,
        };
        let link14 = Link {
            nodes: [1, 4],
            cost: 8.0,
        };
        let link15 = Link {
            nodes: [1, 5],
            cost: 9.0,
        };
        let link23 = Link {
            nodes: [3, 2],
            cost: 5.0,
        };
        let link24 = Link {
            nodes: [4, 2],
            cost: 7.0,
        };
        let link25 = Link {
            nodes: [5, 2],
            cost: 6.0,
        };
        let link34 = Link {
            nodes: [3, 4],
            cost: 1.0,
        };
        let link35 = Link {
            nodes: [5, 3],
            cost: 2.0,
        };
        let link45 = Link {
            nodes: [5, 4],
            cost: 3.0,
        };
        links.push(link12);
        links.push(link13);
        links.push(link14);
        links.push(link15);
        links.push(link23);
        links.push(link24);
        links.push(link25);
        links.push(link34);
        links.push(link35);
        links.push(link45);
        let mut nw = Network {
            nodes,
            links,
            start_point: Node::new(0.0, 0.0, 1, 0.8),
            max_cost: 100.0,
        };
        let (mst_links, mst_nodes) = nw.prims_mst();
        assert!(mst_links.contains(&link12));
    }

    #[test]
    fn test_euler() {
        let nodes = HashMap::from([
            (1, Node::new(0.0, 1.0, 0, 0.7)),
            (2, Node::new(0.0, 1.0, 0, 0.7)),
            (3, Node::new(0.0, 1.0, 0, 0.7)),
            (4, Node::new(0.0, 1.0, 0, 0.7)),
            (5, Node::new(0.0, 1.0, 0, 0.7)),
        ]);
        let mut links: Vec<Link> = Vec::new();
        let link12 = Link {
            nodes: [1, 2],
            cost: 3.0,
        };
        let link13 = Link {
            nodes: [1, 3],
            cost: 1.0,
        };
        let link14 = Link {
            nodes: [1, 4],
            cost: 8.0,
        };
        let link15 = Link {
            nodes: [1, 5],
            cost: 9.0,
        };
        let link23 = Link {
            nodes: [3, 2],
            cost: 5.0,
        };
        let link24 = Link {
            nodes: [4, 2],
            cost: 7.0,
        };
        let link25 = Link {
            nodes: [5, 2],
            cost: 6.0,
        };
        let link34 = Link {
            nodes: [3, 4],
            cost: 1.0,
        };
        let link35 = Link {
            nodes: [5, 3],
            cost: 2.0,
        };
        let link45 = Link {
            nodes: [5, 4],
            cost: 3.0,
        };
        links.push(link12);
        links.push(link13);
        links.push(link14);
        links.push(link15);
        links.push(link23);
        links.push(link24);
        links.push(link25);
        links.push(link34);
        links.push(link35);
        links.push(link45);
        let mut nw = Network {
            nodes,
            links,
            start_point: Node::new(0.0, 0.0, 1, 0.8),
            max_cost: 100.0,
        };
        let mut all_links = Vec::new();
        all_links.push(link12);
        all_links.push(link13);
        all_links.push(link14);
        all_links.push(link15);
        all_links.push(link23);
        all_links.push(link45);
        let euler_tour = nw.euler_tour(all_links);
        assert_eq!(euler_tour.len(), 6)
    }

    #[test]
    fn test_hamilton() {
        let nodes = HashMap::from([
            (1, Node::new(0.0, 1.0, 0, 0.7)),
            (2, Node::new(0.0, 1.0, 0, 0.7)),
            (3, Node::new(0.0, 1.0, 0, 0.7)),
            (4, Node::new(0.0, 1.0, 0, 0.7)),
            (5, Node::new(0.0, 1.0, 0, 0.7)),
        ]);
        let mut links: Vec<Link> = Vec::new();
        let link12 = Link {
            nodes: [1, 2],
            cost: 3.0,
        };
        let link13 = Link {
            nodes: [1, 3],
            cost: 1.0,
        };
        let link14 = Link {
            nodes: [1, 4],
            cost: 8.0,
        };
        let link15 = Link {
            nodes: [1, 5],
            cost: 9.0,
        };
        let link23 = Link {
            nodes: [3, 2],
            cost: 5.0,
        };
        let link24 = Link {
            nodes: [4, 2],
            cost: 7.0,
        };
        let link25 = Link {
            nodes: [5, 2],
            cost: 6.0,
        };
        let link34 = Link {
            nodes: [3, 4],
            cost: 1.0,
        };
        let link35 = Link {
            nodes: [5, 3],
            cost: 2.0,
        };
        let link45 = Link {
            nodes: [5, 4],
            cost: 3.0,
        };
        links.push(link12);
        links.push(link13);
        links.push(link14);
        links.push(link15);
        links.push(link23);
        links.push(link24);
        links.push(link25);
        links.push(link34);
        links.push(link35);
        links.push(link45);
        let mut nw = Network {
            nodes,
            links,
            start_point: Node::new(0.0, 0.0, 1, 0.8),
            max_cost: 100.0,
        };
        let mut all_links = Vec::new();
        all_links.push(link12);
        all_links.push(link13);
        all_links.push(link14);
        all_links.push(link15);
        all_links.push(link23);
        all_links.push(link45);
        let mut euler_tour = nw.euler_tour(all_links);
        let hamiltonian_cycle = nw.hamiltonian_cycle(&mut euler_tour);
        assert_eq!(hamiltonian_cycle.len(), 5);
    }

    #[test]
    fn test_hamilton_2() {
        let nodes = HashMap::from([
            (1, Node::new(0.0, 1.0, 0, 0.7)),
            (2, Node::new(0.0, 1.0, 0, 0.7)),
            (3, Node::new(0.0, 1.0, 0, 0.7)),
            (4, Node::new(0.0, 1.0, 0, 0.7)),
            (5, Node::new(0.0, 1.0, 0, 0.7)),
            (6, Node::new(0.0, 1.0, 0, 0.7)),
        ]);
        let mut links: Vec<Link> = Vec::new();
        let link12 = Link {
            nodes: [1, 2],
            cost: 3.0,
        };
        let link13 = Link {
            nodes: [1, 3],
            cost: 1.0,
        };
        let link14 = Link {
            nodes: [1, 4],
            cost: 8.0,
        };
        let link15 = Link {
            nodes: [1, 5],
            cost: 9.0,
        };
        let link23 = Link {
            nodes: [3, 2],
            cost: 5.0,
        };
        let link24 = Link {
            nodes: [4, 2],
            cost: 7.0,
        };
        let link25 = Link {
            nodes: [5, 2],
            cost: 6.0,
        };
        let link34 = Link {
            nodes: [3, 4],
            cost: 1.0,
        };
        let link35 = Link {
            nodes: [5, 3],
            cost: 20.0,
        };
        let link45 = Link {
            nodes: [5, 4],
            cost: 5.0,
        };
        let link16 = Link {
            nodes: [1, 6],
            cost: 1.0,
        };
        let link26 = Link {
            nodes: [2, 6],
            cost: 2.0,
        };
        let link36 = Link {
            nodes: [3, 6],
            cost: 3.0,
        };
        let link46 = Link {
            nodes: [4, 6],
            cost: 10.0,
        };
        let link56 = Link {
            nodes: [5, 6],
            cost: 8.0,
        };
        links.push(link12);
        links.push(link13);
        links.push(link14);
        links.push(link15);
        links.push(link23);
        links.push(link24);
        links.push(link25);
        links.push(link34);
        links.push(link35);
        links.push(link45);
        links.push(link16);
        links.push(link26);
        links.push(link36);
        links.push(link46);
        links.push(link56);
        let mut all_links = Vec::new();
        all_links.push(link12);
        all_links.push(link13);
        all_links.push(link14);
        all_links.push(link15);
        all_links.push(link23);
        all_links.push(link45);
        all_links.push(link24);
        all_links.push(link46);
        all_links.push(link26);
        let mut nw = Network {
            nodes,
            links,
            start_point: Node::new(0.0, 0.0, 1, 0.8),
            max_cost: 100.0,
        };
        let mut euler_tour = nw.euler_tour(all_links);
        let hamiltonian_cycle = nw.hamiltonian_cycle(&mut euler_tour);
        let nodes_ids: Vec<usize> = (1..7).collect();
        let by_node = nw.links_per_node(&hamiltonian_cycle, &nodes_ids, None);
    }

    #[test]
    fn test_christofides() {
        let nodes = HashMap::from([
            (1, Node::new(0.0, 1.0, 0, 0.7)),
            (2, Node::new(0.0, 1.0, 0, 0.7)),
            (3, Node::new(0.0, 1.0, 0, 0.7)),
            (4, Node::new(0.0, 1.0, 0, 0.7)),
            (5, Node::new(0.0, 1.0, 0, 0.7)),
        ]);
        let mut links: Vec<Link> = Vec::new();
        let link12 = Link {
            nodes: [1, 2],
            cost: 3.0,
        };
        let link13 = Link {
            nodes: [1, 3],
            cost: 4.0,
        };
        let link14 = Link {
            nodes: [1, 4],
            cost: 2.0,
        };
        let link15 = Link {
            nodes: [1, 5],
            cost: 5.0,
        };
        let link23 = Link {
            nodes: [3, 2],
            cost: 5.0,
        };
        let link24 = Link {
            nodes: [4, 2],
            cost: 7.0,
        };
        let link25 = Link {
            nodes: [5, 2],
            cost: 15.0,
        };
        let link34 = Link {
            nodes: [3, 4],
            cost: 9.0,
        };
        let link35 = Link {
            nodes: [5, 3],
            cost: 10.0,
        };
        let link45 = Link {
            nodes: [5, 4],
            cost: 3.0,
        };
        links.push(link12);
        links.push(link13);
        links.push(link14);
        links.push(link15);
        links.push(link23);
        links.push(link24);
        links.push(link25);
        links.push(link34);
        links.push(link35);
        links.push(link45); let mut nw = Network {
            nodes,
            links,
            start_point: Node::new(0.0, 0.0, 1, 0.8),
            max_cost: 100.0,
        };
        let mut hamilton = nw.christofides();
        assert!(hamilton.contains(&1))
    }

    #[test]
    fn test_critical(){
        let nodes: Vec<Node> = vec![Node { x_coord: 0.0, y_coord: 1.0, node_id: 1 }, Node { x_coord: 1.0, y_coord: 0.0, node_id: 2 }, Node { x_coord: 1.0, y_coord: 1.0, node_id: 3 }, Node { x_coord: 23.0, y_coord: 0.0, node_id: 4 }, Node { x_coord: 765.0, y_coord: 234.0, node_id: 5 }, Node { x_coord: 2.0, y_coord: 23.0, node_id: 6 }, Node { x_coord: -0.172685, y_coord: 51.497667, node_id: 7 }];
        let start_node = Node { x_coord: -0.172685, y_coord: 51.497667,node_id:7};
        let mut nw = Network::new_euclidean(nodes, start_node, 999999.0);
        let christofides = nw.christofides();
        println!("{christofides:?}");
    }

    #[test]
    fn test_critica2(){
        let nodes: Vec<Node> = vec![Node { x_coord: 0.0, y_coord: 1.0, node_id: 1 }, Node { x_coord: 1.0, y_coord: 0.0, node_id: 2 }, Node { x_coord: 11.0, y_coord: 1.0, node_id: 3 }, Node { x_coord: 13.0, y_coord: 23.0, node_id: 4 }, Node { x_coord: 12.0, y_coord: 0.0, node_id: 5 }, Node { x_coord: -0.172685, y_coord: 51.497667, node_id: 6 }];
        let start_node = Node { x_coord: -0.172685, y_coord: 51.497667, node_id: 6 };
        let mut nw = Network::new_euclidean(nodes, start_node, 99999999.0);
        let christofides = nw.christofides();
        assert_eq!(christofides.len(), 7);
        println!("{christofides:?}");
    } 

    #[test]
    fn test_two_nodes(){
        let nodes: Vec<Node> = vec![Node { x_coord: 0.0, y_coord: 1.0, node_id: 1 }, Node { x_coord: 1.0, y_coord: 0.0, node_id: 2 }];
        let start_node = Node { x_coord: 0.0, y_coord: 1.0, node_id: 1 };
        let mut nw = Network::new_euclidean(nodes, start_node, 99999999.0);
        let christofides = nw.christofides();
        assert_eq!(christofides.len(), 3);
        println!("{christofides:?}");
    }

    #[test]
    fn test_six_nodes(){
        let nodes = vec![Node { x_coord: 0.0, y_coord: 2.213, node_id: 1 }, Node { x_coord: 43.0, y_coord: 3.324, node_id: 2 }, Node { x_coord: 3.3, y_coord: 434.43, node_id: 3 }, Node { x_coord: 43.242, y_coord: 43.432, node_id: 4 }, Node { x_coord: 0.0, y_coord: 24.23, node_id: 5 }, Node { x_coord: 32.4238, y_coord: 24.324, node_id: 6 }, Node { x_coord: -0.172685, y_coord: 51.497667,node_id:7}];
        let start_node = Node { x_coord: 0.0, y_coord: 2.213, node_id: 1 };
        let mut nw = Network::new_euclidean(nodes, start_node, 99999999.0);
        let christofides = nw.christofides();
        assert_eq!(christofides.len(), 8);
        println!("{christofides:?}");
    }
}