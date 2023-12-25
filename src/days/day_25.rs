use std::cmp::Reverse;

use crate::utils::*;

/// An undirected weighted graph
#[derive(Clone)]
struct Graph {
    edges: Vec<Vec<u32>>,
    nodes: Vec<usize>,
}
impl Graph {
    fn new(n: usize) -> Self {
        Self {
            edges: vec![vec![0; n]; n],
            nodes: vec![1; n],
        }
    }

    fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn edges_of(&self, node: usize) -> impl Iterator<Item = (usize, u32)> + '_ {
        self.edges[node]
            .iter()
            .enumerate()
            .filter_map(|(i, &w)| if w > 0 { Some((i, w)) } else { None })
    }

    fn add_edge(&mut self, a: usize, b: usize, weight: u32) {
        self.edges[a][b] = weight;
        self.edges[b][a] = weight;
    }

    fn swap_nodes(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        self.nodes.swap(a, b);
        self.edges.swap(a, b);
        self.edges.iter_mut().for_each(|e| e.swap(a, b));
    }
    fn merge_nodes(&mut self, a: usize, b: usize) {
        assert_ne!(a, b);
        self.nodes[a] += self.nodes[b];
        for i in 0..self.num_nodes() {
            self.edges[a][i] += self.edges[b][i];
            self.edges[i][a] += self.edges[i][b];
        }
        self.edges[a][a] = 0;
        let last = self.num_nodes() - 1;
        self.swap_nodes(b, last);
        self.nodes.pop();
        self.edges.pop();
        self.edges.iter_mut().for_each(|e| {
            e.pop();
        });
    }
}
impl std::ops::Index<(usize, usize)> for Graph {
    type Output = u32;
    fn index(&self, edge: (usize, usize)) -> &Self::Output {
        &self.edges[edge.0][edge.1]
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/25.txt");

    let mut connections = HashMap::new();
    for l in input.lines() {
        let (from, to) = l.split_once(": ").unwrap();
        let to = to.split_whitespace().to_vec();
        let out = connections.entry(from).or_insert_with(HashSet::<&str>::new);
        out.extend(&to);
        for to in &to {
            connections.entry(to).or_default().insert(from);
        }
    }

    let mut keys = connections.keys().copied().to_vec();
    keys.sort_unstable();
    let key_map = keys.iter().enumerate().map(|(i, k)| (*k, i)).to_map();
    let n = keys.len();

    let mut graph = Graph::new(n);
    for (from, to) in connections {
        let from = key_map[from];
        for to in to {
            let to = key_map[to];
            graph.add_edge(from, to, 1);
        }
    }

    while graph.num_nodes() > 2 {
        for selected in 0..graph.num_nodes() {
            // move selected to the front
            graph.swap_nodes(0, selected);

            let mut g = graph.clone();
            let mut original_index = (0..g.num_nodes()).to_vec();
            let mut last_merged = g.num_nodes();
            while g.num_nodes() > 2 {
                let (next, w) = g.edges_of(0).max_by_key(|(_, w)| *w).unwrap();
                g.merge_nodes(0, next);
                last_merged = original_index.swap_remove(next);
            }
            let cost = g[(0, 1)];
            let s = last_merged;
            let t = original_index[1];
            if cost == 3 {
                // critical edge, has to stay
                continue;
            }
            graph.merge_nodes(s, t);
            break;
        }
    }
    let result = graph.nodes[0] * graph.nodes[1];
    pv!(result);
}
