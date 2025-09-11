use std::sync::atomic::{AtomicBool, Ordering};

pub trait BucketSortKey {
    fn bucket_key(&self, max_val: &Self) -> f64;
    fn max_value(slice: &[Self]) -> Option<Self>
    where
        Self: Sized;
}

impl BucketSortKey for i32 {
    fn bucket_key(&self, max_val: &i32) -> f64 {
        (*self as f64) / ((*max_val as f64) + 1.0)
    }
    fn max_value(slice: &[i32]) -> Option<i32> {
        slice.iter().cloned().max()
    }
}

impl BucketSortKey for f64 {
    fn bucket_key(&self, max_val: &f64) -> f64 {
        *self / (*max_val + 1.0)
    }
    fn max_value(slice: &[f64]) -> Option<f64> {
        slice
            .iter()
            .cloned()
            .fold(None, |max, x| Some(max.map_or(x, |m| m.max(x))))
    }
}

impl BucketSortKey for String {
    fn bucket_key(&self, _max_val: &String) -> f64 {
        // Use first char ASCII to normalize between 0.0 and 1.0
        let ch = self.chars().next().unwrap_or('a').to_ascii_lowercase();
        ((ch as u8).saturating_sub(b'a') as f64) / 26.0
    }
    fn max_value(_slice: &[String]) -> Option<String> {
        None // not needed here
    }
}
pub fn bucket_sort<T>(arr: &mut Vec<T>, cancel_flag: &AtomicBool)
where
    T: BucketSortKey + PartialOrd + Clone,
{
    let n = arr.len();
    if n == 0 {
        return;
    }

    let max_val = T::max_value(arr).unwrap_or_else(|| arr[0].clone());

    let mut buckets: Vec<Vec<T>> = vec![Vec::new(); n];

    for value in arr.iter() {
        if cancel_flag.load(Ordering::Relaxed) {
            // Canceling Mechanism
            println!("Cancelled bubble sort!");
            return;
        }
        let mut idx = (value.bucket_key(&max_val) * n as f64) as usize;
        if idx >= n {
            idx = n - 1;
        }
        buckets[idx].push(value.clone());
    }

    for bucket in buckets.iter_mut() {
        if cancel_flag.load(Ordering::Relaxed) {
            // Canceling Mechanism
            println!("Cancelled bubble sort!");
            return;
        }
        // Use sort_by with partial_cmp for types like f64
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    }

    let mut i = 0;
    for bucket in buckets {
        if cancel_flag.load(Ordering::Relaxed) {
            // Canceling Mechanism
            println!("Cancelled bubble sort!");
            return;
        }
        for val in bucket {
            arr[i] = val;
            i += 1;
        }
    }
}
