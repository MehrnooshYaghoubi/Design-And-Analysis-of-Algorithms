fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return; // Base case: already sorted
    }

    let pivot_index = partition(arr);
    let (left, right) = arr.split_at_mut(pivot_index);
    
    quick_sort(left);          // Recursively sort the left part
    quick_sort(&mut right[1..]); // Recursively sort the right part (excluding pivot)
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2; // Choose middle element as pivot
    arr.swap(pivot_index, arr.len() - 1); // Move pivot to the end
    
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] { // Compare with pivot
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, arr.len() - 1); // Move pivot to its final position
    i
}
