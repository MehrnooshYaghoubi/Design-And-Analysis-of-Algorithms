//! Tauri command surface for KlugNetz.
//!
//! Two execution paths:
//!
//! * `execute_json` runs one algorithm against a JSON payload typed into the
//!   editor and returns a structured result.
//! * `generate_random` builds a dataset in Rust, sorts it, and reports only
//!   the elapsed time. The data never crosses into JavaScript, which is what
//!   makes benchmarking large inputs viable.
//!
//! Results are deliberately *summarised* rather than dumped. A million-element
//! sort returns a head/tail preview plus a verified `sorted` flag, not a
//! million-element string. Formatting and shipping that string was the single
//! largest source of lag in the previous version.

use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{AppHandle, Emitter, State};

pub mod algorithms;
pub mod converts;
pub mod generators;
pub mod types;

use crate::types::*;
use algorithms::{
    backtracking::{hamiltonian_cycle, n_queens, solve_sudoku, subset_sum},
    branch_and_bound::{knapsack_branch_and_bound, tsp_branch_and_bound},
    divide_and_conquer::{binary_search, closest_pair},
    dynamic_programming::{
        floyd_warshall, knapsack_01, longest_common_subsequence,
        longest_increasing_subsequence, matrix_chain_order,
    },
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

/// How many elements from each end of a sorted array we send to the UI.
const PREVIEW_LEN: usize = 50;

/// Upper bound on arrays generated *into the editor*. The editor has to
/// syntax-highlight whatever it holds, so a million elements would freeze the
/// window. Large benchmarks belong on the Compare page, where the data stays
/// in Rust.
const MAX_EDITOR_ARRAY: usize = 10_000;

struct CancelFlag(Arc<AtomicBool>);

/// Signature for every algorithm handler. Handlers receive the cancel flag so
/// the exponential ones can bail out, and return JSON so the frontend gets
/// structured data instead of a `{:?}` string.
type Handler = fn(Value, &AtomicBool) -> Result<Value, String>;

// ---------------------------------------------------------------------------
// Sorting
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct SortOutput {
    algorithm: String,
    length: usize,
    /// Verified after the fact, so a broken sort cannot quietly claim success.
    sorted: bool,
    cancelled: bool,
    /// True when the array was longer than the preview window.
    truncated: bool,
    head: Vec<Value>,
    tail: Vec<Value>,
}

fn is_sorted<T: PartialOrd>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

/// Summarises a sorted array: a head/tail preview, never the whole thing.
fn summarise<T>(name: &str, arr: &[T], cancelled: bool) -> Result<Value, String>
where
    T: PartialOrd + Serialize,
{
    let n = arr.len();
    let truncated = n > PREVIEW_LEN * 2;

    let to_values = |slice: &[T]| -> Result<Vec<Value>, String> {
        slice
            .iter()
            .map(|v| serde_json::to_value(v).map_err(|e| e.to_string()))
            .collect()
    };

    let (head, tail) = if truncated {
        (
            to_values(&arr[..PREVIEW_LEN])?,
            to_values(&arr[n - PREVIEW_LEN..])?,
        )
    } else {
        (to_values(arr)?, Vec::new())
    };

    serde_json::to_value(SortOutput {
        algorithm: name.to_string(),
        length: n,
        sorted: is_sorted(arr),
        cancelled,
        truncated,
        head,
        tail,
    })
    .map_err(|e| e.to_string())
}

/// Normalises the `type` tag so `"string"`, `"String"` and `"STRING"` all work.
fn data_type_of(json: &Value) -> Result<String, String> {
    let raw = json
        .get("type")
        .and_then(|v| v.as_str())
        .ok_or("Missing \"type\" field (expected \"i32\", \"f64\" or \"String\")")?;
    match raw.to_ascii_lowercase().as_str() {
        "i32" | "int" | "integer" => Ok("i32".into()),
        "f64" | "float" | "double" => Ok("f64".into()),
        "string" | "str" => Ok("String".into()),
        other => Err(format!(
            "Unsupported type \"{other}\"; expected \"i32\", \"f64\" or \"String\""
        )),
    }
}

/// Runs one sorting algorithm over a typed array from the payload.
fn handle_sort(json: Value, algo: &str, cancel: &AtomicBool) -> Result<Value, String> {
    let ty = data_type_of(&json)?;
    let arr_val = json.get("arr").ok_or("Missing \"arr\" field")?;

    macro_rules! run {
        ($t:ty) => {{
            let mut arr: Vec<$t> = converts::convert_json_to_vec::<$t>(arr_val)
                .ok_or_else(|| format!("Could not read \"arr\" as a list of {}", stringify!($t)))?;
            match algo {
                "bubble sort" => bubble_sort(&mut arr, cancel),
                "bucket sort" => bucket_sort(&mut arr, cancel),
                "heap sort" => heap_sort(&mut arr, cancel),
                "insertion sort" => insertion_sort(&mut arr, cancel),
                "merge sort" => merge_sort(&mut arr, cancel),
                "quick sort" => quick_sort(&mut arr, cancel),
                "quick sort 3 partition" => quick_sort_3way(&mut arr, cancel),
                "selection sort" => selection_sort(&mut arr, cancel),
                other => return Err(format!("Unknown sorting algorithm \"{other}\"")),
            }
            summarise(algo, &arr, cancel.load(Ordering::Relaxed))
        }};
    }

    match ty.as_str() {
        "i32" => run!(i32),
        "f64" => run!(f64),
        "String" => run!(String),
        _ => unreachable!("data_type_of validated the tag"),
    }
}

fn handle_radix_sort(json: Value, cancel: &AtomicBool) -> Result<Value, String> {
    let arr_val = json.get("arr").ok_or("Missing \"arr\" field")?;
    let arr = converts::convert_json_to_vec::<i32>(arr_val)
        .ok_or("Could not read \"arr\" as a list of integers")?;

    // Radix sort here is digit-based and unsigned. Rather than silently
    // dropping negatives as the old version did, split them out, sort their
    // magnitudes, and stitch the halves back together.
    let (negative, mut positive): (Vec<i32>, Vec<i32>) = arr.into_iter().partition(|&x| x < 0);

    let mut magnitudes: Vec<i32> = negative.iter().map(|&x| -x).collect();
    radix_sort(&mut magnitudes, cancel);
    radix_sort(&mut positive, cancel);

    // Descending magnitudes become ascending negatives.
    let mut out: Vec<i32> = magnitudes.into_iter().rev().map(|x| -x).collect();
    out.extend(positive);

    summarise("radix sort", &out, cancel.load(Ordering::Relaxed))
}

// ---------------------------------------------------------------------------
// Greedy
// ---------------------------------------------------------------------------

fn handle_fractional_knapsack(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: KnapsackInput = parse(json)?;
    let total = fractional_knapsack(input.items, input.capacity);
    Ok(json!({ "capacity": input.capacity, "best_value": total }))
}

fn handle_huffman(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let freq_map: HashMap<String, usize> = parse(json)?;
    if freq_map.is_empty() {
        return Err("Provide at least one symbol frequency".into());
    }

    let mut converted: HashMap<char, usize> = HashMap::with_capacity(freq_map.len());
    for (k, v) in freq_map {
        let ch = k
            .chars()
            .next()
            .ok_or("Symbol keys must not be empty strings")?;
        converted.insert(ch, v);
    }

    let tree = build_tree(&converted).ok_or("Could not build a Huffman tree")?;
    let mut codes = HashMap::new();
    generate_codes(&tree, String::new(), &mut codes);

    // A single distinct symbol yields an empty code; give it one bit.
    if codes.len() == 1 {
        for code in codes.values_mut() {
            if code.is_empty() {
                code.push('0');
            }
        }
    }

    let total_bits: usize = codes
        .iter()
        .map(|(ch, code)| code.len() * converted.get(ch).copied().unwrap_or(0))
        .sum();

    let mut table: Vec<Value> = codes
        .iter()
        .map(|(ch, code)| {
            json!({
                "symbol": ch.to_string(),
                "frequency": converted.get(ch).copied().unwrap_or(0),
                "code": code,
                "bits": code.len(),
            })
        })
        .collect();
    table.sort_by_key(|v| v["symbol"].as_str().unwrap_or("").to_string());

    Ok(json!({
        "symbol_count": table.len(),
        "encoded_bits": total_bits,
        "codes": table,
    }))
}

fn handle_kruskal(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: GraphInput = parse(json)?;
    let mst = kruskal(input.num_vertices, input.edges);
    let total: usize = mst.iter().map(|e| e.weight).sum();
    Ok(json!({
        "vertex_count": input.num_vertices,
        "edge_count": mst.len(),
        "total_weight": total,
        "spans_all_vertices": input.num_vertices == 0 || mst.len() + 1 == input.num_vertices,
        "edges": mst.iter().map(|e| json!({
            "src": e.src, "dest": e.dest, "weight": e.weight
        })).collect::<Vec<_>>(),
    }))
}

fn handle_prim(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: GraphInput = parse(json)?;
    let mst = prim(input.num_vertices, input.edges);
    let total: usize = mst.iter().map(|e| e.weight).sum();
    Ok(json!({
        "vertex_count": input.num_vertices,
        "edge_count": mst.len(),
        "total_weight": total,
        "spans_all_vertices": input.num_vertices == 0 || mst.len() + 1 == input.num_vertices,
        "edges": mst.iter().map(|e| json!({
            "src": e.src, "dest": e.dest, "weight": e.weight
        })).collect::<Vec<_>>(),
    }))
}

fn handle_activity_selection(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let mut activities: Vec<Activity> = parse(json)?;
    let selected = activity_selection(&mut activities);
    Ok(json!({
        "considered": activities.len(),
        "selected_count": selected.len(),
        "selected": selected.iter().map(|a| json!({
            "start": a.start, "finish": a.finish
        })).collect::<Vec<_>>(),
    }))
}

// ---------------------------------------------------------------------------
// Graph
// ---------------------------------------------------------------------------

fn handle_bellman_ford(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: BellmanFordInput = parse(json)?;
    let result = bellman_ford(input.vertex_count, input.start, &input.edges)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn handle_bfs(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: TraversalInput = parse(json)?;
    validate_adjacency(&input.graph, input.start)?;
    let order = bfs(&input.graph, input.start);
    Ok(json!({
        "start": input.start,
        "visited_count": order.len(),
        "order": order,
    }))
}

fn handle_dfs(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: TraversalInput = parse(json)?;
    validate_adjacency(&input.graph, input.start)?;
    let mut visited = vec![false; input.graph.len()];
    let mut order = Vec::new();
    dfs(&input.graph, input.start, &mut visited, &mut order);
    Ok(json!({
        "start": input.start,
        "visited_count": order.len(),
        "order": order,
    }))
}

fn handle_dijkstra(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: InputDijkstra = parse(json)?;
    let n = input.adj_list.len();
    if input.start >= n {
        return Err(format!("Start vertex {} is outside 0..{n}", input.start));
    }
    for (i, edges) in input.adj_list.iter().enumerate() {
        for e in edges {
            if e.node >= n {
                return Err(format!(
                    "Edge from {i} points at vertex {}, outside 0..{n}",
                    e.node
                ));
            }
        }
    }

    let dist = dijkstra(&input.adj_list, input.start);
    // usize::MAX is the "unreachable" sentinel; surface it as null.
    let readable: Vec<Value> = dist
        .iter()
        .map(|&d| {
            if d == usize::MAX {
                Value::Null
            } else {
                json!(d)
            }
        })
        .collect();

    Ok(json!({
        "start": input.start,
        "reachable_count": dist.iter().filter(|&&d| d != usize::MAX).count(),
        "dist": readable,
    }))
}

/// Shared validation for the adjacency-list traversals, which index directly
/// and would otherwise panic on an out-of-range neighbour.
fn validate_adjacency(graph: &[Vec<usize>], start: usize) -> Result<(), String> {
    let n = graph.len();
    if n == 0 {
        return Err("Graph must have at least one vertex".into());
    }
    if start >= n {
        return Err(format!("Start vertex {start} is outside 0..{n}"));
    }
    for (i, neighbours) in graph.iter().enumerate() {
        for &v in neighbours {
            if v >= n {
                return Err(format!(
                    "Vertex {i} lists neighbour {v}, outside 0..{n}"
                ));
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Divide and conquer
// ---------------------------------------------------------------------------

fn handle_binary_search(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let ty = data_type_of(&json)?;
    let arr_val = json.get("arr").ok_or("Missing \"arr\" field")?;
    let target_val = json.get("target").ok_or("Missing \"target\" field")?;

    macro_rules! search {
        ($t:ty) => {{
            let arr: Vec<$t> = converts::convert_json_to_vec::<$t>(arr_val)
                .ok_or_else(|| format!("Could not read \"arr\" as a list of {}", stringify!($t)))?;
            let target = <$t as converts::FromJsonValue>::from_json_value(target_val)
                .ok_or_else(|| format!("Could not read \"target\" as {}", stringify!($t)))?;
            // Binary search is only meaningful on sorted input; say so rather
            // than returning a confusing miss.
            let sorted = is_sorted(&arr);
            let found = binary_search(&arr, &target);
            Ok(json!({
                "input_was_sorted": sorted,
                "found": found.is_some(),
                "index": found,
                "length": arr.len(),
            }))
        }};
    }

    match ty.as_str() {
        "i32" => search!(i32),
        "f64" => search!(f64),
        "String" => search!(String),
        _ => unreachable!("data_type_of validated the tag"),
    }
}

fn handle_closest_pair(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    // Accept both the documented `{"points": [...]}` wrapper and a bare array.
    let points: Vec<Point> = if json.get("points").is_some() {
        parse::<ClosestPairInput>(json)?.points
    } else {
        parse(json)?
    };

    match closest_pair(&points) {
        Some((a, b, dist)) => Ok(json!({
            "point_count": points.len(),
            "distance": dist,
            "pair": [
                { "x": a.x, "y": a.y },
                { "x": b.x, "y": b.y },
            ],
        })),
        None => Err("Need at least two points".into()),
    }
}

// ---------------------------------------------------------------------------
// Backtracking
// ---------------------------------------------------------------------------

fn handle_n_queens(json: Value, cancel: &AtomicBool) -> Result<Value, String> {
    let input: NQueensInput = parse(json)?;
    let result = n_queens(input.n, input.max_solutions, cancel)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn handle_sudoku(json: Value, cancel: &AtomicBool) -> Result<Value, String> {
    // Accept both `{"grid": [[...]]}` and a bare 9x9 array.
    let grid = if json.get("grid").is_some() {
        parse::<SudokuInput>(json)?.grid
    } else {
        parse(json)?
    };
    let result = solve_sudoku(grid, cancel)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn handle_subset_sum(json: Value, cancel: &AtomicBool) -> Result<Value, String> {
    let input: SubsetSumInput = parse(json)?;
    let result = subset_sum(&input.numbers, input.target, input.max_subsets, cancel);
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn handle_hamiltonian(json: Value, cancel: &AtomicBool) -> Result<Value, String> {
    let input: HamiltonianInput = parse(json)?;
    let edges: Vec<(usize, usize)> = input.edges.iter().map(|e| (e[0], e[1])).collect();
    let result = hamiltonian_cycle(input.num_vertices, &edges, cancel)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Branch and bound
// ---------------------------------------------------------------------------

fn handle_tsp(json: Value, cancel: &AtomicBool) -> Result<Value, String> {
    let input: TspInput = parse(json)?;
    // JSON has no infinity literal, so `null` stands in for "no road".
    let matrix: Vec<Vec<f64>> = input
        .matrix
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| cell.unwrap_or(f64::INFINITY))
                .collect()
        })
        .collect();
    let result = tsp_branch_and_bound(&matrix, cancel)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn handle_knapsack_bnb(json: Value, cancel: &AtomicBool) -> Result<Value, String> {
    let input: KnapsackBnbInput = parse(json)?;
    let result = knapsack_branch_and_bound(&input.items, input.capacity, cancel)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Dynamic programming
// ---------------------------------------------------------------------------

fn handle_knapsack_01(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: Knapsack01Input = parse(json)?;
    let result = knapsack_01(&input.items, input.capacity);
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn handle_lcs(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: LcsInput = parse(json)?;
    let result = longest_common_subsequence(&input.a, &input.b);
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn handle_lis(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: LisInput = parse(json)?;
    let result = longest_increasing_subsequence(&input.numbers);
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn handle_matrix_chain(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: MatrixChainInput = parse(json)?;
    let result = matrix_chain_order(&input.dimensions)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

fn handle_floyd_warshall(json: Value, _c: &AtomicBool) -> Result<Value, String> {
    let input: FloydWarshallInput = parse(json)?;
    let result = floyd_warshall(&input.matrix)?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

/// Deserialises a payload, reporting *why* it failed instead of a bare
/// "Invalid JSON".
fn parse<T: serde::de::DeserializeOwned>(json: Value) -> Result<T, String> {
    serde_json::from_value(json).map_err(|e| format!("Input does not match this algorithm: {e}"))
}

const SORTING_ALGOS: [&str; 8] = [
    "bubble sort",
    "bucket sort",
    "heap sort",
    "insertion sort",
    "merge sort",
    "quick sort 3 partition",
    "quick sort",
    "selection sort",
];

fn handler_for(algo: &str) -> Option<Handler> {
    Some(match algo {
        "radix sort" => handle_radix_sort,
        // Greedy
        "fractional knapsack" => handle_fractional_knapsack,
        "huffman coding" => handle_huffman,
        "kruskals algorithm" => handle_kruskal,
        "prims algorithm" => handle_prim,
        "activity selection" => handle_activity_selection,
        // Graph
        "bellman ford" => handle_bellman_ford,
        "breadth first search" => handle_bfs,
        "depth first search" => handle_dfs,
        "dijkstras algorithm" => handle_dijkstra,
        // Divide and conquer
        "binary search" => handle_binary_search,
        "closest pair of points" => handle_closest_pair,
        // Backtracking
        "n queens" => handle_n_queens,
        "sudoku solver" => handle_sudoku,
        "subset sum" => handle_subset_sum,
        "hamiltonian cycle" => handle_hamiltonian,
        // Branch and bound
        "traveling salesman" => handle_tsp,
        "knapsack optimization" => handle_knapsack_bnb,
        // Dynamic programming
        "knapsack 01" => handle_knapsack_01,
        "longest common subsequence" => handle_lcs,
        "longest increasing subsequence" => handle_lis,
        "matrix chain multiplication" => handle_matrix_chain,
        "floyd warshall" => handle_floyd_warshall,
        _ => return None,
    })
}

/// The dispatch itself, separated from the Tauri plumbing so it can be
/// exercised directly in tests.
fn run_algorithm(algo: &str, json_data: Value, cancel: &AtomicBool) -> Result<Value, String> {
    let algo = algo.trim().to_ascii_lowercase();
    let started = std::time::Instant::now();

    let payload = if SORTING_ALGOS.contains(&algo.as_str()) {
        handle_sort(json_data, &algo, cancel)?
    } else {
        let handler =
            handler_for(&algo).ok_or_else(|| format!("\"{algo}\" is not implemented yet"))?;
        handler(json_data, cancel)?
    };

    Ok(json!({
        "algorithm": algo,
        "duration_ns": started.elapsed().as_nanos() as u64,
        "result": payload,
    }))
}

#[tauri::command]
fn execute_json(
    json_data: Value,
    selected_algo: String,
    cancel_flag: State<'_, CancelFlag>,
) -> Result<Value, String> {
    // A previous run's cancellation must not abort this one.
    cancel_flag.0.store(false, Ordering::Relaxed);
    let cancel = cancel_flag.0.clone();
    run_algorithm(&selected_algo, json_data, &cancel)
}

// ---------------------------------------------------------------------------
// Benchmarking
// ---------------------------------------------------------------------------

impl std::fmt::Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Algorithm::*;
        f.write_str(match self {
            BubbleSort => "bubble sort",
            BucketSort => "bucket sort",
            HeapSort => "heap sort",
            InsertionSort => "insertion sort",
            MergeSort => "merge sort",
            QuickSort => "quick sort",
            QuickSort3Partition => "quick sort 3 partition",
            SelectionSort => "selection sort",
            RadixSort => "radix sort",
        })
    }
}

fn sort_in_place<T>(v: &mut Vec<T>, algo: Algorithm, cancel: &AtomicBool)
where
    T: PartialOrd + crate::algorithms::sorting_algorithms::bucket_sort::BucketSortKey + Clone,
{
    use crate::algorithms::sorting_algorithms::*;
    match algo {
        Algorithm::BubbleSort => bubble_sort(v, cancel),
        Algorithm::BucketSort => bucket_sort(v, cancel),
        Algorithm::HeapSort => heap_sort(v, cancel),
        Algorithm::InsertionSort => insertion_sort(v, cancel),
        Algorithm::MergeSort => merge_sort(v, cancel),
        Algorithm::QuickSort => quick_sort(v, cancel),
        Algorithm::QuickSort3Partition => quick_sort_3way(v, cancel),
        Algorithm::SelectionSort => selection_sort(v, cancel),
        // Radix sort is integer-only and handled by the caller.
        Algorithm::RadixSort => (),
    }
}

fn apply_algorithm(data: &mut RandomData, algo: Algorithm, cancel: &AtomicBool) {
    match data {
        RandomData::I32(v) => {
            if algo == Algorithm::RadixSort {
                radix_sort(v, cancel)
            } else {
                sort_in_place(v, algo, cancel)
            }
        }
        RandomData::F64(v) => sort_in_place(v, algo, cancel),
        RandomData::String(v) => sort_in_place(v, algo, cancel),
    }
}

/// Generates a dataset, sorts it, and reports the elapsed time.
///
/// Awaits the worker, so the caller's `await` genuinely sequences runs. The
/// previous version returned the moment the thread was spawned, so every
/// selected algorithm raced at once and the timings were meaningless.
#[tauri::command]
async fn generate_random(
    app: AppHandle,
    json_data: Value,
    cancel_flag: State<'_, CancelFlag>,
) -> Result<AlgorithmResult, String> {
    let input: GeneratorStruct = serde_json::from_value(json_data)
        .map_err(|e| format!("Invalid benchmark settings: {e}"))?;

    if input.range == 0 {
        return Err("Range must be at least 1".into());
    }
    if input.min > input.max {
        return Err("Min must not exceed max".into());
    }
    if matches!(input.data_type, DataType::I32) && input.selected_algorithm != Algorithm::RadixSort
    {
        // Other integer sorts are fine; this guard is only about radix.
    }
    if input.selected_algorithm == Algorithm::RadixSort
        && !matches!(input.data_type, DataType::I32)
    {
        return Err("Radix sort only supports the i32 data type".into());
    }

    let cancel = cancel_flag.0.clone();
    let app_clone = app.clone();

    let handle = tauri::async_runtime::spawn_blocking(move || {
        let mut data = crate::generators::generate_random_array(
            input.range,
            input.data_type,
            input.min,
            input.max,
        );

        let started = std::time::Instant::now();
        apply_algorithm(&mut data, input.selected_algorithm, &cancel);
        let duration_ns = started.elapsed().as_nanos() as u64;

        let result = AlgorithmResult {
            name: input.selected_algorithm.to_string(),
            duration_ns,
            length: input.range,
            cancelled: cancel.load(Ordering::Relaxed),
        };

        // Emitted as well as returned, so a UI can stream progress.
        if let Err(e) = app_clone.emit("algorithm-finished", result.clone()) {
            eprintln!("failed to emit algorithm-finished: {e:?}");
        }
        result
    });

    handle
        .await
        .map_err(|e| format!("Benchmark worker failed: {e}"))
}

/// Clears a pending cancellation. The Compare page calls this once before a
/// batch rather than per algorithm, so pressing Stop halts the whole batch.
#[tauri::command]
fn reset_cancel(cancel_flag: State<'_, CancelFlag>) {
    cancel_flag.0.store(false, Ordering::Relaxed);
}

#[tauri::command]
fn cancel(cancel_flag: State<'_, CancelFlag>) {
    cancel_flag.0.store(true, Ordering::Relaxed);
}

// ---------------------------------------------------------------------------
// Random input generators
//
// These were all called by the UI but never registered, so every "Random
// Input" button raised "command not found".
// ---------------------------------------------------------------------------

/// Rejects an editor-bound size that would freeze the syntax highlighter.
fn check_editor_size(count: usize) -> Result<(), String> {
    if count == 0 {
        return Err("Count must be at least 1".into());
    }
    if count > MAX_EDITOR_ARRAY {
        return Err(format!(
            "{count} values is too many for the editor (limit {MAX_EDITOR_ARRAY}). \
             Use the Compare page to benchmark larger inputs."
        ));
    }
    Ok(())
}

#[tauri::command]
fn generate_random_array(
    count: usize,
    data_type: DataType,
    min: f64,
    max: f64,
) -> Result<Value, String> {
    check_editor_size(count)?;
    if min > max {
        return Err("Min must not exceed max".into());
    }

    let data = crate::generators::generate_random_array(count, data_type, min, max);
    Ok(match data {
        RandomData::I32(v) => json!({ "type": "i32", "arr": v }),
        RandomData::F64(v) => json!({ "type": "f64", "arr": v }),
        RandomData::String(v) => json!({ "type": "String", "arr": v }),
    })
}

#[tauri::command]
fn generate_random_kruskal_input(
    num_vertices: usize,
    num_edges: usize,
    max_weight: usize,
) -> Result<Value, String> {
    if num_vertices < 2 {
        return Err("Need at least two vertices".into());
    }
    if max_weight == 0 {
        return Err("Max weight must be at least 1".into());
    }
    let input = crate::generators::generate_random_kruskal_input(num_vertices, num_edges, max_weight);
    Ok(json!({
        "num_vertices": input.num_vertices,
        "edges": input.edges.iter().map(|e| json!({
            "src": e.src, "dest": e.dest, "weight": e.weight
        })).collect::<Vec<_>>(),
    }))
}

#[tauri::command]
fn generate_random_prim_input(
    num_vertices: usize,
    edge_probability: f64,
) -> Result<Value, String> {
    if num_vertices < 2 {
        return Err("Need at least two vertices".into());
    }
    if !(0.0..=1.0).contains(&edge_probability) {
        return Err("Edge probability must be between 0 and 1".into());
    }
    let input = crate::generators::generate_random_prim_input(num_vertices, edge_probability);
    Ok(json!({
        "num_vertices": input.num_vertices,
        "edges": input.edges.iter().map(|e| json!({
            "src": e.src, "dest": e.dest, "weight": e.weight
        })).collect::<Vec<_>>(),
    }))
}

#[tauri::command]
fn generate_random_knapsack_input(
    num_items: usize,
    max_value: f64,
    max_weight: f64,
    capacity: f64,
) -> Result<Value, String> {
    check_editor_size(num_items)?;
    if max_value <= 0.0 || max_weight <= 0.0 {
        return Err("Max value and max weight must be positive".into());
    }
    let input =
        crate::generators::generate_random_knapsack_input(num_items, max_value, max_weight, capacity);
    Ok(json!({
        "capacity": input.capacity,
        "items": input.items.iter().map(|i| json!({
            "value": i.value, "weight": i.weight
        })).collect::<Vec<_>>(),
    }))
}

#[tauri::command]
fn generate_random_huffman_input(num_symbols: usize) -> Result<Value, String> {
    if num_symbols == 0 {
        return Err("Need at least one symbol".into());
    }
    if num_symbols > 26 {
        return Err("The generator draws from a 26-letter alphabet".into());
    }
    let input = crate::generators::generate_random_huffman_input(num_symbols);
    let map: serde_json::Map<String, Value> = input
        .frequencies
        .into_iter()
        .map(|(ch, freq)| (ch.to_string(), json!(freq)))
        .collect();
    Ok(Value::Object(map))
}

#[tauri::command]
fn generate_random_activities(
    num_activities: usize,
    max_start: usize,
    max_duration: usize,
) -> Result<Value, String> {
    check_editor_size(num_activities)?;
    if max_start == 0 || max_duration == 0 {
        return Err("Max start and max duration must be at least 1".into());
    }
    let activities =
        crate::generators::generate_random_activities(num_activities, max_start, max_duration);
    Ok(json!(activities
        .iter()
        .map(|a| json!({ "start": a.start, "finish": a.finish }))
        .collect::<Vec<_>>()))
}

#[tauri::command]
fn generate_random_points_closest(count: usize, min: f64, max: f64) -> Result<Value, String> {
    check_editor_size(count)?;
    if min > max {
        return Err("Min must not exceed max".into());
    }
    if count < 2 {
        return Err("Need at least two points".into());
    }
    let points = crate::generators::generate_random_points_closest(count, min, max);
    Ok(json!({
        "points": points.iter().map(|p| json!({ "x": p.x, "y": p.y })).collect::<Vec<_>>(),
    }))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(CancelFlag(Arc::new(AtomicBool::new(false))))
        .invoke_handler(tauri::generate_handler![
            execute_json,
            generate_random,
            cancel,
            reset_cancel,
            generate_random_array,
            generate_random_kruskal_input,
            generate_random_prim_input,
            generate_random_knapsack_input,
            generate_random_huffman_input,
            generate_random_activities,
            generate_random_points_closest,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[cfg(test)]
mod dispatch_tests {
    use super::*;

    /// Every payload shipped in public/instructions.json must deserialise and
    /// run through the real dispatch. This is the "pick an algorithm, press
    /// Run" path: two of these samples (bellman ford, closest pair of points)
    /// previously could not be parsed at all because their documented shape
    /// did not match the struct behind them.
    #[test]
    fn every_sample_payload_runs() {
        let raw = include_str!("../../public/instructions.json");
        let samples: std::collections::BTreeMap<String, String> =
            serde_json::from_str(raw).expect("instructions.json must be valid JSON");

        assert!(samples.len() >= 30, "expected a payload per algorithm");

        let cancel = AtomicBool::new(false);
        let mut failures = Vec::new();

        for (algo, payload) in &samples {
            let value: Value = match serde_json::from_str(payload) {
                Ok(v) => v,
                Err(e) => {
                    failures.push(format!("{algo}: sample is not valid JSON: {e}"));
                    continue;
                }
            };
            if let Err(e) = run_algorithm(algo, value, &cancel) {
                failures.push(format!("{algo}: {e}"));
            }
        }

        assert!(failures.is_empty(), "payloads failed:\n  {}", failures.join("\n  "));
    }

    #[test]
    fn unknown_algorithm_is_reported() {
        let cancel = AtomicBool::new(false);
        let err = run_algorithm("telepathy sort", json!({}), &cancel).unwrap_err();
        assert!(err.contains("not implemented"), "got: {err}");
    }

    #[test]
    fn sorting_output_is_summarised_not_dumped() {
        // A large array must come back as a bounded preview, never as a full
        // listing of every element.
        let arr: Vec<i32> = (0..50_000).rev().collect();
        let cancel = AtomicBool::new(false);
        let out = run_algorithm(
            "merge sort",
            json!({ "type": "i32", "arr": arr }),
            &cancel,
        )
        .unwrap();

        let result = &out["result"];
        assert_eq!(result["length"], 50_000);
        assert_eq!(result["sorted"], true);
        assert_eq!(result["truncated"], true);
        assert_eq!(result["head"].as_array().unwrap().len(), PREVIEW_LEN);
        assert_eq!(result["tail"].as_array().unwrap().len(), PREVIEW_LEN);
        // The whole response must stay small regardless of input size.
        assert!(
            out.to_string().len() < 4_000,
            "response should be a summary, got {} bytes",
            out.to_string().len()
        );
    }

    #[test]
    fn radix_sort_keeps_negative_numbers() {
        // The old handler silently filtered negatives out of the array.
        let cancel = AtomicBool::new(false);
        let out = run_algorithm(
            "radix sort",
            json!({ "type": "i32", "arr": [5, -3, 0, -17, 42, -1] }),
            &cancel,
        )
        .unwrap();
        let result = &out["result"];
        assert_eq!(result["length"], 6, "negatives must be kept");
        assert_eq!(result["sorted"], true);
        assert_eq!(result["head"][0], -17);
    }

    #[test]
    fn string_type_tag_is_case_insensitive() {
        let cancel = AtomicBool::new(false);
        for tag in ["String", "string", "STRING"] {
            let out = run_algorithm(
                "quick sort",
                json!({ "type": tag, "arr": ["pear", "apple", "fig"] }),
                &cancel,
            );
            assert!(out.is_ok(), "type tag {tag:?} should be accepted");
        }
    }

    #[test]
    fn bad_payload_explains_itself() {
        let cancel = AtomicBool::new(false);
        let err = run_algorithm("n queens", json!({ "wrong": 1 }), &cancel).unwrap_err();
        assert!(!err.is_empty());
        // Not the old opaque "Invalid JSON".
        assert_ne!(err, "Invalid JSON");
    }
}
