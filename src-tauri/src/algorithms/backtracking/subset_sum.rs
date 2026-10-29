//! Subset sum via backtracking with suffix-sum and sorted pruning.
//!
//! Numbers are sorted descending and suffix sums precomputed, giving two
//! cheap cuts at every node: stop if the running total already exceeds the
//! target, and stop if every remaining number together cannot reach it.

use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Serialize)]
pub struct SubsetSumResult {
    pub target: i64,
    /// True when at least one subset sums to the target.
    pub found: bool,
    /// Total number of distinct subsets that reach the target.
    pub subset_count: usize,
    pub cancelled: bool,
    /// True when more subsets exist than were materialised below.
    pub truncated: bool,
    /// The values of each returned subset, in input order.
    pub subsets: Vec<Vec<i64>>,
}

struct Search<'a> {
    /// Values sorted descending, paired with their original index.
    values: Vec<(usize, i64)>,
    /// `pos_suffix[i]` is the sum of all positive values from `i` onward:
    /// the most the running total can still gain.
    pos_suffix: Vec<i64>,
    /// `neg_suffix[i]` is the sum of all negative values from `i` onward:
    /// the most the running total can still lose.
    neg_suffix: Vec<i64>,
    target: i64,
    max_subsets: usize,
    count: usize,
    cancelled: bool,
    chosen: Vec<(usize, i64)>,
    subsets: Vec<Vec<i64>>,
    cancel: &'a AtomicBool,
}

impl<'a> Search<'a> {
    fn record(&mut self) {
        self.count += 1;
        if self.subsets.len() < self.max_subsets {
            // Report in the caller's original ordering, not the sorted one.
            let mut picked = self.chosen.clone();
            picked.sort_unstable_by_key(|&(idx, _)| idx);
            self.subsets.push(picked.into_iter().map(|(_, v)| v).collect());
        }
    }

    fn solve(&mut self, i: usize, sum: i64) {
        if self.cancel.load(Ordering::Relaxed) {
            self.cancelled = true;
            return;
        }

        // A subset is complete only once every value has been decided on, so
        // recording here counts each distinct subset exactly once. Recording
        // the moment `sum == target` instead would re-count the same subset
        // once per remaining skip decision.
        if i == self.values.len() {
            if sum == self.target {
                self.record();
            }
            return;
        }

        // Bound in both directions: negative values mean an overshoot is still
        // recoverable, so a one-sided bound would prune valid subsets.
        if sum + self.pos_suffix[i] < self.target {
            return; // cannot climb to the target
        }
        if sum + self.neg_suffix[i] > self.target {
            return; // cannot fall to the target
        }

        let (idx, val) = self.values[i];

        // Branch 1: take this value.
        self.chosen.push((idx, val));
        self.solve(i + 1, sum + val);
        self.chosen.pop();
        if self.cancelled {
            return;
        }

        // Branch 2: skip it.
        self.solve(i + 1, sum);
    }
}

pub fn subset_sum(
    numbers: &[i64],
    target: i64,
    max_subsets: usize,
    cancel: &AtomicBool,
) -> SubsetSumResult {
    let mut values: Vec<(usize, i64)> = numbers.iter().copied().enumerate().collect();
    // Descending order makes the suffix-sum bound bite early.
    values.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    // Separate optimistic bounds for climbing and falling.
    let mut pos_suffix = vec![0i64; values.len() + 1];
    let mut neg_suffix = vec![0i64; values.len() + 1];
    for i in (0..values.len()).rev() {
        pos_suffix[i] = pos_suffix[i + 1] + values[i].1.max(0);
        neg_suffix[i] = neg_suffix[i + 1] + values[i].1.min(0);
    }

    let mut search = Search {
        values,
        pos_suffix,
        neg_suffix,
        target,
        max_subsets,
        count: 0,
        cancelled: false,
        chosen: Vec::new(),
        subsets: Vec::new(),
        cancel,
    };
    search.solve(0, 0);

    SubsetSumResult {
        target,
        found: search.count > 0,
        subset_count: search.count,
        cancelled: search.cancelled,
        truncated: search.count > search.subsets.len(),
        subsets: search.subsets,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(nums: &[i64], target: i64) -> SubsetSumResult {
        subset_sum(nums, target, 64, &AtomicBool::new(false))
    }

    #[test]
    fn finds_a_subset() {
        let res = run(&[3, 34, 4, 12, 5, 2], 9);
        assert!(res.found);
        for s in &res.subsets {
            assert_eq!(s.iter().sum::<i64>(), 9);
        }
    }

    #[test]
    fn reports_absence() {
        let res = run(&[3, 34, 4, 12, 5, 2], 30);
        assert!(!res.found);
        assert_eq!(res.subset_count, 0);
        assert!(res.subsets.is_empty());
    }

    #[test]
    fn counts_every_subset() {
        // 1+4, 2+3, and 5 all reach 5.
        let res = run(&[1, 2, 3, 4, 5], 5);
        assert_eq!(res.subset_count, 3);
    }

    #[test]
    fn empty_subset_matches_zero_target() {
        let res = run(&[1, 2, 3], 0);
        assert!(res.found);
        assert!(res.subsets.iter().any(|s| s.is_empty()));
    }

    #[test]
    fn handles_negative_values() {
        // -3 + 5 + 2 = 4 and 4 alone = 4.
        let res = run(&[-3, 5, 2, 4], 4);
        assert!(res.found);
        for s in &res.subsets {
            assert_eq!(s.iter().sum::<i64>(), 4);
        }
        assert!(res.subset_count >= 2);
    }

    #[test]
    fn empty_input() {
        assert!(!run(&[], 7).found);
        assert!(run(&[], 0).found);
    }

    #[test]
    fn output_is_capped_but_count_is_not() {
        let nums: Vec<i64> = vec![1; 20];
        // C(20,5) = 15504 subsets of five 1s sum to 5.
        let res = subset_sum(&nums, 5, 10, &AtomicBool::new(false));
        assert_eq!(res.subset_count, 15504);
        assert_eq!(res.subsets.len(), 10);
        assert!(res.truncated);
    }

    #[test]
    fn preserves_input_order_in_output() {
        let res = run(&[5, 1, 4], 9);
        assert!(res.found);
        // The 5+4 subset must be reported as [5, 4], matching input order.
        assert!(res.subsets.iter().any(|s| s == &vec![5, 4]));
    }
}
