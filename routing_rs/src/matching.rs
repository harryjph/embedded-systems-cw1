use mwmatching::*;
use crate::link::Link;
use std::collections::HashMap;

fn link_to_edge(link: &Link) -> Edge {
    (link.nodes[0] as usize, link.nodes[1] as usize, -link.cost as i32)
}

pub fn mwmatching(links: &Vec<Link>) -> Vec<(u32, u32)> {
    let edges = links.iter().map(link_to_edge).collect();
    let connections = Matching::new(edges).solve();
    let mut edgemap = HashMap::new();
    for i in 0..connections.len() {
        let (less, more) =  if i <  connections[i] { (i, connections[i]) } else { (connections[i], i) };
        if !edgemap.contains_key(&(less as u32)) {
            edgemap.insert(less as u32, more as u32);
        }
    }
    edgemap.iter().map(|(a, b)| (a.clone(), b.clone())).collect::<Vec<(u32, u32)>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testmwmatchign() {
        let n1 = Link{nodes: [0, 1], cost: 1.0};
        let n2 = Link{nodes: [1, 2], cost: 10.9};
        let n3 = Link{nodes: [2, 3], cost: 1.0};
        let n4 = Link{nodes: [3, 0], cost: 10.0};
        let links = vec![n1, n2, n3, n4];
        let matches = mwmatching(&links);
        assert_eq!(matches[0], (0, 1));
        assert_eq!(matches[1], (2, 3));
    }
}
