use std::sync::atomic::{AtomicBool, Ordering};

pub fn bubble_sort<T: PartialOrd>(arr: &mut [T], cancel_flag: &AtomicBool) {
    let n = arr.len();
    for i in 0..n {
        if cancel_flag.load(Ordering::Relaxed) {
            // Canceling Mechanism
            println!("Cancelled bubble sort!");
            return;
        }
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
