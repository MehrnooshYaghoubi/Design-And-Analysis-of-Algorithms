use std::sync::atomic::{AtomicBool, Ordering};

pub fn merge_sort<T: PartialOrd + Clone>(arr: &mut [T], cancel_flag: &AtomicBool) {
    if arr.len() <= 1 {
        return;
    }

    let mid = arr.len() / 2;

    if cancel_flag.load(Ordering::Relaxed) {
        // Canceling Mechanism
        println!("Cancelled bubble sort!");
        return;
    }
    // Sort the left and right halves first
    merge_sort(&mut arr[..mid], cancel_flag);
    merge_sort(&mut arr[mid..], cancel_flag);

    // Copy sorted halves for merging
    let left_half = arr[..mid].to_vec();
    let right_half = arr[mid..].to_vec();

    merge(arr, &left_half, &right_half);
}

fn merge<T: PartialOrd + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

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
