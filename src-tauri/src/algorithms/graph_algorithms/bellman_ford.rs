//! Bellman-Ford single-source shortest paths.
//!
//! Rewritten to take `i64` edge weights. The previous version routed through
//! `GraphInput`, whose `weight` is a `usize`, so negative weights could not
//! even be deserialised -- in the one algorithm whose whole purpose is
//! handling them. It also computed `vertex_count - 1`, which underflows to
//! `usize::MAX` on an empty graph.
//!
//! Unreachable vertices are `None` rather than a sentinel, so no caller can
//! accidentally add two "infinities" together.

use serde::Serialize;

use crate::types::SignedEdge;

#[derive(Debug, Clone, Serialize)]
pub struct BellmanFordResult {
    pub vertex_count: usize,
    pub start: usize,
    /// `dist[v]` is the shortest distance to `v`, or `None` if unreachable.
    pub dist: Vec<Option<i64>>,
    /// Predecessor of each vertex on a shortest path, for path rebuilding.
    pub predecessor: Vec<Option<usize>>,
    /// Relaxation rounds actually performed before the distances settled.
    pub rounds: usize,
    /// True when a negative-weight cycle is reachable from the start.
    pub has_negative_cycle: bool,
}

pub fn bellman_ford(
    vertex_count: usize,
    start: usize,
    edges: &[SignedEdge],
) -> Result<BellmanFordResult, String> {
    if vertex_count == 0 {
        return Err("Graph must have at least one vertex".into());
    }
    if start >= vertex_count {
        return Err(format!(
            "Start vertex {start} is outside 0..{vertex_count}"
        ));
    }
    for e in edges {
        if e.from >= vertex_count || e.to >= vertex_count {
            return Err(format!(
                "Edge {} -> {} refers to a vertex outside 0..{vertex_count}",
                e.from, e.to
            ));
        }
    }

    let mut dist: Vec<Option<i64>> = vec![None; vertex_count];
    let mut predecessor: Vec<Option<usize>> = vec![None; vertex_count];
    dist[start] = Some(0);

    // At most V-1 rounds are needed; `saturating_sub` keeps a 1-vertex graph
    // from underflowing.
    let mut rounds = 0usize;
    for _ in 0..vertex_count.saturating_sub(1) {
        let mut updated = false;
        for e in edges {
            let Some(d_from) = dist[e.from] else { continue };
            let candidate = d_from.saturating_add(e.weight);
            if dist[e.to].map_or(true, |d_to| candidate < d_to) {
                dist[e.to] = Some(candidate);
                predecessor[e.to] = Some(e.from);
                updated = true;
            }
        }
        rounds += 1;
        if !updated {
            break; // settled early
        }
    }

    // One more pass: any further improvement proves a negative cycle.
    let mut has_negative_cycle = false;
    for e in edges {
        let Some(d_from) = dist[e.from] else { continue };
        if dist[e.to].map_or(true, |d_to| d_from.saturating_add(e.weight) < d_to) {
            has_negative_cycle = true;
            break;
        }
    }

    Ok(BellmanFordResult {
        vertex_count,
        start,
        dist,
        predecessor,
        rounds,
        has_negative_cycle,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(from: usize, to: usize, weight: i64) -> SignedEdge {
        SignedEdge { from, to, weight }
    }

    #[test]
    fn handles_the_documented_example() {
        // This is the exact instructions.json payload, which the previous
        // usize-weighted version could not even parse.
        let edges = [
            e(0, 1, 6),
            e(0, 2, 7),
            e(1, 2, 8),
            e(1, 3, 5),
            e(1, 4, -4),
            e(2, 3, -3),
        ];
        let r = bellman_ford(5, 0, &edges).unwrap();
        assert_eq!(r.dist[0], Some(0));
        assert_eq!(r.dist[1], Some(6));
        assert_eq!(r.dist[2], Some(7));
        assert_eq!(r.dist[3], Some(4)); // 0 -> 2 -> 3 = 7 - 3
        assert_eq!(r.dist[4], Some(2)); // 0 -> 1 -> 4 = 6 - 4
        assert!(!r.has_negative_cycle);
    }

    #[test]
    fn positive_weights_only() {
        let edges = [e(0, 1, 4), e(0, 2, 1), e(2, 1, 2), e(1, 3, 1)];
        let r = bellman_ford(4, 0, &edges).unwrap();
        assert_eq!(r.dist[1], Some(3)); // via 2
        assert_eq!(r.dist[3], Some(4));
    }

    #[test]
    fn unreachable_is_none() {
        let r = bellman_ford(3, 0, &[e(0, 1, 5)]).unwrap();
        assert_eq!(r.dist[1], Some(5));
        assert_eq!(r.dist[2], None);
    }

    #[test]
    fn detects_negative_cycle() {
        let edges = [e(0, 1, 1), e(1, 2, -1), e(2, 1, -1)];
        let r = bellman_ford(3, 0, &edges).unwrap();
        assert!(r.has_negative_cycle);
    }

    #[test]
    fn negative_cycle_not_reachable_from_start_is_ignored() {
        // The 2 <-> 3 negative cycle cannot be reached from vertex 0.
        let edges = [e(0, 1, 1), e(2, 3, -5), e(3, 2, -5)];
        let r = bellman_ford(4, 0, &edges).unwrap();
        assert!(!r.has_negative_cycle);
        assert_eq!(r.dist[1], Some(1));
        assert_eq!(r.dist[2], None);
    }

    #[test]
    fn predecessors_rebuild_a_path() {
        let edges = [e(0, 1, 1), e(1, 2, 2), e(2, 3, 3)];
        let r = bellman_ford(4, 0, &edges).unwrap();
        let mut path = vec![3];
        while let Some(p) = r.predecessor[*path.last().unwrap()] {
            path.push(p);
        }
        path.reverse();
        assert_eq!(path, vec![0, 1, 2, 3]);
    }

    #[test]
    fn single_vertex_does_not_underflow() {
        // vertex_count - 1 == 0 here; the old code computed usize::MAX.
        let r = bellman_ford(1, 0, &[]).unwrap();
        assert_eq!(r.dist, vec![Some(0)]);
        assert_eq!(r.rounds, 0);
    }

    #[test]
    fn rejects_bad_input() {
        assert!(bellman_ford(0, 0, &[]).is_err());
        assert!(bellman_ford(3, 5, &[]).is_err());
        assert!(bellman_ford(3, 0, &[e(0, 9, 1)]).is_err());
    }

    #[test]
    fn stops_early_when_settled() {
        let r = bellman_ford(5, 0, &[e(0, 1, 1)]).unwrap();
        assert!(r.rounds < 4, "should settle before V-1 rounds");
    }
}
