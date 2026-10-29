use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Types And Structs

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    #[serde(rename = "i32")]
    I32,
    #[serde(rename = "f64")]
    F64,
    #[serde(rename = "string")]
    String,
}

#[derive(Clone, Deserialize, Copy, Debug, PartialEq)]
pub enum Algorithm {
    #[serde(rename = "bubble sort")]
    BubbleSort,
    #[serde(rename = "bucket sort")]
    BucketSort,
    #[serde(rename = "heap sort")]
    HeapSort,
    #[serde(rename = "insertion sort")]
    InsertionSort,
    #[serde(rename = "merge sort")]
    MergeSort,
    #[serde(rename = "quick sort")]
    QuickSort,
    #[serde(rename = "quick sort 3 partition")]
    QuickSort3Partition,
    #[serde(rename = "selection sort")]
    SelectionSort,
    #[serde(rename = "radix sort")]
    RadixSort,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct AlgorithmResult {
    pub name: String,
    pub duration_ns: u64,
    /// Number of elements sorted, so the UI can label the run.
    pub length: usize,
    /// True when the run was stopped early.
    pub cancelled: bool,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GeneratorStruct {
    pub selected_algorithm: Algorithm,
    pub range: usize,
    pub data_type: DataType,
    pub min: f64,
    pub max: f64,
}

#[derive(Debug)]
pub enum RandomData {
    I32(Vec<i32>),
    F64(Vec<f64>),
    String(Vec<String>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Edge {
    pub src: usize,
    pub dest: usize,
    pub weight: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GraphInput {
    pub num_vertices: usize,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub value: f64,
    pub weight: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KnapsackInput {
    pub capacity: f64,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Activity {
    pub start: usize,
    pub finish: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct HuffmanInput {
    pub frequencies: HashMap<char, usize>,
}

#[derive(serde::Deserialize)]
pub struct TraversalInput {
    pub graph: Vec<Vec<usize>>,
    pub start: usize,
}

#[derive(serde::Deserialize)]
pub struct InputDijkstra {
    pub adj_list: Vec<Vec<DijkstraEdge>>,
    pub start: usize,
}

#[derive(Copy, Clone, Deserialize)]
pub struct DijkstraEdge {
    pub node: usize,
    pub cost: usize,
}

// ---------------------------------------------------------------------------
// Inputs for the backtracking, branch-and-bound and dynamic-programming sets.
// ---------------------------------------------------------------------------

/// An integer-weighted item, used by both knapsack implementations.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct WeightedItem {
    pub value: u64,
    pub weight: u64,
}

/// A directed edge that may carry a negative weight. `Edge` cannot: its
/// `weight` is a `usize`, which is exactly why Bellman-Ford could not parse
/// its own documented example.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct SignedEdge {
    pub from: usize,
    pub to: usize,
    pub weight: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BellmanFordInput {
    pub vertex_count: usize,
    pub start: usize,
    pub edges: Vec<SignedEdge>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ClosestPairInput {
    pub points: Vec<Point>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NQueensInput {
    pub n: usize,
    /// How many boards to materialise. Counting is always exhaustive.
    #[serde(default = "default_max_solutions")]
    pub max_solutions: usize,
}

fn default_max_solutions() -> usize {
    32
}

#[derive(Debug, Clone, Deserialize)]
pub struct SudokuInput {
    pub grid: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubsetSumInput {
    pub numbers: Vec<i64>,
    pub target: i64,
    #[serde(default = "default_max_subsets")]
    pub max_subsets: usize,
}

fn default_max_subsets() -> usize {
    32
}

#[derive(Debug, Clone, Deserialize)]
pub struct HamiltonianInput {
    pub num_vertices: usize,
    /// Undirected `[u, v]` pairs.
    pub edges: Vec<[usize; 2]>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TspInput {
    /// Square cost matrix; `null` means "no road".
    pub matrix: Vec<Vec<Option<f64>>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Knapsack01Input {
    pub capacity: u64,
    pub items: Vec<WeightedItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KnapsackBnbInput {
    pub capacity: f64,
    pub items: Vec<WeightedItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LcsInput {
    pub a: String,
    pub b: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LisInput {
    pub numbers: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MatrixChainInput {
    /// Matrix `i` has shape `dimensions[i] x dimensions[i + 1]`.
    pub dimensions: Vec<usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FloydWarshallInput {
    /// Square weight matrix; `null` means "no edge".
    pub matrix: Vec<Vec<Option<i64>>>,
}
