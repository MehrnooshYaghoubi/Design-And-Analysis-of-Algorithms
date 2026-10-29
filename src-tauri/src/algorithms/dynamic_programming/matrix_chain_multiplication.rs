//! Matrix chain multiplication ordering.
//!
//! Given dimensions `p` where matrix `i` is `p[i] x p[i+1]`, finds the
//! parenthesisation minimising scalar multiplications. Interval DP over
//! increasing chain lengths, with a split table that lets us print the
//! bracketing.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MatrixChainResult {
    pub matrix_count: usize,
    /// Minimum number of scalar multiplications.
    pub min_operations: u64,
    /// The optimal bracketing, e.g. `((A0A1)A2)`.
    pub parenthesization: String,
}

pub fn matrix_chain_order(dims: &[usize]) -> Result<MatrixChainResult, String> {
    if dims.len() < 2 {
        return Err("Need at least two dimensions to describe one matrix".into());
    }
    if dims.iter().any(|&d| d == 0) {
        return Err("Matrix dimensions must be non-zero".into());
    }

    let n = dims.len() - 1; // number of matrices

    if n == 1 {
        return Ok(MatrixChainResult {
            matrix_count: 1,
            min_operations: 0,
            parenthesization: "A0".to_string(),
        });
    }

    // cost[i][j] = cheapest way to multiply matrices i..=j
    let mut cost = vec![vec![0u64; n]; n];
    let mut split = vec![vec![0usize; n]; n];

    // Chains of length 2 upward, so sub-chains are always already solved.
    for len in 2..=n {
        for i in 0..=(n - len) {
            let j = i + len - 1;
            cost[i][j] = u64::MAX;
            for k in i..j {
                // Multiply (i..=k) by (k+1..=j): the two results are
                // p[i] x p[k+1] and p[k+1] x p[j+1].
                let joined = (dims[i] as u64) * (dims[k + 1] as u64) * (dims[j + 1] as u64);
                let total = cost[i][k]
                    .saturating_add(cost[k + 1][j])
                    .saturating_add(joined);
                if total < cost[i][j] {
                    cost[i][j] = total;
                    split[i][j] = k;
                }
            }
        }
    }

    let mut out = String::new();
    build_parens(&split, 0, n - 1, &mut out);

    Ok(MatrixChainResult {
        matrix_count: n,
        min_operations: cost[0][n - 1],
        parenthesization: out,
    })
}

fn build_parens(split: &[Vec<usize>], i: usize, j: usize, out: &mut String) {
    if i == j {
        out.push('A');
        out.push_str(&i.to_string());
        return;
    }
    out.push('(');
    let k = split[i][j];
    build_parens(split, i, k, out);
    build_parens(split, k + 1, j, out);
    out.push(')');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_three_matrix_chain() {
        // 10x30, 30x5, 5x60 -> optimum is (A0A1)A2 at 4500.
        let r = matrix_chain_order(&[10, 30, 5, 60]).unwrap();
        assert_eq!(r.min_operations, 4500);
        assert_eq!(r.parenthesization, "((A0A1)A2)");
        assert_eq!(r.matrix_count, 3);
    }

    #[test]
    fn classic_four_matrix_chain() {
        // 40x20, 20x30, 30x10, 10x30 -> 26000.
        let r = matrix_chain_order(&[40, 20, 30, 10, 30]).unwrap();
        assert_eq!(r.min_operations, 26000);
        assert_eq!(r.parenthesization, "((A0(A1A2))A3)");
    }

    #[test]
    fn five_matrix_chain() {
        let r = matrix_chain_order(&[30, 35, 15, 5, 10, 20, 25]).unwrap();
        assert_eq!(r.min_operations, 15125);
        assert_eq!(r.parenthesization, "((A0(A1A2))((A3A4)A5))");
    }

    #[test]
    fn two_matrices_have_one_ordering() {
        let r = matrix_chain_order(&[10, 20, 30]).unwrap();
        assert_eq!(r.min_operations, 6000);
        assert_eq!(r.parenthesization, "(A0A1)");
    }

    #[test]
    fn single_matrix_costs_nothing() {
        let r = matrix_chain_order(&[10, 20]).unwrap();
        assert_eq!(r.min_operations, 0);
        assert_eq!(r.matrix_count, 1);
        assert_eq!(r.parenthesization, "A0");
    }

    #[test]
    fn rejects_too_few_dimensions() {
        assert!(matrix_chain_order(&[10]).is_err());
        assert!(matrix_chain_order(&[]).is_err());
    }

    #[test]
    fn rejects_zero_dimension() {
        assert!(matrix_chain_order(&[10, 0, 5]).is_err());
    }

    #[test]
    fn parenthesization_is_balanced() {
        let r = matrix_chain_order(&[5, 10, 3, 12, 5, 50, 6]).unwrap();
        let opens = r.parenthesization.matches('(').count();
        let closes = r.parenthesization.matches(')').count();
        assert_eq!(opens, closes);
        // Every matrix must appear exactly once.
        for i in 0..r.matrix_count {
            assert_eq!(
                r.parenthesization.matches(&format!("A{i}")).count(),
                1,
                "A{i} should appear once"
            );
        }
    }
}
