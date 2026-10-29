//! Travelling salesman, exact, via branch and bound.
//!
//! Depth-first over partial tours with a lower bound built from the two
//! cheapest edges at each unvisited city (every city needs an arrival and a
//! departure, so half their sum can never overestimate the remaining cost).
//! Branches whose bound already matches or exceeds the incumbent are cut.

use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};

/// Exact TSP is factorial; refuse inputs that cannot finish.
pub const MAX_CITIES: usize = 16;

#[derive(Debug, Clone, Serialize)]
pub struct TspResult {
    pub city_count: usize,
    /// Cost of the best tour found.
    pub best_cost: f64,
    /// The tour as city indices, starting and ending at city 0.
    pub tour: Vec<usize>,
    /// Search nodes explored, a measure of how hard the instance was.
    pub nodes_explored: usize,
    pub cancelled: bool,
    /// True when no tour visits every city (the graph is not connected enough).
    pub no_tour: bool,
}

struct Search<'a> {
    n: usize,
    dist: &'a [Vec<f64>],
    /// Half the sum of the two cheapest edges at each city.
    min_edge_half: Vec<f64>,
    visited: Vec<bool>,
    path: Vec<usize>,
    best_cost: f64,
    best_path: Vec<usize>,
    nodes: usize,
    cancelled: bool,
    cancel: &'a AtomicBool,
}

impl<'a> Search<'a> {
    /// Optimistic estimate of what the unvisited cities must still cost.
    fn remaining_bound(&self) -> f64 {
        (0..self.n)
            .filter(|&c| !self.visited[c])
            .map(|c| self.min_edge_half[c])
            .sum()
    }

    fn solve(&mut self, level: usize, cost_so_far: f64) {
        if self.cancel.load(Ordering::Relaxed) {
            self.cancelled = true;
            return;
        }
        self.nodes += 1;

        if level == self.n {
            // Close the tour back to the start.
            let last = self.path[self.n - 1];
            let home = self.dist[last][0];
            if home.is_finite() {
                let total = cost_so_far + home;
                if total < self.best_cost {
                    self.best_cost = total;
                    self.best_path = self.path.clone();
                }
            }
            return;
        }

        // Cut when even an optimistic completion cannot beat the incumbent.
        if cost_so_far + self.remaining_bound() >= self.best_cost {
            return;
        }

        let current = self.path[level - 1];

        // Try nearer cities first: a good incumbent found early prunes more.
        let mut candidates: Vec<(usize, f64)> = (0..self.n)
            .filter(|&c| !self.visited[c] && self.dist[current][c].is_finite())
            .map(|c| (c, self.dist[current][c]))
            .collect();
        candidates.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        for (city, step) in candidates {
            let next_cost = cost_so_far + step;
            if next_cost >= self.best_cost {
                continue; // sorted, so everything after is worse too
            }

            self.visited[city] = true;
            self.path[level] = city;

            self.solve(level + 1, next_cost);

            self.visited[city] = false;
            if self.cancelled {
                return;
            }
        }
    }
}

