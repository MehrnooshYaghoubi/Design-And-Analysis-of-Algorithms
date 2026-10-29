//! Floyd-Warshall all-pairs shortest paths.
//!
//! Replaces an earlier placeholder that held Python source. Uses `Option<i64>`
//! for "unreachable" rather than a sentinel like `i64::MAX`, which silently
//! overflows the moment you add two of them together.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct FloydWarshallResult {
    pub vertex_count: usize,
    /// `dist[i][j]` is the shortest distance, or `None` if `j` is unreachable
    /// from `i`.
    pub dist: Vec<Vec<Option<i64>>>,
    /// True when some vertex lies on a negative-weight cycle.
    pub has_negative_cycle: bool,
    /// Vertices whose shortest paths are undefined because of a negative cycle.
    pub negative_cycle_vertices: Vec<usize>,
}

/// `matrix[i][j]` is the direct edge weight, or `None` for no edge.
/// A `0`-weight self-loop is assumed for every vertex.
pub fn floyd_warshall(matrix: &[Vec<Option<i64>>]) -> Result<FloydWarshallResult, String> {
    let n = matrix.len();
    if matrix.iter().any(|row| row.len() != n) {
        return Err("Adjacency matrix must be square".into());
    }

    let mut dist = matrix.to_vec();
    for i in 0..n {
        // A vertex always reaches itself at no cost, unless a negative
        // self-loop was supplied.
        dist[i][i] = Some(dist[i][i].unwrap_or(0).min(0));
    }

    for k in 0..n {
        for i in 0..n {
            // Skip the whole row when i cannot reach k at all.
            let Some(ik) = dist[i][k] else { continue };
            for j in 0..n {
                let Some(kj) = dist[k][j] else { continue };
                let through = ik.saturating_add(kj);
                match dist[i][j] {
                    Some(current) if current <= through => {}
                    _ => dist[i][j] = Some(through),
                }
            }
        }
    }

    // A negative self-distance means the vertex sits on a negative cycle.
    let negative_cycle_vertices: Vec<usize> = (0..n)
        .filter(|&i| dist[i][i].map_or(false, |d| d < 0))
        .collect();

    Ok(FloydWarshallResult {
        vertex_count: n,
        dist,
        has_negative_cycle: !negative_cycle_vertices.is_empty(),
        negative_cycle_vertices,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INF: Option<i64> = None;

    #[test]
    fn classic_four_vertex_graph() {
        let m = vec![
            vec![Some(0), Some(5), INF, Some(10)],
            vec![INF, Some(0), Some(3), INF],
            vec![INF, INF, Some(0), Some(1)],
            vec![INF, INF, INF, Some(0)],
        ];
        let r = floyd_warshall(&m).unwrap();
        assert_eq!(r.dist[0][1], Some(5));
        assert_eq!(r.dist[0][2], Some(8));
        // 0 -> 1 -> 2 -> 3 costs 9, cheaper than the direct edge of 10.
        assert_eq!(r.dist[0][3], Some(9));
        assert!(!r.has_negative_cycle);
    }

    #[test]
    fn unreachable_stays_none() {
        let m = vec![
            vec![Some(0), Some(1), INF],
            vec![INF, Some(0), INF],
            vec![INF, INF, Some(0)],
        ];
        let r = floyd_warshall(&m).unwrap();
        assert_eq!(r.dist[0][2], INF);
        assert_eq!(r.dist[2][0], INF);
        assert_eq!(r.dist[2][2], Some(0));
    }

    #[test]
    fn handles_negative_edges_without_a_cycle() {
        let m = vec![
            vec![Some(0), Some(4), Some(5)],
            vec![INF, Some(0), Some(-3)],
            vec![INF, INF, Some(0)],
        ];
        let r = floyd_warshall(&m).unwrap();
        assert_eq!(r.dist[0][2], Some(1)); // 4 + (-3) beats 5
        assert!(!r.has_negative_cycle);
    }

    #[test]
    fn detects_negative_cycle() {
        let m = vec![
            vec![Some(0), Some(1), INF],
            vec![INF, Some(0), Some(-4)],
            vec![Some(1), INF, Some(0)],
        ];
        let r = floyd_warshall(&m).unwrap();
        assert!(r.has_negative_cycle);
        assert!(!r.negative_cycle_vertices.is_empty());
    }

    #[test]
    fn no_overflow_on_unreachable_pairs() {
        // Two "infinite" legs must never be added into a finite distance.
        let m = vec![
            vec![Some(0), INF, INF],
            vec![INF, Some(0), INF],
            vec![INF, INF, Some(0)],
        ];
        let r = floyd_warshall(&m).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(r.dist[i][j], if i == j { Some(0) } else { INF });
            }
        }
    }

    #[test]
    fn rejects_non_square_matrix() {
        assert!(floyd_warshall(&[vec![Some(0), Some(1)], vec![Some(1)]]).is_err());
    }

    #[test]
    fn empty_graph() {
        let r = floyd_warshall(&[]).unwrap();
        assert_eq!(r.vertex_count, 0);
        assert!(!r.has_negative_cycle);
    }
}
