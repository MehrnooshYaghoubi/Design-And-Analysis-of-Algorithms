use std::sync::atomic::{AtomicBool, Ordering};

fn counting_sort(arr: &mut Vec<i32>, exp: i32) {
    let n = arr.len();
    let mut output = vec![0; n];
    let mut count = vec![0; 10];

    for &num in arr.iter() {
        let digit = ((num / exp) % 10) as usize;
        count[digit] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        let digit = ((num / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = num;
    }

    arr.clone_from_slice(&output);
}

pub fn radix_sort(arr: &mut Vec<i32>, cancel_flag: &AtomicBool) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap();

    let mut exp = 1;
    while max / exp > 0 {
        if cancel_flag.load(Ordering::Relaxed) {
            // Canceling Mechanism
            println!("Cancelled bubble sort!");
            return;
        }
        counting_sort(arr, exp);
        exp *= 10;
    }
}
