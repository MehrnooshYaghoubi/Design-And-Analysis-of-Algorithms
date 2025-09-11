use algorithms::{
    divide_and_conquer::{binary_search, closest_pair},
    graph_algorithms::{bellman_ford, bfs, dfs, dijkstra},
    greedy_algorithms::{
        activity_selection, build_tree, fractional_knapsack::fractional_knapsack, generate_codes,
        kruskals_algorithm::kruskal, prim,
    },
    sorting_algorithms::{
        bubble_sort, bucket_sort, heap_sort, insertion_sort, merge_sort, quick_sort,
        quick_sort_3way, radix_sort, selection_sort,
    },
};
use serde::Deserialize;
use serde_json::Value;
use std::thread;
use std::{collections::HashMap, time::Instant};
use tauri::{AppHandle, Emitter, Window};
pub mod algorithms;
pub mod converts;
pub mod generators;
pub mod types;
use crate::{algorithms::sorting_algorithms::bucket_sort::BucketSortKey, types::*};

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::State;

struct CancelFlag(Arc<AtomicBool>);

macro_rules! dispatch_sort {
    (
        $json:expr,
        $algo:expr,
        $cancel_flag:expr,
        $(
            $ty:ty => {
                $(
                    $name:literal => $func:expr
                ),*
            }
        ),*
    ) => {{
        let ty = $json.get("type")
            .and_then(|v| v.as_str())
            .ok_or("Missing 'type'")?;

        match ty {
            $(
                stringify!($ty) => {
                    let arr_val = $json.get("arr")
                        .ok_or("Missing 'arr' field")?;
                    let mut arr: Vec<$ty> = converts::convert_json_to_vec::<$ty>(arr_val)
                        .ok_or("Failed to parse array")?;

                    match $algo {
                        $(
                            $name => {
                                // Pass cancel flag here
                                $func(&mut arr, $cancel_flag);
                                Ok(format!("{:?}", arr))
                            },
                        )*
                        _ => Err("Unknown sorting algorithm".to_string()),
                    }
                },
            )*
            _ => Err("Unsupported type".to_string()),
        }
    }};
}

#[tauri::command]
fn execute_json(
    json_data: Value,
    selected_algo: String,
    cancel_flag: State<'_, CancelFlag>,
) -> Result<String, String> {
    let cancel_flag = cancel_flag.0.clone();

    // spawn if you want async, but here I’ll show sync version first
    // so results are properly returned
    println!("Received JSON: {:?}", json_data);

    // Register algorithm handlers
    let mut handlers: HashMap<&'static str, AlgoHandler> = HashMap::new();
    handlers.insert("radix sort", handle_radix_sort);
    handlers.insert("fractional knapsack", handle_fractional_knapsack);
    handlers.insert("huffman coding", handle_huffman);
    handlers.insert("kruskals algorithm", handle_kruskal);
    handlers.insert("prims algorithm", handle_prim);
    handlers.insert("bellman ford", handle_bellman_ford);
    handlers.insert("breadth first search", handle_bfs);
    handlers.insert("depth first search", handle_dfs);
    handlers.insert("dijkstras algorithm", handle_dijkstra);
    handlers.insert("activity selection", handle_activity_selection);
    handlers.insert("binary search", handle_binary_search);
    handlers.insert("closest pair of points", handle_closest_pair);

    // Sorting algos
    let sorting_algos = [
        "bubble sort",
        "bucket sort",
        "heap sort",
        "insertion sort",
        "merge sort",
        "quick sort 3 partition",
        "quick sort",
        "selection sort",
    ];

    if sorting_algos.contains(&selected_algo.as_str()) {
        return handle_sort(json_data, &selected_algo, &cancel_flag);
    }

    if let Some(handler) = handlers.get(selected_algo.as_str()) {
        return handler(json_data);
    }

    Err("Wrong Algorithm Selected".into())
}

// handling algorithms

fn handle_sort(json_data: Value, algo: &str, cancel_flag: &AtomicBool) -> Result<String, String> {
    dispatch_sort!(
        json_data, algo, cancel_flag,
        i32 => {
            "bubble sort" => bubble_sort::<i32>,
            "bucket sort" => bucket_sort::<i32>,
            "heap sort" => heap_sort::<i32>,
            "insertion sort" => insertion_sort::<i32>,
            "merge sort" => merge_sort::<i32>,
            "quick sort 3 partition" => quick_sort_3way::<i32>,
            "quick sort" => quick_sort::<i32>,
            "selection sort" => selection_sort::<i32>
        },
        f64 => {
            "bubble sort" => bubble_sort::<f64>,
            "bucket sort" => bucket_sort::<f64>,
            "heap sort" => heap_sort::<f64>,
            "insertion sort" => insertion_sort::<f64>,
            "merge sort" => merge_sort::<f64>,
            "quick sort 3 partition" => quick_sort_3way::<f64>,
            "quick sort" => quick_sort::<f64>,
            "selection sort" => selection_sort::<f64>
        },
        String => {
            "bubble sort" => bubble_sort::<String>,
            "bucket sort" => bucket_sort::<String>,
            "heap sort" => heap_sort::<String>,
            "insertion sort" => insertion_sort::<String>,
            "merge sort" => merge_sort::<String>,
            "quick sort 3 partition" => quick_sort_3way::<String>,
            "quick sort" => quick_sort::<String>,
            "selection sort" => selection_sort::<String>
        }
    )
}

