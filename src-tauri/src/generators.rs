use crate::types::*;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

// Random Generators

pub fn generate_random_array(count: usize, data_type: DataType, min: f64, max: f64) -> RandomData {
    let mut rng = rand::rng();

    match data_type {
        DataType::I32 => {
            let data: Vec<i32> = (0..count)
                .map(|_| rng.random_range(min as i32..=max as i32))
                .collect();
            RandomData::I32(data)
        }
        DataType::F64 => {
            let data: Vec<f64> = (0..count).map(|_| rng.random_range(min..=max)).collect();
            RandomData::F64(data)
        }
        DataType::String => {
            let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

            let data: Vec<String> = (0..count)
                .map(|_| {
                    let mut s = String::with_capacity(10);
                    for _ in 0..10 {
                        s.push(charset[rng.random_range(0..charset.len())] as char);
                    }
                    s
                })
                .collect();

            RandomData::String(data)
        }
    }
}

pub fn generate_random_kruskal_input(
    num_vertices: usize,
    num_edges: usize,
    max_weight: usize,
) -> GraphInput {
    let mut rng = rand::rng();
    let mut edges = HashSet::with_capacity(num_edges);
    let mut edge_list = Vec::with_capacity(num_edges);

    let max_possible_edges = (num_vertices * (num_vertices - 1)) / 2;
    let num_edges = num_edges.min(max_possible_edges);

    while edge_list.len() < num_edges {
        let src = rng.random_range(0..num_vertices);
        let mut dest = rng.random_range(0..num_vertices);

        // ensure no self-loops or duplicate edges
        while dest == src || edges.contains(&(src.min(dest), src.max(dest))) {
            dest = rng.random_range(0..num_vertices);
        }

        let weight = rng.random_range(1..=max_weight);

        edges.insert((src.min(dest), src.max(dest)));
        edge_list.push(Edge { src, dest, weight });
    }

    GraphInput {
        num_vertices,
        edges: edge_list,
    }
}

pub fn generate_random_knapsack_input(
    num_items: usize,
    max_value: f64,
    max_weight: f64,
    capacity: f64,
) -> KnapsackInput {
    let mut rng = rand::rng();
    let mut items = Vec::with_capacity(num_items);

    for _ in 0..num_items {
        let value = rng.random_range(1.0..=max_value);
        let weight = rng.random_range(1.0..=max_weight);
        items.push(Item { value, weight });
    }

    KnapsackInput { capacity, items }
}

pub fn generate_random_prim_input(num_vertices: usize, edge_probability: f64) -> GraphInput {
    let mut rng = rand::rng();
    let mut edges = Vec::with_capacity((num_vertices * (num_vertices - 1)) / 2);

    for i in 0..num_vertices {
        for j in (i + 1)..num_vertices {
            if rng.random_bool(edge_probability) {
                let weight = rng.random_range(1..=20);
                edges.push(Edge {
                    src: i,
                    dest: j,
                    weight,
                });
            }
        }
    }

    GraphInput {
        num_vertices,
        edges,
    }
}

pub fn generate_random_activities(
    num_activities: usize,
    max_start: usize,
    max_duration: usize,
) -> Vec<Activity> {
    let mut rng = rand::rng();
    let mut activities = Vec::with_capacity(num_activities);

    for _ in 0..num_activities {
        let start: usize = rng.random_range(0..max_start);
        let duration = rng.random_range(1..=max_duration);
        let finish = start + duration;
        activities.push(Activity { start, finish });
    }

    // Sort activities by finish time
    activities.sort_unstable_by_key(|a| a.finish);

    activities
}

pub fn generate_random_points(count: usize, min: f64, max: f64) -> Vec<Point> {
    let mut rng = rand::rng();
    let mut points = Vec::with_capacity(count);

    for _ in 0..count {
        let x = rng.random_range(min..=max);
        let y = rng.random_range(min..=max);
        points.push(Point { x, y });
    }

    points
}

pub fn generate_random_huffman_input(num_symbols: usize) -> HuffmanInput {
    let mut rng = rand::rng();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();

    assert!(
        num_symbols <= alphabet.len(),
        "num_symbols cannot exceed alphabet size"
    );

    let mut frequencies = HashMap::with_capacity(num_symbols);
    let mut used_indices = std::collections::HashSet::with_capacity(num_symbols);

    while frequencies.len() < num_symbols {
        let idx = rng.random_range(0..alphabet.len());
        if used_indices.insert(idx) {
            let frequency = rng.random_range(1..=50);
            frequencies.insert(alphabet[idx], frequency);
        }
    }

    HuffmanInput { frequencies }
}

pub fn generate_random_points_closest(count: usize, min: f64, max: f64) -> Vec<Point> {
    let mut rng = rand::rng();
    let mut points = Vec::with_capacity(count);

    for _ in 0..count {
        let x = rng.random_range(min..=max);
        let y = rng.random_range(min..=max);
        points.push(Point { x, y });
    }

    points
}
