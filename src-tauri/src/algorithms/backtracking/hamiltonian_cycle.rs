//! Hamiltonian cycle via backtracking on an adjacency matrix.
//!
//! The path is grown one vertex at a time from vertex 0. Two prunes keep the
//! search tractable: a candidate must be adjacent to the current endpoint and
//! unvisited, and once the path is complete it must close back to the start.

use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Serialize)]
pub struct HamiltonianResult {
    pub vertex_count: usize,
    pub found: bool,
    pub cancelled: bool,
    /// Backtracking steps taken.
    pub steps: usize,
    /// The cycle as vertices in visit order, returning to the start.
    /// Empty when no cycle exists.
    pub cycle: Vec<usize>,
}

struct Search<'a> {
    n: usize,
    adj: Vec<Vec<bool>>,
    path: Vec<usize>,
    visited: Vec<bool>,
    steps: usize,
    cancelled: bool,
    cancel: &'a AtomicBool,
}

impl<'a> Search<'a> {
    fn solve(&mut self, pos: usize) -> bool {
        if self.cancel.load(Ordering::Relaxed) {
            self.cancelled = true;
            return false;
        }

        if pos == self.n {
            // Every vertex is on the path; the cycle closes only if the last
            // vertex links back to the first.
            return self.adj[self.path[pos - 1]][self.path[0]];
        }

        let last = self.path[pos - 1];
        for v in 0..self.n {
            if self.visited[v] || !self.adj[last][v] {
                continue;
            }

            self.path[pos] = v;
            self.visited[v] = true;
            self.steps += 1;

            if self.solve(pos + 1) {
                return true;
            }
            if self.cancelled {
                return false;
            }

            self.visited[v] = false;
        }

        false
    }
}

/// `edges` is a list of undirected `(u, v)` pairs over `n` vertices.
pub fn hamiltonian_cycle(
    n: usize,
    edges: &[(usize, usize)],
    cancel: &AtomicBool,
) -> Result<HamiltonianResult, String> {
    if n == 0 {
        return Err("Graph must have at least one vertex".into());
    }

    let mut adj = vec![vec![false; n]; n];
    for &(u, v) in edges {
        if u >= n || v >= n {
            return Err(format!(
                "Edge ({u}, {v}) refers to a vertex outside 0..{n}"
            ));
        }
        if u != v {
            adj[u][v] = true;
            adj[v][u] = true;
        }
    }

    // A single vertex is trivially its own cycle.
    if n == 1 {
        return Ok(HamiltonianResult {
            vertex_count: 1,
            found: true,
            cancelled: false,
            steps: 0,
            cycle: vec![0, 0],
        });
    }

    let mut search = Search {
        n,
        adj,
        path: vec![0; n],
        visited: vec![false; n],
        steps: 0,
        cancelled: false,
        cancel,
    };
    // Any Hamiltonian cycle can be rotated to start at vertex 0.
    search.visited[0] = true;
    let found = search.solve(1);

    let cycle = if found {
        let mut c = search.path.clone();
        c.push(search.path[0]);
        c
    } else {
        Vec::new()
    };

    Ok(HamiltonianResult {
        vertex_count: n,
        found,
        cancelled: search.cancelled,
        steps: search.steps,
        cycle,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: usize, edges: &[(usize, usize)]) -> HamiltonianResult {
        hamiltonian_cycle(n, edges, &AtomicBool::new(false)).unwrap()
    }

    fn assert_is_cycle(res: &HamiltonianResult, n: usize, edges: &[(usize, usize)]) {
        assert!(res.found);
        assert_eq!(res.cycle.len(), n + 1, "cycle must revisit the start");
        assert_eq!(res.cycle[0], *res.cycle.last().unwrap());

        let mut seen = vec![false; n];
        for &v in &res.cycle[..n] {
            assert!(!seen[v], "vertex {v} visited twice");
            seen[v] = true;
        }
        assert!(seen.iter().all(|&s| s), "not every vertex visited");

        let has = |a: usize, b: usize| {
            edges.iter().any(|&(u, v)| (u, v) == (a, b) || (u, v) == (b, a))
        };
        for w in res.cycle.windows(2) {
            assert!(has(w[0], w[1]), "edge {:?} absent from graph", w);
        }
    }

    #[test]
    fn finds_cycle_in_a_square() {
        let edges = [(0, 1), (1, 2), (2, 3), (3, 0)];
        assert_is_cycle(&run(4, &edges), 4, &edges);
    }

    #[test]
    fn finds_cycle_in_complete_graph() {
        let edges = [(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];
        assert_is_cycle(&run(4, &edges), 4, &edges);
    }

    #[test]
    fn reports_absence_on_a_path() {
        // A path graph has no cycle.
        let res = run(4, &[(0, 1), (1, 2), (2, 3)]);
        assert!(!res.found);
        assert!(res.cycle.is_empty());
    }

    #[test]
    fn reports_absence_on_a_star() {
        let res = run(4, &[(0, 1), (0, 2), (0, 3)]);
        assert!(!res.found);
    }

    #[test]
    fn single_vertex_is_trivially_cyclic() {
        let res = run(1, &[]);
        assert!(res.found);
        assert_eq!(res.cycle, vec![0, 0]);
    }

    #[test]
    fn disconnected_graph_has_no_cycle() {
        assert!(!run(4, &[(0, 1), (2, 3)]).found);
    }

    #[test]
    fn rejects_out_of_range_edge() {
        assert!(hamiltonian_cycle(3, &[(0, 7)], &AtomicBool::new(false)).is_err());
    }

    #[test]
    fn ignores_self_loops() {
        let edges = [(0, 0), (0, 1), (1, 2), (2, 3), (3, 0)];
        let res = run(4, &edges);
        assert!(res.found);
    }
}