fn handle_radix_sort(json_data: Value) -> Result<String, String> {
    let arr_val = json_data
        .get("arr")
        .ok_or("Missing or invalid 'arr' field")?;
    let arr = converts::convert_json_to_vec::<i32>(arr_val)
        .ok_or("Failed to parse array for radix sort")?;

    // Filter out negative values
    let mut arr: Vec<i32> = arr.into_iter().filter(|&x| x >= 0).collect();

    // Create a dummy cancel flag
    let dummy_flag = AtomicBool::new(false);

    // Call radix_sort with the dummy flag
    radix_sort(&mut arr, &dummy_flag);

    Ok(format!("{:?}", arr))
}

fn handle_fractional_knapsack(json_data: Value) -> Result<String, String> {
    let input: KnapsackInput =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;
    let result = fractional_knapsack(input.items, input.capacity);
    Ok(format!("{:?}", result))
}

fn handle_huffman(json_data: Value) -> Result<String, String> {
    let freq_map: HashMap<String, usize> =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;
    let converted: HashMap<char, usize> = freq_map
        .into_iter()
        .filter_map(|(k, v)| k.chars().next().map(|ch| (ch, v)))
        .collect();

    if let Some(tree) = build_tree(&converted) {
        let mut codes = HashMap::new();
        generate_codes(&tree, String::new(), &mut codes);
        Ok(format!("Huffman Codes: {:?}", codes))
    } else {
        Err("Huffman Failed".into())
    }
}

fn handle_kruskal(json_data: Value) -> Result<String, String> {
    let input: GraphInput =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;
    let mst = kruskal(input.num_vertices, input.edges);
    Ok(format!("{:?}", mst))
}

fn handle_prim(json_data: Value) -> Result<String, String> {
    let input: GraphInput =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;
    let mst = prim(input.num_vertices, input.edges);
    Ok(format!("{:?}", mst))
}

fn handle_bellman_ford(json_data: Value) -> Result<String, String> {
    let input: BellmanFordInput =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;

    match bellman_ford(&input.graph, input.start) {
        Ok(distances) => Ok(format!("{:?}", distances)),
        Err(e) => Err(e.into()),
    }
}

fn handle_bfs(json_data: Value) -> Result<String, String> {
    let input: TraversalInput =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;

    let order = bfs(&input.graph, input.start);
    Ok(format!("{:?}", order))
}

fn handle_dfs(json_data: Value) -> Result<String, String> {
    let input: TraversalInput =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;

    let mut visited = vec![false; input.graph.len()];
    let mut order = Vec::new();

    dfs(&input.graph, input.start, &mut visited, &mut order);
    Ok(format!("{:?}", order))
}

fn handle_dijkstra(json_data: Value) -> Result<String, String> {
    let input: InputDijkstra =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;

    // Convert input edges to your Edge struct (if different, here they're the same)
    let adj_list: Vec<Vec<DijkstraEdge>> = input
        .adj_list
        .into_iter()
        .map(|vec_edge_input| {
            vec_edge_input
                .into_iter()
                .map(|e| DijkstraEdge {
                    node: e.node,
                    cost: e.cost,
                })
                .collect()
        })
        .collect();

    let dist = dijkstra(&adj_list, input.start);
    Ok(format!("{:?}", dist))
}

fn handle_activity_selection(json_data: Value) -> Result<String, String> {
    let mut activities: Vec<Activity> =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;
    let selected = activity_selection(&mut activities);
    Ok(format!("{:?}", selected))
}

