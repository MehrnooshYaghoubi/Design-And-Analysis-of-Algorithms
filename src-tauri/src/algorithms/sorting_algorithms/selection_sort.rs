use std::sync::atomic::{AtomicBool, Ordering};

pub fn selection_sort<T: PartialOrd + Clone>(collection: &mut Vec<T>, cancel_flag: &AtomicBool) {
    let length = collection.len();

    for i in 0..length.saturating_sub(1) {
        if cancel_flag.load(Ordering::Relaxed) {
            // Canceling Mechanism
            println!("Cancelled bubble sort!");
            return;
        }
        let mut least = i;
        for k in (i + 1)..length {
            if let Some(true) = collection[k]
                .partial_cmp(&collection[least])
                .map(|o| o.is_lt())
            {
                least = k;
            }
        }
        collection.swap(i, least);
    }
}
