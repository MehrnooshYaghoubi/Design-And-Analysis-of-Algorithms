# 🚀 Algorithms Explorer (KlugNet)

![KlugNet](https://i.pinimg.com/originals/41/3e/f4/413ef4fe169a7907f0d5f09b69c2ebde.gif)

## 📌 Overview

**KlugNet** is an interactive platform built with **Rust** and **Tauri**, designed to showcase a rich collection of algorithms across diverse paradigms.  
It serves as both a **learning resource** and an **experimental playground** for students, developers, and researchers to understand and visualize algorithmic concepts.

The project features implementations of:

- 🟢 Greedy Algorithms  
- 🔵 Dynamic Programming  
- 🟣 Backtracking  
- 🟡 Branch and Bound  
- 🟠 Divide and Conquer  
- 🔴 Graph Algorithms  
- ⚪ Sorting Algorithms  

---

## ✨ Features

- **Extensive Algorithm Library** → Covers core paradigms from sorting to optimization problems.  
- **Rust-Powered Backend** → Fast, memory-safe, and reliable algorithm execution.  
- **Interactive Frontend** → Visualize and experiment with algorithms via Tauri-powered UI.  
- **Cross-Platform** → Build and run on Windows, macOS, and Linux.  
- **Educational Design** → Step-by-step visualization for deeper understanding.  

---

## 📚 Algorithms & Complexities

### 🔹 Sorting Algorithms
| Algorithm              | Time Complexity (Avg) | Worst Case | Space |
|-------------------------|-----------------------|------------|-------|
| Bubble Sort            | O(n²)                 | O(n²)      | O(1) |
| Insertion Sort         | O(n²)                 | O(n²)      | O(1) |
| Selection Sort         | O(n²)                 | O(n²)      | O(1) |
| Quick Sort             | O(n log n)            | O(n²)      | O(log n) |
| Quick Sort (3-way)     | O(n log n)            | O(n²)      | O(log n) |
| Merge Sort             | O(n log n)            | O(n log n) | O(n) |
| Heap Sort              | O(n log n)            | O(n log n) | O(1) |
| Radix Sort             | O(nk)                 | O(nk)      | O(n+k) |
| Bucket Sort            | O(n+k)                | O(n²)      | O(n) |

---

### 🔹 Greedy Algorithms
| Algorithm              | Time Complexity       | Space |
|-------------------------|-----------------------|-------|
| Activity Selection     | O(n log n)            | O(1) |
| Fractional Knapsack    | O(n log n)            | O(1) |
| Huffman Coding         | O(n log n)            | O(n) |
| Kruskal’s Algorithm    | O(E log V)            | O(E+V) |
| Prim’s Algorithm       | O(E log V)            | O(V²) |

---

### 🔹 Graph Algorithms
| Algorithm              | Time Complexity       | Space |
|-------------------------|-----------------------|-------|
| BFS                    | O(V + E)              | O(V) |
| DFS                    | O(V + E)              | O(V) |
| Dijkstra’s Algorithm   | O(V log V + E)        | O(V+E) |
| Bellman-Ford           | O(VE)                 | O(V) |
| Floyd-Warshall         | O(V³)                 | O(V²) |

---

### 🔹 Divide and Conquer
| Algorithm              | Time Complexity       | Space |
|-------------------------|-----------------------|-------|
| Binary Search          | O(log n)              | O(1) |
| Merge Sort             | O(n log n)            | O(n) |
| Quick Sort             | O(n log n) (avg)      | O(log n) |
| Closest Pair of Points | O(n log n)            | O(n) |

---

### 🔹 Dynamic Programming
| Algorithm                         | Time Complexity | Space |
|----------------------------------|-----------------|-------|
| 0/1 Knapsack                     | O(nW)           | O(nW) |
| Longest Common Subsequence       | O(mn)           | O(mn) |
| Longest Increasing Subsequence   | O(n²) / O(n log n) | O(n) |
| Matrix Chain Multiplication      | O(n³)           | O(n²) |
| Floyd-Warshall                   | O(V³)           | O(V²) |

---

### 🔹 Backtracking
| Problem                 | Time Complexity     | Space |
|--------------------------|---------------------|-------|
| N-Queens                | O(n!)               | O(n²) |
| Sudoku Solver           | O(9^(n²))           | O(n²) |
| Subset Sum              | O(2^n)              | O(n) |
| Hamiltonian Cycle       | O(n!)               | O(n) |

---

### 🔹 Branch and Bound
| Problem                 | Time Complexity (approx.) | Space |
|--------------------------|---------------------------|-------|
| Traveling Salesman       | O(n!)                     | O(n) |
| Knapsack Optimization    | O(2^n) (improved pruning) | O(n) |

---

## ⚡ Getting Started

### ✅ Prerequisites
- [Node.js](https://nodejs.org/)  
- [Rust](https://www.rust-lang.org/)  
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)  

### 📥 Installation
```bash
git clone https://github.com/your-username/algorithms-explorer.git
cd algorithms-explorer
