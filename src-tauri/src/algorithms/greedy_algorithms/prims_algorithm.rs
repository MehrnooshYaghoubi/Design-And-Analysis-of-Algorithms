use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use crate::types::Edge;

pub fn prim(n: usize, edges: Vec<Edge>) -> Vec<Edge> {
    // Build adjacency list from edge list
    let mut adj = vec![vec![]; n];
    for edge in edges {
        adj[edge.src].push((edge.dest, edge.weight));
        adj[edge.dest].push((edge.src, edge.weight)); // undirected graph
    }

    let mut mst = Vec::new();
    let mut visited = HashSet::new();
    let mut heap = BinaryHeap::new();

    visited.insert(0);
    for &(neighbor, weight) in &adj[0] {
        heap.push(Reverse((weight, 0, neighbor)));
    }

    while let Some(Reverse((weight, u, v))) = heap.pop() {
        if visited.contains(&v) {
            continue;
        }

        visited.insert(v);
        mst.push(Edge {
            src: u,
            dest: v,
            weight,
        });

        for &(neighbor, w) in &adj[v] {
            if !visited.contains(&neighbor) {
                heap.push(Reverse((w, v, neighbor)));
            }
        }
    }

    mst
}
