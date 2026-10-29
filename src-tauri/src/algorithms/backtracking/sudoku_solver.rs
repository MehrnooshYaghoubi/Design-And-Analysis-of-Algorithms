//! Sudoku solver: backtracking over the most-constrained cell first.
//!
//! Row, column and box candidate sets are kept as 9-bit masks, so the legal
//! digits for a cell are one AND. Always descending into the cell with the
//! fewest candidates (minimum remaining values) prunes far harder than
//! scanning cells in order.

use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};

const N: usize = 9;
const BOX: usize = 3;
const ALL: u16 = 0x1FF; // bits 0..=8 set

#[derive(Debug, Clone, Serialize)]
pub struct SudokuResult {
    pub solved: bool,
    pub cancelled: bool,
    /// Number of cells filled by the search (0 if the puzzle was already full).
    pub cells_filled: usize,
    /// Backtracking steps taken, a measure of puzzle difficulty.
    pub steps: usize,
    /// The solved grid, or the original grid when unsolvable.
    pub grid: Vec<Vec<u8>>,
}

struct Solver<'a> {
    grid: [[u8; N]; N],
    rows: [u16; N],
    cols: [u16; N],
    boxes: [u16; N],
    steps: usize,
    cancelled: bool,
    cancel: &'a AtomicBool,
}

#[inline]
fn box_index(r: usize, c: usize) -> usize {
    (r / BOX) * BOX + c / BOX
}

impl<'a> Solver<'a> {
    fn new(grid: [[u8; N]; N], cancel: &'a AtomicBool) -> Result<Self, String> {
        let mut s = Solver {
            grid,
            rows: [0; N],
            cols: [0; N],
            boxes: [0; N],
            steps: 0,
            cancelled: false,
            cancel,
        };

        for r in 0..N {
            for c in 0..N {
                let v = s.grid[r][c];
                if v == 0 {
                    continue;
                }
                if v > 9 {
                    return Err(format!("Cell ({r},{c}) holds {v}; digits must be 0-9"));
                }
                let bit = 1u16 << (v - 1);
                let b = box_index(r, c);
                if s.rows[r] & bit != 0 || s.cols[c] & bit != 0 || s.boxes[b] & bit != 0 {
                    return Err(format!(
                        "Puzzle is inconsistent: digit {v} repeats at row {r}, column {c}"
                    ));
                }
                s.rows[r] |= bit;
                s.cols[c] |= bit;
                s.boxes[b] |= bit;
            }
        }
        Ok(s)
    }

    #[inline]
    fn candidates(&self, r: usize, c: usize) -> u16 {
        ALL & !(self.rows[r] | self.cols[c] | self.boxes[box_index(r, c)])
    }

    /// Picks the empty cell with the fewest legal digits. Returns `None` when
    /// the grid is full. A cell with zero candidates is returned immediately so
    /// the caller fails fast.
    fn most_constrained(&self) -> Option<(usize, usize, u16)> {
        let mut best: Option<(usize, usize, u16)> = None;
        let mut best_count = u32::MAX;

        for r in 0..N {
            for c in 0..N {
                if self.grid[r][c] != 0 {
                    continue;
                }
                let cand = self.candidates(r, c);
                let count = cand.count_ones();
                if count < best_count {
                    best_count = count;
                    best = Some((r, c, cand));
                    if count <= 1 {
                        return best; // 0 = dead end, 1 = forced
                    }
                }
            }
        }
        best
    }

    fn solve(&mut self) -> bool {
        if self.cancel.load(Ordering::Relaxed) {
            self.cancelled = true;
            return false;
        }

        let Some((r, c, mut cand)) = self.most_constrained() else {
            return true; // no empty cells left
        };

        while cand != 0 {
            let bit = cand & cand.wrapping_neg();
            cand ^= bit;

            let b = box_index(r, c);
            self.grid[r][c] = bit.trailing_zeros() as u8 + 1;
            self.rows[r] |= bit;
            self.cols[c] |= bit;
            self.boxes[b] |= bit;
            self.steps += 1;

            if self.solve() {
                return true;
            }
            if self.cancelled {
                return false;
            }

            // Undo and try the next candidate.
            self.grid[r][c] = 0;
            self.rows[r] &= !bit;
            self.cols[c] &= !bit;
            self.boxes[b] &= !bit;
        }

        false
    }
}

