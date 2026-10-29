//! Longest strictly increasing subsequence in O(n log n).
//!
//! Patience sorting: `tails[k]` holds the smallest possible tail value of an
//! increasing subsequence of length `k+1`. Each element replaces the first
//! tail that is >= itself, found by binary search. Predecessor links let us
//! rebuild the actual subsequence, which the naive O(n^2) table cannot do in
//! less memory.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct LisResult {
    pub length: usize,
    /// The subsequence values, in order.
    pub subsequence: Vec<i64>,
    /// Indices in the original array that the subsequence came from.
    pub indices: Vec<usize>,
}

pub fn longest_increasing_subsequence(nums: &[i64]) -> LisResult {
    if nums.is_empty() {
        return LisResult {
            length: 0,
            subsequence: Vec::new(),
            indices: Vec::new(),
        };
    }

    // `tail_idx[k]` is the index of the current tail of a length-(k+1) run.
    let mut tail_idx: Vec<usize> = Vec::with_capacity(nums.len());
    // `prev[i]` is the index preceding i in the best run ending at i.
    let mut prev: Vec<Option<usize>> = vec![None; nums.len()];

    for i in 0..nums.len() {
        // First position whose tail value is >= nums[i]; strictly increasing,
        // so equal values must displace rather than extend.
        let pos = tail_idx.partition_point(|&t| nums[t] < nums[i]);

        prev[i] = if pos == 0 { None } else { Some(tail_idx[pos - 1]) };

        if pos == tail_idx.len() {
            tail_idx.push(i);
        } else {
            tail_idx[pos] = i;
        }
    }

    // Follow the predecessor chain back from the final tail.
    let mut indices = Vec::with_capacity(tail_idx.len());
    let mut cur = Some(*tail_idx.last().expect("non-empty input"));
    while let Some(i) = cur {
        indices.push(i);
        cur = prev[i];
    }
    indices.reverse();

    let subsequence = indices.iter().map(|&i| nums[i]).collect();

    LisResult {
        length: tail_idx.len(),
        subsequence,
        indices,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_well_formed(r: &LisResult, nums: &[i64]) {
        assert_eq!(r.length, r.subsequence.len());
        assert_eq!(r.length, r.indices.len());
        // Indices ascend and address the right values.
        for w in r.indices.windows(2) {
            assert!(w[0] < w[1], "indices must ascend");
        }
        for (k, &i) in r.indices.iter().enumerate() {
            assert_eq!(nums[i], r.subsequence[k]);
        }
        // Values strictly ascend.
        for w in r.subsequence.windows(2) {
            assert!(w[0] < w[1], "subsequence must strictly increase");
        }
    }

    #[test]
    fn classic_example() {
        let nums = [10, 9, 2, 5, 3, 7, 101, 18];
        let r = longest_increasing_subsequence(&nums);
        assert_eq!(r.length, 4); // e.g. 2, 3, 7, 18
        assert_well_formed(&r, &nums);
    }

    #[test]
    fn already_sorted() {
        let nums = [1, 2, 3, 4, 5];
        let r = longest_increasing_subsequence(&nums);
        assert_eq!(r.length, 5);
        assert_eq!(r.subsequence, vec![1, 2, 3, 4, 5]);
        assert_well_formed(&r, &nums);
    }

    #[test]
    fn strictly_decreasing_gives_one() {
        let nums = [5, 4, 3, 2, 1];
        let r = longest_increasing_subsequence(&nums);
        assert_eq!(r.length, 1);
        assert_well_formed(&r, &nums);
    }

    #[test]
    fn duplicates_do_not_count_as_increasing() {
        let nums = [2, 2, 2, 2];
        let r = longest_increasing_subsequence(&nums);
        assert_eq!(r.length, 1);
        assert_well_formed(&r, &nums);
    }

    #[test]
    fn mixed_duplicates() {
        let nums = [1, 3, 3, 4, 2, 5];
        let r = longest_increasing_subsequence(&nums);
        assert_eq!(r.length, 4); // 1, 3, 4, 5
        assert_well_formed(&r, &nums);
    }

    #[test]
    fn empty_and_single() {
        let r = longest_increasing_subsequence(&[]);
        assert_eq!(r.length, 0);
        assert!(r.subsequence.is_empty());

        let nums = [42];
        let r = longest_increasing_subsequence(&nums);
        assert_eq!(r.length, 1);
        assert_eq!(r.subsequence, vec![42]);
        assert_well_formed(&r, &nums);
    }

    #[test]
    fn handles_negatives() {
        let nums = [-5, -3, -4, 0, -1, 2];
        let r = longest_increasing_subsequence(&nums);
        assert_eq!(r.length, 4); // -5, -4, -1, 2
        assert_well_formed(&r, &nums);
    }

    #[test]
    fn reconstruction_is_correct_not_just_long() {
        // A greedy "collect every tail" bug would return 2,3,7,18 wrongly
        // ordered or mismatched indices here.
        let nums = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let r = longest_increasing_subsequence(&nums);
        assert_eq!(r.length, 4); // e.g. 1, 4, 5, 9
        assert_well_formed(&r, &nums);
    }
}
