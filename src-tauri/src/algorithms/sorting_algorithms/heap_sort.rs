use std::sync::atomic::{AtomicBool, Ordering};

pub fn heap_sort<T: PartialOrd>(arr: &mut [T], cancel_flag: &AtomicBool) {
    let n = arr.len();

    // Build max heap
    for i in (0..n / 2).rev() {
        if cancel_flag.load(Ordering::Relaxed) {
            // Canceling Mechanism
            println!("Cancelled bubble sort!");
            return;
        }
        heapify(arr, n, i);
    }

    // Extract elements from heap
    for i in (1..n).rev() {
        if cancel_flag.load(Ordering::Relaxed) {
            // Canceling Mechanism
            println!("Cancelled bubble sort!");
            return;
        }
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: PartialOrd>(arr: &mut [T], heap_size: usize, root_index: usize) {
    let mut largest = root_index;
    let left = 2 * root_index + 1;
    let right = 2 * root_index + 2;

    if left < heap_size {
        if let Some(std::cmp::Ordering::Less) = arr[largest].partial_cmp(&arr[left]) {
            largest = left;
        }
    }

    if right < heap_size {
        if let Some(std::cmp::Ordering::Less) = arr[largest].partial_cmp(&arr[right]) {
            largest = right;
        }
    }

    if largest != root_index {
        arr.swap(root_index, largest);
        heapify(arr, heap_size, largest);
    }
}