pub fn solve_sudoku(grid_in: Vec<Vec<u8>>, cancel: &AtomicBool) -> Result<SudokuResult, String> {
    if grid_in.len() != N || grid_in.iter().any(|row| row.len() != N) {
        return Err("Sudoku grid must be exactly 9 x 9".into());
    }

    let mut grid = [[0u8; N]; N];
    let mut empty = 0usize;
    for r in 0..N {
        for c in 0..N {
            grid[r][c] = grid_in[r][c];
            if grid[r][c] == 0 {
                empty += 1;
            }
        }
    }

    let mut solver = Solver::new(grid, cancel)?;
    let solved = solver.solve();

    let out = if solved {
        solver.grid.iter().map(|r| r.to_vec()).collect()
    } else {
        grid_in
    };

    Ok(SudokuResult {
        solved,
        cancelled: solver.cancelled,
        cells_filled: if solved { empty } else { 0 },
        steps: solver.steps,
        grid: out,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE: [[u8; 9]; 9] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    fn grid_of(p: [[u8; 9]; 9]) -> Vec<Vec<u8>> {
        p.iter().map(|r| r.to_vec()).collect()
    }

    fn assert_valid(g: &[Vec<u8>]) {
        for i in 0..9 {
            let mut row = [false; 10];
            let mut col = [false; 10];
            for j in 0..9 {
                assert!(!row[g[i][j] as usize], "duplicate in row {i}");
                row[g[i][j] as usize] = true;
                assert!(!col[g[j][i] as usize], "duplicate in column {i}");
                col[g[j][i] as usize] = true;
            }
        }
        for br in 0..3 {
            for bc in 0..3 {
                let mut seen = [false; 10];
                for r in 0..3 {
                    for c in 0..3 {
                        let v = g[br * 3 + r][bc * 3 + c] as usize;
                        assert!(!seen[v], "duplicate in box ({br},{bc})");
                        seen[v] = true;
                    }
                }
            }
        }
    }

    #[test]
    fn solves_classic_puzzle() {
        let res = solve_sudoku(grid_of(PUZZLE), &AtomicBool::new(false)).unwrap();
        assert!(res.solved);
        assert_eq!(res.cells_filled, 51);
        assert_valid(&res.grid);
        // Givens must be preserved.
        for r in 0..9 {
            for c in 0..9 {
                if PUZZLE[r][c] != 0 {
                    assert_eq!(res.grid[r][c], PUZZLE[r][c]);
                }
            }
        }
    }

    #[test]
    fn detects_unsolvable_puzzle() {
        let mut p = PUZZLE;
        // Force a contradiction that is not an immediate duplicate.
        p[0][2] = 1;
        p[1][1] = 2;
        p[0][3] = 2;
        let res = solve_sudoku(grid_of(p), &AtomicBool::new(false));
        match res {
            Ok(r) => assert!(!r.solved, "expected no solution"),
            Err(_) => {} // rejected as inconsistent, also acceptable
        }
    }

    #[test]
    fn rejects_duplicate_givens() {
        let mut p = PUZZLE;
        p[0][2] = 5; // row already has a 5
        assert!(solve_sudoku(grid_of(p), &AtomicBool::new(false)).is_err());
    }

    #[test]
    fn rejects_wrong_shape() {
        assert!(solve_sudoku(vec![vec![0; 9]; 8], &AtomicBool::new(false)).is_err());
    }

    #[test]
    fn already_solved_grid_is_accepted() {
        let solved = solve_sudoku(grid_of(PUZZLE), &AtomicBool::new(false)).unwrap().grid;
        let again = solve_sudoku(solved.clone(), &AtomicBool::new(false)).unwrap();
        assert!(again.solved);
        assert_eq!(again.cells_filled, 0);
        assert_eq!(again.grid, solved);
    }
}