/// `dist[i][j]` is the cost from city `i` to `j`; use `f64::INFINITY` for
/// "no road". The matrix need not be symmetric.
pub fn tsp_branch_and_bound(
    dist: &[Vec<f64>],
    cancel: &AtomicBool,
) -> Result<TspResult, String> {
    let n = dist.len();
    if n == 0 {
        return Err("Need at least one city".into());
    }
    if dist.iter().any(|row| row.len() != n) {
        return Err("Distance matrix must be square".into());
    }
    if n > MAX_CITIES {
        return Err(format!(
            "{n} cities is beyond exact search; maximum is {MAX_CITIES}"
        ));
    }
    if dist
        .iter()
        .flatten()
        .any(|d| d.is_nan() || (*d < 0.0 && d.is_finite()))
    {
        return Err("Distances must be non-negative and not NaN".into());
    }

    if n == 1 {
        return Ok(TspResult {
            city_count: 1,
            best_cost: 0.0,
            tour: vec![0, 0],
            nodes_explored: 0,
            cancelled: false,
            no_tour: false,
        });
    }

    // Two cheapest outgoing edges per city, halved.
    let min_edge_half: Vec<f64> = (0..n)
        .map(|i| {
            let mut edges: Vec<f64> = (0..n)
                .filter(|&j| j != i && dist[i][j].is_finite())
                .map(|j| dist[i][j])
                .collect();
            edges.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            match edges.len() {
                0 => 0.0,
                1 => edges[0] / 2.0,
                _ => (edges[0] + edges[1]) / 2.0,
            }
        })
        .collect();

    let mut search = Search {
        n,
        dist,
        min_edge_half,
        visited: vec![false; n],
        path: vec![0; n],
        best_cost: f64::INFINITY,
        best_path: Vec::new(),
        nodes: 0,
        cancelled: false,
        cancel,
    };
    // Every tour is a cycle, so fixing the start at city 0 loses nothing.
    search.visited[0] = true;
    search.path[0] = 0;
    search.solve(1, 0.0);

    let no_tour = !search.best_cost.is_finite();
    let tour = if no_tour {
        Vec::new()
    } else {
        let mut t = search.best_path.clone();
        t.push(0);
        t
    };

    Ok(TspResult {
        city_count: n,
        best_cost: if no_tour { 0.0 } else { search.best_cost },
        tour,
        nodes_explored: search.nodes,
        cancelled: search.cancelled,
        no_tour,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(d: &[Vec<f64>]) -> TspResult {
        tsp_branch_and_bound(d, &AtomicBool::new(false)).unwrap()
    }

    /// Brute-force optimum, to check the bound never cuts a better tour.
    fn brute_force(dist: &[Vec<f64>]) -> f64 {
        let n = dist.len();
        let mut rest: Vec<usize> = (1..n).collect();
        let mut best = f64::INFINITY;
        permute(&mut rest, 0, dist, &mut best);
        best
    }

    fn permute(order: &mut Vec<usize>, k: usize, dist: &[Vec<f64>], best: &mut f64) {
        if k == order.len() {
            let mut cost = 0.0;
            let mut prev = 0usize;
            for &c in order.iter() {
                cost += dist[prev][c];
                prev = c;
            }
            cost += dist[prev][0];
            if cost < *best {
                *best = cost;
            }
            return;
        }
        for i in k..order.len() {
            order.swap(k, i);
            permute(order, k + 1, dist, best);
            order.swap(k, i);
        }
    }

    fn assert_valid_tour(r: &TspResult, n: usize) {
        assert_eq!(r.tour.len(), n + 1);
        assert_eq!(r.tour[0], 0);
        assert_eq!(*r.tour.last().unwrap(), 0);
        let mut seen = vec![false; n];
        for &c in &r.tour[..n] {
            assert!(!seen[c], "city {c} visited twice");
            seen[c] = true;
        }
        assert!(seen.iter().all(|&s| s));
    }

    #[test]
    fn classic_four_city_instance() {
        let d = vec![
            vec![0.0, 10.0, 15.0, 20.0],
            vec![10.0, 0.0, 35.0, 25.0],
            vec![15.0, 35.0, 0.0, 30.0],
            vec![20.0, 25.0, 30.0, 0.0],
        ];
        let r = run(&d);
        assert_eq!(r.best_cost, 80.0);
        assert_valid_tour(&r, 4);
    }

    #[test]
    fn matches_brute_force_on_several_instances() {
        let instances = vec![
            vec![
                vec![0.0, 2.0, 9.0, 10.0],
                vec![1.0, 0.0, 6.0, 4.0],
                vec![15.0, 7.0, 0.0, 8.0],
                vec![6.0, 3.0, 12.0, 0.0],
            ],
            vec![
                vec![0.0, 3.0, 1.0, 5.0, 8.0],
                vec![3.0, 0.0, 6.0, 7.0, 9.0],
                vec![1.0, 6.0, 0.0, 4.0, 2.0],
                vec![5.0, 7.0, 4.0, 0.0, 3.0],
                vec![8.0, 9.0, 2.0, 3.0, 0.0],
            ],
        ];
        for d in instances {
            let r = run(&d);
            assert!(
                (r.best_cost - brute_force(&d)).abs() < 1e-9,
                "bound pruned the optimum: got {}, want {}",
                r.best_cost,
                brute_force(&d)
            );
            assert_valid_tour(&r, d.len());
        }
    }

    #[test]
    fn asymmetric_matrix() {
        let d = vec![
            vec![0.0, 1.0, 100.0],
            vec![100.0, 0.0, 1.0],
            vec![1.0, 100.0, 0.0],
        ];
        let r = run(&d);
        assert_eq!(r.best_cost, 3.0); // 0 -> 1 -> 2 -> 0
        assert_valid_tour(&r, 3);
    }

    #[test]
    fn single_city() {
        let r = run(&[vec![0.0]]);
        assert_eq!(r.best_cost, 0.0);
        assert_eq!(r.tour, vec![0, 0]);
    }

    #[test]
    fn reports_when_no_tour_exists() {
        let inf = f64::INFINITY;
        // City 2 can be entered but never left.
        let d = vec![
            vec![0.0, 1.0, 1.0],
            vec![1.0, 0.0, 1.0],
            vec![inf, inf, 0.0],
        ];
        let r = run(&d);
        assert!(r.no_tour);
        assert!(r.tour.is_empty());
    }

    #[test]
    fn rejects_bad_input() {
        assert!(tsp_branch_and_bound(&[], &AtomicBool::new(false)).is_err());
        assert!(tsp_branch_and_bound(
            &[vec![0.0, 1.0], vec![1.0]],
            &AtomicBool::new(false)
        )
        .is_err());
        assert!(tsp_branch_and_bound(
            &[vec![0.0, -5.0], vec![1.0, 0.0]],
            &AtomicBool::new(false)
        )
        .is_err());
        assert!(tsp_branch_and_bound(
            &vec![vec![0.0; MAX_CITIES + 1]; MAX_CITIES + 1],
            &AtomicBool::new(false)
        )
        .is_err());
    }

    #[test]
    fn respects_cancellation() {
        let flag = AtomicBool::new(true);
        let d = vec![vec![0.0; 10]; 10];
        let r = tsp_branch_and_bound(&d, &flag).unwrap();
        assert!(r.cancelled);
    }
}
