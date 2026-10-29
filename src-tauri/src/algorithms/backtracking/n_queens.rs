//! N-Queens via backtracking with bitmask pruning.
//!
//! Columns and both diagonals are tracked as bitmasks, so placement legality
//! is a single AND and the recursion never rescans the board. Solutions are
//! counted in full but only `max_solutions` boards are materialised, which
//! keeps a large `n` from flooding the UI with results.

use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};

/// Largest board we accept; beyond this the search space is not explorable.
pub const MAX_N: usize = 18;

#[derive(Debug, Clone, Serialize)]
pub struct NQueensResult {
    pub n: usize,
    /// Total number of distinct solutions found.
    pub solution_count: usize,
    /// True when the search stopped early via the cancel flag.
    pub cancelled: bool,
    /// True when more solutions exist than were materialised below.
    pub truncated: bool,
    /// One entry per returned solution: `solutions[s][row]` is the queen column.
    pub solutions: Vec<Vec<usize>>,
    /// The same solutions rendered as `.`/`Q` rows, for display.
    pub boards: Vec<Vec<String>>,
}

struct Search<'a> {
    n: usize,
    max_solutions: usize,
    count: usize,
    cancelled: bool,
    placement: Vec<usize>,
    solutions: Vec<Vec<usize>>,
    cancel: &'a AtomicBool,
}

impl<'a> Search<'a> {
    /// Explores one row. `cols`/`d1`/`d2` carry the occupied columns and the
    /// two diagonal families as bitmasks.
    fn solve(&mut self, row: usize, cols: u32, d1: u32, d2: u32) {
        if self.cancel.load(Ordering::Relaxed) {
            self.cancelled = true;
            return;
        }

        if row == self.n {
            self.count += 1;
            if self.solutions.len() < self.max_solutions {
                self.solutions.push(self.placement.clone());
            }
            return;
        }

        let full = (1u32 << self.n) - 1;
        // Every column not attacked by an earlier queen.
        let mut available = full & !(cols | d1 | d2);

        while available != 0 {
            let bit = available & available.wrapping_neg(); // lowest set bit
            available ^= bit;
            let col = bit.trailing_zeros() as usize;

            self.placement[row] = col;
            // Shifting the diagonal masks moves them into the next row's frame.
            self.solve(
                row + 1,
                cols | bit,
                (d1 | bit) << 1,
                (d2 | bit) >> 1,
            );

            if self.cancelled {
                return;
            }
        }
    }
}

fn render(n: usize, solution: &[usize]) -> Vec<String> {
    solution
        .iter()
        .map(|&col| {
            let mut row = vec![b'.'; n];
            row[col] = b'Q';
            String::from_utf8(row).expect("ASCII board row")
        })
        .collect()
}

pub fn n_queens(n: usize, max_solutions: usize, cancel: &AtomicBool) -> Result<NQueensResult, String> {
    if n == 0 {
        return Err("Board size must be at least 1".into());
    }
    if n > MAX_N {
        return Err(format!(
            "Board size {n} is too large to enumerate; maximum is {MAX_N}"
        ));
    }

    let mut search = Search {
        n,
        max_solutions,
        count: 0,
        cancelled: false,
        placement: vec![0; n],
        solutions: Vec::new(),
        cancel,
    };
    search.solve(0, 0, 0, 0);

    let boards = search.solutions.iter().map(|s| render(n, s)).collect();

    Ok(NQueensResult {
        n,
        solution_count: search.count,
        cancelled: search.cancelled,
        truncated: search.count > search.solutions.len(),
        solutions: search.solutions,
        boards,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(n: usize) -> NQueensResult {
        n_queens(n, 8, &AtomicBool::new(false)).unwrap()
    }

    #[test]
    fn known_solution_counts() {
        // Classic sequence: 1, 0, 0, 2, 10, 4, 40, 92 for n = 1..=8.
        let expected = [1, 0, 0, 2, 10, 4, 40, 92];
        for (i, &want) in expected.iter().enumerate() {
            assert_eq!(run(i + 1).solution_count, want, "n = {}", i + 1);
        }
    }

    #[test]
    fn solutions_are_valid() {
        let res = run(8);
        for sol in &res.solutions {
            assert_eq!(sol.len(), 8);
            for i in 0..sol.len() {
                for j in (i + 1)..sol.len() {
                    assert_ne!(sol[i], sol[j], "shared column");
                    let dr = (j - i) as isize;
                    let dc = sol[i] as isize - sol[j] as isize;
                    assert_ne!(dc.abs(), dr, "shared diagonal");
                }
            }
        }
    }

    #[test]
    fn output_is_capped_but_count_is_not() {
        let res = n_queens(8, 3, &AtomicBool::new(false)).unwrap();
        assert_eq!(res.solution_count, 92);
        assert_eq!(res.solutions.len(), 3);
        assert!(res.truncated);
    }

    #[test]
    fn rejects_oversized_board() {
        assert!(n_queens(MAX_N + 1, 1, &AtomicBool::new(false)).is_err());
    }
}
