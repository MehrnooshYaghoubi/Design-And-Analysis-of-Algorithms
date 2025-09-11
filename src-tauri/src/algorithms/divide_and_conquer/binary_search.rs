pub fn binary_search<T: PartialOrd>(arr: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match arr[mid].partial_cmp(target) {
            Some(std::cmp::Ordering::Equal) => return Some(mid),
            Some(std::cmp::Ordering::Less) => left = mid + 1,
            Some(std::cmp::Ordering::Greater) => right = mid,
            None => {
                // Handle NaN or incomparable cases: treat as greater (skip) or less
                // Here, treat as greater so go left
                right = mid;
            }
        }
    }

    None
}
