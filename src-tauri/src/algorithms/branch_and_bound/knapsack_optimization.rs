//! 0/1 knapsack via least-cost branch and bound.
//!
//! Best-first search over a priority queue ordered by an optimistic upper
//! bound. The bound comes from the fractional (greedy) relaxation of the
//! remaining items, which is admissible for 0/1 knapsack, so the first
//! complete solution pulled from the queue cannot be beaten. Unlike the DP in
//! `dynamic_programming::knapsack_01`, this needs no table, so it handles
//! huge capacities where the DP table would not fit.

use serde::Serialize;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicBool, Ordering as AtomicOrdering};

use crate::types::WeightedItem;

#[derive(Debug, Clone, Serialize)]
pub struct KnapsackBnbResult {
    pub capacity: f64,
    pub best_value: f64,
    pub total_weight: f64,
    /// Indices of chosen items, in input order.
    pub selected_indices: Vec<usize>,
    /// Nodes expanded from the queue.
    pub nodes_explored: usize,
    pub cancelled: bool,
}

/// An item paired with its original index, sorted by value density.
struct Ranked {
    index: usize,
    value: f64,
    weight: f64,
}

struct Node {
    /// Depth in the ranked ordering.
    level: usize,
    value: f64,
    weight: f64,
    /// Optimistic best total reachable from here.
    bound: f64,
    taken: Vec<usize>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.bound == other.bound
    }
}
impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // BinaryHeap is a max-heap and we want the largest bound first, so
        // this is a direct comparison rather than a reversed one.
        self.bound
            .partial_cmp(&other.bound)
            .unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Fractional relaxation of items from `level` onward: fill greedily by
/// density and allow a fractional last item. Never underestimates.
fn bound_of(level: usize, value: f64, weight: f64, cap: f64, items: &[Ranked]) -> f64 {
    let mut bound = value;
    let mut w = weight;

    for item in items.iter().skip(level) {
        if w + item.weight <= cap {
            w += item.weight;
            bound += item.value;
        } else {
            let room = cap - w;
            if room > 0.0 && item.weight > 0.0 {
                bound += item.value * (room / item.weight);
            }
            break;
        }
    }
    bound
}

