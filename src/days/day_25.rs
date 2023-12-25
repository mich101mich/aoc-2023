use std::cmp::Reverse;

use crate::utils::*;

/// An undirected weighted graph
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

    let mut combined = vec![false; n];
    let mut combined_weights = vec![0; n];
    let mut queue = BinaryHeap::new(); // (weight, node). Maximized by weight
    while graph.num_nodes() > 2 {
        let mut num_remaining = graph.num_nodes();
        combined.fill(false);
        combined_weights.fill(0);
        queue.clear();
        queue.push((0u32, 0usize));
        let combine_target = loop {
            let (_, next) = queue.pop().unwrap();
            if combined[next] {
                continue; // already processed
            }
            combined[next] = true;
            num_remaining -= 1;
            if num_remaining == 1 {
                break next;
            }
            for (other, w) in graph.edges_of(next).filter(|(o, _)| !combined[*o]) {
                combined_weights[other] += w;
                queue.push((combined_weights[other], other));
            }
        };

        let last = combined.iter().position(|&c| !c).unwrap();
        let cost = graph.edges_of(last).map(|(_, w)| w).sum::<u32>();
        if cost == 3 {
            let a = graph.nodes[last];
            let b = n - a;
            let result = a * b;
            pv!(result);
            break;
        }
        graph.merge_nodes(combine_target, last);
        combined.pop();
        combined_weights.pop();
    }
}