fn handle_binary_search(json_data: Value) -> Result<String, String> {
    #[derive(serde::Deserialize)]
    struct InputType {
        #[serde(rename = "type")]
        typ: String,
    }

    // First, extract the "type" field only
    let input_type: InputType =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;

    match input_type.typ.as_str() {
        "i32" => {
            #[derive(serde::Deserialize)]
            struct InputBS {
                arr: Vec<i32>,
                target: i32,
            }
            let input: InputBS = serde_json::from_value(json_data).map_err(|_| "Invalid JSON")?;
            let result = binary_search(&input.arr, &input.target)
                .map(|idx| idx as i32)
                .unwrap_or(-1);
            Ok(format!("{}", result))
        }
        "f64" => {
            #[derive(serde::Deserialize)]
            struct InputBS {
                arr: Vec<f64>,
                target: f64,
            }
            let input: InputBS = serde_json::from_value(json_data).map_err(|_| "Invalid JSON")?;
            let result = binary_search(&input.arr, &input.target)
                .map(|idx| idx as i32)
                .unwrap_or(-1);
            Ok(format!("{}", result))
        }
        "string" | "String" => {
            #[derive(serde::Deserialize)]
            struct InputBS {
                arr: Vec<String>,
                target: String,
            }
            let input: InputBS = serde_json::from_value(json_data).map_err(|_| "Invalid JSON")?;
            let result = binary_search(&input.arr, &input.target)
                .map(|idx| idx as i32)
                .unwrap_or(-1);
            Ok(format!("{}", result))
        }
        other => Err(format!("Unsupported type: {}", other).into()),
    }
}

fn handle_closest_pair(json_data: Value) -> Result<String, String> {
    let points: Vec<Point> =
        serde_json::from_value(json_data.clone()).map_err(|_| "Invalid JSON")?;

    match closest_pair(&points) {
        Some((p1, p2, dist)) => Ok(format!(
            "Closest points: ({:?}), ({:?}), Distance: {:.6}",
            p1, p2, dist
        )),
        None => Err("Need at least two points".into()),
    }
}

// ----- Handeling Generators

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Algorithm::*;
        let s = match self {
            BubbleSort => "bubble sort",
            BucketSort => "bucket sort",
            HeapSort => "heap sort",
            InsertionSort => "insertion sort",
            MergeSort => "merge sort",
            QuickSort => "quick sort",
            QuickSort3Partition => "quick sort 3 partition",
            SelectionSort => "selection sort",
            RadixSort => "radix sort",
        };
        f.write_str(s)
    }
}

fn sort_in_place<
    T: PartialOrd + crate::algorithms::sorting_algorithms::bucket_sort::BucketSortKey + Clone,
>(
    v: &mut Vec<T>,
    algo: Algorithm,
    cancel_flag: &AtomicBool,
) {
    use crate::algorithms::sorting_algorithms::*;
    match algo {
        Algorithm::BubbleSort => bubble_sort(v, cancel_flag),
        Algorithm::BucketSort => bucket_sort(v, cancel_flag),
        Algorithm::HeapSort => heap_sort(v, cancel_flag),
        Algorithm::InsertionSort => insertion_sort(v, cancel_flag),
        Algorithm::MergeSort => merge_sort(v, cancel_flag),
        Algorithm::QuickSort => quick_sort(v, cancel_flag),
        Algorithm::QuickSort3Partition => quick_sort_3way(v, cancel_flag),
        Algorithm::SelectionSort => selection_sort(v, cancel_flag),
        Algorithm::RadixSort => (),
    }
}

fn apply_algorithm(data: &mut RandomData, algo: Algorithm, cancel_flag: &AtomicBool) {
    match data {
        RandomData::I32(v) => {
            if algo == Algorithm::RadixSort {
                radix_sort(v, cancel_flag)
            } else {
                sort_in_place(v, algo, cancel_flag)
            }
        }
        RandomData::F64(v) => sort_in_place(v, algo, cancel_flag),
        RandomData::String(v) => sort_in_place(v, algo, cancel_flag),
    }
}

#[tauri::command]
async fn generate_random(
    app: AppHandle,
    json_data: Value,
    cancel_flag: State<'_, CancelFlag>,
) -> Result<(), String> {
    cancel_flag.0.store(false, Ordering::Relaxed); // reset the cancellation (not cancelled)

    let input: GeneratorStruct =
        serde_json::from_value(json_data).map_err(|e| format!("Invalid JSON: {e}"))?;
    let app_clone = app.clone();
    let cancel_flag = cancel_flag.0.clone();
    tauri::async_runtime::spawn_blocking(move || {
        // generate your payload
        let mut data = crate::generators::generate_random_array(
            input.range,
            input.data_type,
            input.min,
            input.max,
        );

        let start = std::time::Instant::now();
        apply_algorithm(&mut data, input.selected_algorithm, &cancel_flag);
        let dur = start.elapsed().as_nanos() as u64;

        let result = AlgorithmResult {
            name: input.selected_algorithm.to_string(),
            duration_ns: dur,
        };

        if let Err(e) = app_clone.emit("algorithm-finished", result) {
            eprintln!("emit_all failed: {e:?}");
        }
    });

    Ok(())
}

#[tauri::command]
async fn cancel(cancel_flag: State<'_, CancelFlag>) -> Result<(), String> {
    cancel_flag.0.store(true, Ordering::Relaxed);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(CancelFlag(Arc::new(AtomicBool::new(false))))
        .invoke_handler(tauri::generate_handler![
            execute_json,
            generate_random,
            cancel
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
