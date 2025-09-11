# Algorithms Explorer (KlugNet)

![KlugNet](https://i.pinimg.com/originals/41/3e/f4/413ef4fe169a7907f0d5f09b69c2ebde.gif)

## Overview

**KlugNet** is an interactive platform built with **Rust** and **Tauri**, designed to showcase a diverse collection of algorithms from fundamental paradigms. This project provides an educational and engaging experience for visualizing, understanding, and experimenting with various algorithmic approaches, including:

- Greedy Algorithms
- Dynamic Programming
- Backtracking
- Branch and Bound
- Divide and Conquer
- Graph Algorithms
- Sorting Algorithms

---

## Features

- **Extensive Algorithm Collection**  
  Explore implementations of numerous algorithms across key paradigms.

- **Rust-Powered Backend**  
  Leverages the performance and safety of Rust for robust algorithm execution and data processing.

- **Interactive Frontend**  
  A user-friendly interface (built with web technologies via Tauri) allows for easy navigation, parameter input, and visualization of algorithm steps and results.

- **Cross-Platform Compatibility**  
  As a Tauri application, KlugNet can be built for Windows, macOS, and Linux.

- **Educational Tool**  
  Ideal for students, developers, and anyone interested in deepening their understanding of algorithms.

---

## Algorithms and Complexities

| **Category**            | **Algorithm**                     | **Time Complexity**       | **Space Complexity**      |
|--------------------------|------------------------------------|---------------------------|---------------------------|
| **Sorting Algorithms**   | Bubble Sort                       | O(n²)                     | O(1)                     |
|                          | Quick Sort                        | O(n log n) (avg), O(n²) (worst) | O(log n) (recursion) |
|                          | Merge Sort                        | O(n log n)                | O(n)                     |
|                          | Radix Sort                        | O(nk)                     | O(n + k)                 |
| **Greedy Algorithms**    | Huffman Coding                    | O(n log n)                | O(n)                     |
|                          | Kruskal's Algorithm               | O(E log V)                | O(E + V)                 |
|                          | Prim's Algorithm                  | O(E log V)                | O(V²) (adjacency matrix) |
| **Graph Algorithms**     | Dijkstra's Algorithm              | O(V log V + E)            | O(V + E)                 |
|                          | Bellman-Ford Algorithm            | O(VE)                     | O(V)                     |
|                          | Breadth-First Search (BFS)        | O(V + E)                  | O(V)                     |
|                          | Depth-First Search (DFS)          | O(V + E)                  | O(V)                     |
| **Divide and Conquer**   | Closest Pair of Points            | O(n log n)                | O(n)                     |
|                          | Binary Search                     | O(log n)                  | O(1)                     |
| **Dynamic Programming**  | Floyd-Warshall Algorithm          | O(V³)                     | O(V²)                    |
|                          | Longest Common Subsequence        | O(mn)                     | O(mn)                    |
|                          | 0/1 Knapsack                      | O(nW)                     | O(nW)                    |
| **Backtracking**         | N-Queens Problem                  | O(n!)                     | O(n²)                    |
|                          | Sudoku Solver                     | O(9^(n²))                 | O(n²)                    |

---

## Getting Started

### Prerequisites

Ensure you have the following installed:

- [Node.js](https://nodejs.org/)
- [Rust](https://www.rust-lang.org/)
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/algorithms-explorer.git
   cd algorithms-explorer