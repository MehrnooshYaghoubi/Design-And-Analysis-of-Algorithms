use std::sync::atomic::{AtomicBool, Ordering};

pub fn quick_sort_3way<T: PartialOrd + Clone>(arr: &mut [T], cancel_flag: &AtomicBool) {
    let len = arr.len();
    if len <= 1 {
        return;
    }
    quick_sort_3way_slice(arr, cancel_flag);
}

fn quick_sort_3way_slice<T: PartialOrd + Clone>(arr: &mut [T], cancel_flag: &AtomicBool) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mut lt = 0;
    let mut gt = len - 1;
    let pivot = arr[0].clone();
    let mut i = 1;

    while i <= gt {
        // Use partial_cmp and unwrap_or to handle possible None (NaN cases)
        if cancel_flag.load(Ordering::Relaxed) {
            // Canceling Mechanism
            println!("Cancelled bubble sort!");
            return;
        }
        if arr[i]
            .partial_cmp(&pivot)
            .unwrap_or(std::cmp::Ordering::Greater)
            == std::cmp::Ordering::Less
        {
            arr.swap(lt, i);
            lt += 1;
            i += 1;
        } else if arr[i]
            .partial_cmp(&pivot)
            .unwrap_or(std::cmp::Ordering::Less)
            == std::cmp::Ordering::Greater
        {
            arr.swap(i, gt);
            if gt == 0 {
                break; // prevent underflow
            }
            gt -= 1;
        } else {
            i += 1;
        }
    }

    if cancel_flag.load(Ordering::Relaxed) {
        // Canceling Mechanism
        println!("Cancelled bubble sort!");
        return;
    }

    quick_sort_3way_slice(&mut arr[0..lt], cancel_flag);
    quick_sort_3way_slice(&mut arr[gt + 1..], cancel_flag);
}
