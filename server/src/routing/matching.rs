use super::link::Link;
use bimap::BiMap;
use mwmatching::*;
use std::collections::HashMap;

pub fn mwmatching(nodes: &Vec<usize>, links: &Vec<Link>) -> Vec<(usize, usize)> {
    let mut map = BiMap::new();
    nodes.iter().enumerate().for_each(|(i, node)| {
        map.insert(node, i);
    });

    let edges = links
        .iter()
        .map(|link| {
            (
                map.get_by_left(&link.nodes[0]).unwrap().clone(),
                map.get_by_left(&link.nodes[1]).unwrap().clone(),
                -link.cost as i32,
            )
        })
        .collect();
    let connections = Matching::new(edges).max_cardinality().solve();
    let mut edgemap = HashMap::new();
    for i in 0..connections.len() {
        let (less, more) = if i < connections[i] {
            (i, connections[i])
        } else {
            (connections[i], i)
        };
        if !edgemap.contains_key(&less) {
            edgemap.insert(less, more);
        }
    }
    edgemap
        .iter()
        .map(|(a, b)| {
            (
                *map.get_by_right(a).unwrap().clone(),
                *map.get_by_right(b).unwrap().clone(),
            )
        })
        .collect::<Vec<(usize, usize)>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testmwmatchign() {
        let n1 = Link {
            nodes: [123, 232],
            cost: 1.0,
        };
        let n2 = Link {
            nodes: [232, 323],
            cost: 10.9,
        };
        let n3 = Link {
            nodes: [323, 4345],
            cost: 1.0,
        };
        let n4 = Link {
            nodes: [4345, 123],
            cost: 10.0,
        };
        let links = vec![n1, n2, n3, n4];
        let mut matches = mwmatching(&vec![123, 232, 323, 4345], &links);

        let mut ans = vec![(123, 232), (323, 4345)];
        ans.sort();
        matches.sort();
        assert_eq!(matches, ans);
    }
}
