use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Types And Structs

pub type AlgoHandler = fn(Value) -> Result<String, String>;

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

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BellmanFordInput {
    pub graph: GraphInput,
    pub start: usize,
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
