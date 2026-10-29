//! 0/1 knapsack.
//!
//! Two strategies share one entry point. When the full `n x capacity` table
//! fits in `MAX_TABLE_CELLS` we build it and walk it backwards to recover the
//! chosen items. Past that the table alone would be gigabytes, so we fall back
//! to a rolling two-row DP that reports the optimal value using O(capacity)
//! memory without the item list.

use serde::Serialize;

use crate::types::WeightedItem;

/// Above this many table cells we stop materialising the full DP table.
pub const MAX_TABLE_CELLS: usize = 40_000_000;

#[derive(Debug, Clone, Serialize)]
pub struct Knapsack01Result {
    pub capacity: u64,
    /// Best achievable total value.
    pub best_value: u64,
    /// Total weight of the selected items.
    pub total_weight: u64,
    /// Indices of the chosen items, or `None` when the input was too large to
    /// reconstruct the selection.
    pub selected_indices: Option<Vec<usize>>,
    /// True when only the value was computed, without the item list.
    pub value_only: bool,
}

pub fn knapsack_01(items: &[WeightedItem], capacity: u64) -> Knapsack01Result {
    let n = items.len();
    let cap = capacity as usize;

    if n == 0 || cap == 0 {
        return Knapsack01Result {
            capacity,
            best_value: 0,
            total_weight: 0,
            selected_indices: Some(Vec::new()),
            value_only: false,
        };
    }

    let cells = (n + 1).saturating_mul(cap + 1);
    if cells <= MAX_TABLE_CELLS {
        full_table(items, capacity)
    } else {
        rolling_rows(items, capacity)
    }
}

/// Full table plus backward reconstruction of the chosen items.
fn full_table(items: &[WeightedItem], capacity: u64) -> Knapsack01Result {
    let n = items.len();
    let cap = capacity as usize;
    let mut table = vec![vec![0u64; cap + 1]; n + 1];

    for i in 1..=n {
        let w = items[i - 1].weight as usize;
        let v = items[i - 1].value;
        for c in 0..=cap {
            // Skipping item i-1 is always allowed.
            let mut best = table[i - 1][c];
            if w <= c {
                let take = table[i - 1][c - w] + v;
                if take > best {
                    best = take;
                }
            }
            table[i][c] = best;
        }
    }

    // Walk back: the item was taken exactly when the value changed.
    let mut selected = Vec::new();
    let mut c = cap;
    for i in (1..=n).rev() {
        if table[i][c] != table[i - 1][c] {
            selected.push(i - 1);
            c -= items[i - 1].weight as usize;
        }
    }
    selected.reverse();

    let total_weight = selected.iter().map(|&i| items[i].weight).sum();

    Knapsack01Result {
        capacity,
        best_value: table[n][cap],
        total_weight,
        selected_indices: Some(selected),
        value_only: false,
    }
}

/// Rolling two-row DP: optimal value only, O(capacity) memory.
fn rolling_rows(items: &[WeightedItem], capacity: u64) -> Knapsack01Result {
    let cap = capacity as usize;
    let mut prev = vec![0u64; cap + 1];
    let mut cur = vec![0u64; cap + 1];

    for item in items {
        let w = item.weight as usize;
        let v = item.value;
        for c in 0..=cap {
            let mut best = prev[c];
            if w <= c {
                let take = prev[c - w] + v;
                if take > best {
                    best = take;
                }
            }
            cur[c] = best;
        }
        std::mem::swap(&mut prev, &mut cur);
    }

    Knapsack01Result {
        capacity,
        best_value: prev[cap],
        total_weight: 0,
        selected_indices: None,
        value_only: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items(pairs: &[(u64, u64)]) -> Vec<WeightedItem> {
        pairs
            .iter()
            .map(|&(value, weight)| WeightedItem { value, weight })
            .collect()
    }

    #[test]
    fn classic_instance() {
        // Values 60/100/120 against weights 10/20/30, capacity 50.
        // Optimum is items 1 and 2: value 220, weight 50.
        let it = items(&[(60, 10), (100, 20), (120, 30)]);
        let r = knapsack_01(&it, 50);
        assert_eq!(r.best_value, 220);
        assert_eq!(r.total_weight, 50);
        assert_eq!(r.selected_indices, Some(vec![1, 2]));
        assert!(!r.value_only);
    }

    #[test]
    fn selection_is_consistent_with_value() {
        let it = items(&[(10, 5), (40, 4), (30, 6), (50, 3)]);
        let r = knapsack_01(&it, 10);
        let sel = r.selected_indices.clone().unwrap();
        let value: u64 = sel.iter().map(|&i| it[i].value).sum();
        let weight: u64 = sel.iter().map(|&i| it[i].weight).sum();
        assert_eq!(value, r.best_value);
        assert_eq!(weight, r.total_weight);
        assert!(weight <= 10);
        assert_eq!(r.best_value, 90); // items 1 and 3
    }

    #[test]
    fn item_heavier_than_capacity_is_skipped() {
        let it = items(&[(100, 50), (10, 2)]);
        let r = knapsack_01(&it, 10);
        assert_eq!(r.best_value, 10);
        assert_eq!(r.selected_indices, Some(vec![1]));
    }

    #[test]
    fn zero_capacity_and_empty_input() {
        assert_eq!(knapsack_01(&items(&[(5, 1)]), 0).best_value, 0);
        assert_eq!(knapsack_01(&[], 10).best_value, 0);
        assert_eq!(knapsack_01(&[], 10).selected_indices, Some(vec![]));
    }

    #[test]
    fn exact_fit_takes_everything() {
        let it = items(&[(1, 3), (2, 4), (3, 5)]);
        let r = knapsack_01(&it, 12);
        assert_eq!(r.best_value, 6);
        assert_eq!(r.total_weight, 12);
    }

    #[test]
    fn zero_weight_items_are_always_taken() {
        let it = items(&[(7, 0), (5, 10)]);
        let r = knapsack_01(&it, 5);
        assert_eq!(r.best_value, 7);
        assert_eq!(r.selected_indices, Some(vec![0]));
    }

    #[test]
    fn rolling_path_matches_full_table() {
        let it = items(&[(60, 10), (100, 20), (120, 30)]);
        let full = full_table(&it, 50);
        let rolling = rolling_rows(&it, 50);
        assert_eq!(full.best_value, rolling.best_value);
        assert!(rolling.value_only);
        assert_eq!(rolling.selected_indices, None);
    }
}
