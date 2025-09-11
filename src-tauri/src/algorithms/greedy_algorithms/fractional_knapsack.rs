use crate::types::Item;

// Fractional Knapsack: maximize value with capacity limit
pub fn fractional_knapsack(mut items: Vec<Item>, capacity: f64) -> f64 {
    // Sort items by value-to-weight ratio descending
    items.sort_by(|a, b| {
        let r1 = a.value / a.weight;
        let r2 = b.value / b.weight;
        r2.partial_cmp(&r1).unwrap()
    });

    let mut remaining_capacity = capacity;
    let mut total_value = 0.0;

    for item in items {
        if remaining_capacity == 0.0 {
            break;
        }
        if item.weight <= remaining_capacity {
            // Take full item
            remaining_capacity -= item.weight;
            total_value += item.value;
        } else {
            // Take fraction of item
            let fraction = remaining_capacity / item.weight;
            total_value += item.value * fraction;
            remaining_capacity = 0.0;
        }
    }

    total_value
}