pub fn knapsack_branch_and_bound(
    items: &[WeightedItem],
    capacity: f64,
    cancel: &AtomicBool,
) -> Result<KnapsackBnbResult, String> {
    if capacity < 0.0 || capacity.is_nan() {
        return Err("Capacity must be non-negative".into());
    }
    if items
        .iter()
        .any(|i| (i.value as f64).is_nan() || (i.weight as f64).is_nan())
    {
        return Err("Item values and weights must be numbers".into());
    }

    // Sort by value density; the bound is only admissible in this order.
    let mut ranked: Vec<Ranked> = items
        .iter()
        .enumerate()
        .map(|(index, it)| Ranked {
            index,
            value: it.value as f64,
            weight: it.weight as f64,
        })
        .collect();
    ranked.sort_unstable_by(|a, b| {
        let da = if a.weight > 0.0 { a.value / a.weight } else { f64::INFINITY };
        let db = if b.weight > 0.0 { b.value / b.weight } else { f64::INFINITY };
        db.partial_cmp(&da).unwrap_or(Ordering::Equal)
    });

    let mut heap = BinaryHeap::new();
    heap.push(Node {
        level: 0,
        value: 0.0,
        weight: 0.0,
        bound: bound_of(0, 0.0, 0.0, capacity, &ranked),
        taken: Vec::new(),
    });

    let mut best_value = 0.0f64;
    let mut best_taken: Vec<usize> = Vec::new();
    let mut best_weight = 0.0f64;
    let mut nodes = 0usize;
    let mut cancelled = false;

    while let Some(node) = heap.pop() {
        if cancel.load(AtomicOrdering::Relaxed) {
            cancelled = true;
            break;
        }
        nodes += 1;

        // Everything still queued is bounded by this node, so we are done.
        if node.bound <= best_value {
            break;
        }
        if node.level >= ranked.len() {
            continue;
        }

        let item = &ranked[node.level];

        // Branch 1: take the item, if it fits.
        if node.weight + item.weight <= capacity {
            let value = node.value + item.value;
            let weight = node.weight + item.weight;
            let mut taken = node.taken.clone();
            taken.push(item.index);

            if value > best_value {
                best_value = value;
                best_taken = taken.clone();
                best_weight = weight;
            }

            let bound = bound_of(node.level + 1, value, weight, capacity, &ranked);
            if bound > best_value {
                heap.push(Node {
                    level: node.level + 1,
                    value,
                    weight,
                    bound,
                    taken,
                });
            }
        }

        // Branch 2: skip the item.
        let bound = bound_of(node.level + 1, node.value, node.weight, capacity, &ranked);
        if bound > best_value {
            heap.push(Node {
                level: node.level + 1,
                value: node.value,
                weight: node.weight,
                bound,
                taken: node.taken,
            });
        }
    }

    best_taken.sort_unstable();

    Ok(KnapsackBnbResult {
        capacity,
        best_value,
        total_weight: best_weight,
        selected_indices: best_taken,
        nodes_explored: nodes,
        cancelled,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::dynamic_programming::knapsack_01;

    fn items(pairs: &[(u64, u64)]) -> Vec<WeightedItem> {
        pairs
            .iter()
            .map(|&(value, weight)| WeightedItem { value, weight })
            .collect()
    }

    fn run(it: &[WeightedItem], cap: f64) -> KnapsackBnbResult {
        knapsack_branch_and_bound(it, cap, &AtomicBool::new(false)).unwrap()
    }

    #[test]
    fn classic_instance() {
        let it = items(&[(60, 10), (100, 20), (120, 30)]);
        let r = run(&it, 50.0);
        assert_eq!(r.best_value, 220.0);
        assert_eq!(r.selected_indices, vec![1, 2]);
        assert_eq!(r.total_weight, 50.0);
    }

    #[test]
    fn agrees_with_the_dp_solution() {
        let instances: Vec<(Vec<(u64, u64)>, u64)> = vec![
            (vec![(60, 10), (100, 20), (120, 30)], 50),
            (vec![(10, 5), (40, 4), (30, 6), (50, 3)], 10),
            (vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)], 7),
            (vec![(20, 3), (5, 2), (10, 4), (40, 6), (15, 5)], 10),
            (vec![(9, 3), (9, 3), (9, 3), (9, 3)], 9),
        ];

        for (pairs, cap) in instances {
            let it = items(&pairs);
            let bnb = run(&it, cap as f64);
            let dp = knapsack_01(&it, cap);
            assert_eq!(
                bnb.best_value, dp.best_value as f64,
                "mismatch for {pairs:?} at capacity {cap}"
            );

            // The reported selection must actually produce that value.
            let value: u64 = bnb.selected_indices.iter().map(|&i| it[i].value).sum();
            let weight: u64 = bnb.selected_indices.iter().map(|&i| it[i].weight).sum();
            assert_eq!(value as f64, bnb.best_value);
            assert!(weight as f64 <= cap as f64);
        }
    }

    #[test]
    fn nothing_fits() {
        let it = items(&[(100, 50), (80, 40)]);
        let r = run(&it, 10.0);
        assert_eq!(r.best_value, 0.0);
        assert!(r.selected_indices.is_empty());
    }

    #[test]
    fn everything_fits() {
        let it = items(&[(5, 1), (6, 2), (7, 3)]);
        let r = run(&it, 100.0);
        assert_eq!(r.best_value, 18.0);
        assert_eq!(r.selected_indices, vec![0, 1, 2]);
    }

    #[test]
    fn handles_huge_capacity_without_a_table() {
        // A DP table at this capacity would be far too large to allocate.
        let it = items(&[(10, 1_000_000_000), (20, 2_000_000_000)]);
        let r = run(&it, 3_000_000_000.0);
        assert_eq!(r.best_value, 30.0);
    }

    #[test]
    fn zero_weight_items_are_free_value() {
        let it = items(&[(7, 0), (5, 10)]);
        let r = run(&it, 5.0);
        assert_eq!(r.best_value, 7.0);
        assert_eq!(r.selected_indices, vec![0]);
    }

    #[test]
    fn empty_input_and_zero_capacity() {
        assert_eq!(run(&[], 10.0).best_value, 0.0);
        assert_eq!(run(&items(&[(5, 1)]), 0.0).best_value, 0.0);
    }

    #[test]
    fn rejects_negative_capacity() {
        assert!(knapsack_branch_and_bound(&[], -1.0, &AtomicBool::new(false)).is_err());
    }
}
