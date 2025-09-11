fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // Base case: already sorted
    }

    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec(); // Clone left half
    let mut right = arr[mid..].to_vec(); // Clone right half

    merge_sort(&mut left);  // Recursively sort left
    merge_sort(&mut right); // Recursively sort right

    merge(arr, &left, &right); // Merge the two halves
}

fn merge<T: Ord>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0; // Index for left
    let mut j = 0; // Index for right
    let mut k = 0; // Index for merged array

    // Compare elements and merge
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone(); // Clone element from left
            i += 1;
        } else {
            arr[k] = right[j].clone(); // Clone element from right
            j += 1;
        }
        k += 1;
    }

    // Copy remaining elements (if any)
    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}
