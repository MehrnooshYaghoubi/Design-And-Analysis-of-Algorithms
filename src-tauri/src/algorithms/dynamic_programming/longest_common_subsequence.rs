//! Longest common subsequence of two strings.
//!
//! Operates on `char` vectors rather than bytes so multi-byte characters are
//! never split. The DP table is materialised when it fits, which lets us
//! recover the subsequence itself; otherwise only the length is reported.

use serde::Serialize;

/// Above this many table cells we compute the length only.
pub const MAX_TABLE_CELLS: usize = 40_000_000;

#[derive(Debug, Clone, Serialize)]
pub struct LcsResult {
    pub length: usize,
    /// The subsequence itself, or `None` when the inputs were too large.
    pub subsequence: Option<String>,
    pub value_only: bool,
}

pub fn longest_common_subsequence(a: &str, b: &str) -> LcsResult {
    let x: Vec<char> = a.chars().collect();
    let y: Vec<char> = b.chars().collect();
    let (n, m) = (x.len(), y.len());

    if n == 0 || m == 0 {
        return LcsResult {
            length: 0,
            subsequence: Some(String::new()),
            value_only: false,
        };
    }

    let cells = (n + 1).saturating_mul(m + 1);
    if cells > MAX_TABLE_CELLS {
        return LcsResult {
            length: length_only(&x, &y),
            subsequence: None,
            value_only: true,
        };
    }

    let mut table = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            table[i][j] = if x[i - 1] == y[j - 1] {
                table[i - 1][j - 1] + 1
            } else {
                table[i - 1][j].max(table[i][j - 1])
            };
        }
    }

    // Walk back from the corner, collecting matches.
    let mut out = Vec::with_capacity(table[n][m]);
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        if x[i - 1] == y[j - 1] {
            out.push(x[i - 1]);
            i -= 1;
            j -= 1;
        } else if table[i - 1][j] >= table[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    out.reverse();

    LcsResult {
        length: table[n][m],
        subsequence: Some(out.into_iter().collect()),
        value_only: false,
    }
}

/// Rolling two-row DP for the length alone.
fn length_only(x: &[char], y: &[char]) -> usize {
    let m = y.len();
    let mut prev = vec![0usize; m + 1];
    let mut cur = vec![0usize; m + 1];

    for &xi in x {
        for j in 1..=m {
            cur[j] = if xi == y[j - 1] {
                prev[j - 1] + 1
            } else {
                prev[j].max(cur[j - 1])
            };
        }
        std::mem::swap(&mut prev, &mut cur);
        cur[0] = 0;
    }
    prev[m]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms `sub` really is a subsequence of `s`.
    fn is_subsequence(sub: &str, s: &str) -> bool {
        let mut it = s.chars();
        sub.chars().all(|c| it.any(|d| d == c))
    }

    #[test]
    fn classic_example() {
        let r = longest_common_subsequence("ABCBDAB", "BDCABA");
        assert_eq!(r.length, 4);
        let sub = r.subsequence.unwrap();
        assert_eq!(sub.chars().count(), 4);
        assert!(is_subsequence(&sub, "ABCBDAB"));
        assert!(is_subsequence(&sub, "BDCABA"));
    }

    #[test]
    fn identical_strings() {
        let r = longest_common_subsequence("hello", "hello");
        assert_eq!(r.length, 5);
        assert_eq!(r.subsequence.as_deref(), Some("hello"));
    }

    #[test]
    fn no_common_characters() {
        let r = longest_common_subsequence("abc", "xyz");
        assert_eq!(r.length, 0);
        assert_eq!(r.subsequence.as_deref(), Some(""));
    }

    #[test]
    fn empty_inputs() {
        assert_eq!(longest_common_subsequence("", "abc").length, 0);
        assert_eq!(longest_common_subsequence("abc", "").length, 0);
        assert_eq!(longest_common_subsequence("", "").length, 0);
    }

    #[test]
    fn reconstruction_is_a_real_subsequence() {
        let a = "AGGTAB";
        let b = "GXTXAYB";
        let r = longest_common_subsequence(a, b);
        assert_eq!(r.length, 4); // GTAB
        let sub = r.subsequence.unwrap();
        assert!(is_subsequence(&sub, a));
        assert!(is_subsequence(&sub, b));
    }

    #[test]
    fn handles_multibyte_characters() {
        // Byte-level slicing would corrupt these.
        let r = longest_common_subsequence("héllo wörld", "héllo thörn");
        let sub = r.subsequence.unwrap();
        assert!(sub.contains('é'));
        assert!(sub.contains('ö'));
        assert!(is_subsequence(&sub, "héllo wörld"));
    }

    #[test]
    fn rolling_length_matches_table() {
        let a: Vec<char> = "ABCBDABXYZQ".chars().collect();
        let b: Vec<char> = "BDCABAZQXY".chars().collect();
        let table = longest_common_subsequence("ABCBDABXYZQ", "BDCABAZQXY").length;
        assert_eq!(length_only(&a, &b), table);
    }
}
