# KlugNetz

An interactive algorithms laboratory: 33 algorithms implemented in Rust, run
natively through [Tauri](https://tauri.app), driven from a React interface.

---

## What it does

**Laboratory** — pick an algorithm, feed it a JSON payload (typed by hand,
imported from a file, or generated), and read a structured result with the
elapsed time measured in Rust.

**Compare** — run several sorting algorithms over the same randomly generated
dataset and compare how they scale. The dataset is built and sorted entirely in
Rust and never crosses into JavaScript, so the array size is limited by memory
rather than by what a DOM can display.

---

## Using the app

The window has three screens, reachable from the sidebar: **Home**,
**Laboratory** and **Compare**. The sun/moon button in the title bar switches
between the light and dark theme, and the choice is remembered.

### Laboratory — run one algorithm

1. **Pick an algorithm.** Expand *Algorithms* in the sidebar and choose one
   from a family. The chosen name appears as a tag above the editor, and a
   sample payload for it is loaded into `input.json`.
2. **Supply the input**, in whichever way suits:
   - **type or paste** JSON straight into the editor — the footer shows the
     character count and the cursor position, and invalid JSON is reported
     before anything is sent to Rust;
   - **Import** — load a `.json` file from disk;
   - **Random** — open a small form (length, value range, vertex count, edge
     probability, … depending on the algorithm) and generate a fresh payload.
     The button is disabled for algorithms with no generator.
3. **Run.** The result arrives as a card in the right-hand column: the output
   values, the time of the run, and two timings — **compute**, measured inside
   Rust around the algorithm itself, and **round trip**, the whole call
   including serialisation. Cards stack newest-first, the log keeps the **last
   50**, and the bin icon clears it.

Every algorithm's expected payload is in the *Algorithms* tables below, and the
sample loaded with each one is a working example of that shape.

### Compare — benchmark several sorts

1. Go to **Compare**. In the sidebar only the sorting algorithms are
   selectable; click to add each one, and click a tag to remove it.
2. Set the **dataset**: number of elements, element type (`i32`, `f64` or
   `String`) and the min/max value range. Radix sort accepts `i32` only.
3. Press **Start**. Each algorithm runs in turn over a freshly generated array
   of the same shape, and the chart fills in as results land. The status line
   names the algorithm currently running and its position in the batch.
4. Read the chart. Bars are ordered **fastest first**, share one time unit, and
   each is labelled with its own value; hover one for the exact duration, the
   element count and how many times slower it is than the fastest. Every
   algorithm keeps its own pastel colour, shown as a swatch next to its name,
   so a colour means the same thing in the chart and in the table — the
   bar-chart/table toggle in the header switches between the two views of the
   same numbers.

**Stop** interrupts the batch. The long-running searches (N-Queens, Sudoku,
subset sum, Hamiltonian cycle, travelling salesman, branch-and-bound knapsack)
poll a cancellation flag, so Stop ends them rather than leaving the window
wedged. A warning appears before you start if a quadratic sort is paired with
200,000 elements or more.

---

## Algorithms

Input columns give the top-level keys of the JSON payload. Complexities are for
the implementation in this repository.

### Sorting

Payload: `{ "type": "i32" | "f64" | "String", "arr": [...] }`. These are also
the algorithms available on the **Compare** screen.

| Algorithm | Time (average) | Time (worst) | Extra space | Notes |
|---|---|---|---|---|
| Bubble sort | O(n²) | O(n²) | O(1) | Stable; runs all n passes, with no early exit |
| Bucket sort | O(n + k) | O(n log n) | O(n) | n buckets by value, each sorted with Rust's stable sort |
| Heap sort | O(n log n) | O(n log n) | O(1) | In place, not stable |
| Insertion sort | O(n²) | O(n²) | O(1) | O(n) on nearly sorted input; stable |
| Merge sort | O(n log n) | O(n log n) | O(n) | Stable |
| Quick sort | O(n log n) | O(n²) | O(log n) | In place |
| Quick sort (3-way) | O(n log n) | O(n²) | O(log n) | Three-way partition; handles many duplicates |
| Radix sort | O(d·n) | O(d·n) | O(n) | `i32` only — digit passes, no comparisons |
| Selection sort | O(n²) | O(n²) | O(1) | Fewest writes of the quadratic three |

### Divide & conquer

| Algorithm | Input | Time | Notes |
|---|---|---|---|
| Binary search | `{ type, arr, target }` | O(log n) | `arr` must be sorted ascending; the result reports `input_was_sorted` so a meaningless miss is visible |
| Closest pair of points | `{ points: [{ x, y }] }` | O(n log n) | Returns the pair and its distance |
| Merge sort | `{ type, arr }` | O(n log n) | The recursive formulation |
| Quick sort | `{ type, arr }` | O(n log n) avg | Partition-and-recurse |

### Greedy

| Algorithm | Input | Time | Notes |
|---|---|---|---|
| Activity selection | `[{ start, finish }]` | O(n log n) | Most non-overlapping activities |
| Fractional knapsack | `{ capacity, items: [{ value, weight }] }` | O(n log n) | Items may be split; value-per-weight order |
| Huffman coding | `{ "<symbol>": <frequency> }` | O(n log n) | Returns the code per symbol |
| Kruskal's algorithm | `{ num_vertices, edges: [{ src, dest, weight }] }` | O(E log E) | MST via union-find |
| Prim's algorithm | `{ num_vertices, edges: [{ src, dest, weight }] }` | O(E log V) | MST via a binary heap |

### Graph

| Algorithm | Input | Time | Notes |
|---|---|---|---|
| Bellman-Ford | `{ vertex_count, start, edges: [{ from, to, weight }] }` | O(V·E) | Negative weights allowed; reports negative cycles |
| Breadth-first search | `{ graph: [[...]], start }` | O(V + E) | Adjacency lists; visit order and levels |
| Depth-first search | `{ graph: [[...]], start }` | O(V + E) | Adjacency lists; visit order |
| Dijkstra's algorithm | `{ adj_list: [[{ node, cost }]], start }` | O((V + E) log V) | Binary heap; non-negative weights |

### Dynamic programming

| Algorithm | Input | Time | Notes |
|---|---|---|---|
| Floyd-Warshall | `{ matrix: [[...]] }` | O(V³) | All-pairs shortest paths; `null` for "no edge" |
| 0/1 knapsack | `{ capacity, items: [{ value, weight }] }` | O(n·W) | Items taken whole; returns the chosen set |
| Longest common subsequence | `{ a, b }` | O(n·m) | Two strings; returns the subsequence |
| Longest increasing subsequence | `{ numbers: [...] }` | O(n log n) | Patience-style tails with binary search |
| Matrix chain multiplication | `{ dimensions: [...] }` | O(n³) | Cheapest parenthesisation |

### Backtracking

| Algorithm | Input | Time (worst) | Notes |
|---|---|---|---|
| Hamiltonian cycle | `{ num_vertices, edges }` | O(n!) | Cancellable |
| N-Queens | `{ n, max_solutions }` | O(n!) | `max_solutions` caps the search; cancellable |
| Subset sum | `{ numbers, target, max_subsets }` | O(2ⁿ) | `max_subsets` caps the search; cancellable |
| Sudoku solver | `{ grid: [[...]] }` | O(9^cells) | 9×9 grid, `0` for a blank; cancellable |

### Branch & bound

| Algorithm | Input | Time (worst) | Notes |
|---|---|---|---|
| 0/1 knapsack | `{ capacity, items: [{ value, weight }] }` | O(2ⁿ) | Bounded by the fractional relaxation; cancellable |
| Travelling salesman | `{ matrix: [[...]] }` | O(n!) | Cost matrix, pruned by a lower bound; cancellable |

---

## Running it

```bash
npm install
npm run tauri:dev     # development window
npm run tauri:build   # packaged release binary
npm run test:rust     # 92 algorithm unit tests
```

A release build is what you want for timing: the release profile enables
`opt-level = 3` with fat LTO, and even the dev profile raises the algorithms
above `opt-level = 0` so benchmarks are not dominated by unoptimised bounds
checks.

---

## Design notes

**Results are summarised, never dumped.** A million-element sort returns the
first and last 50 elements plus a verified `sorted` flag and the length — not a
million-element string. Formatting and shipping that string was the single
largest source of lag. In the editor, syntax highlighting switches off above
40,000 characters, since Prism re-tokenises the whole document on every
keystroke.

**Chart colour is identity, not measurement.** Each bar carries a pastel fill
drawn from a ring of well-separated hues, assigned by hashing the algorithm's
*name* — so a colour belongs to an algorithm and stays with it when the ranking
changes between runs. A palette indexed by row position, which an earlier
version used, recolours the whole chart the moment one sort overtakes another.
The comparison itself is carried by the ordering (fastest first) and by the
value printed at each bar's tip; the swatch beside each row is its own legend,
and the same swatches appear in the table view. Lightness is set per theme, so
the pastels hold up on white as well as on near-black.

---

## Layout

```
src/
  components/    titlebar, sidebar, algorithm tree, editor, result card, chart
  pages/         Algorithms (laboratory), compare (benchmarks)
  css/App.css    design tokens and primitives
  palette.js     pastel hue assignment for the comparison chart
  store.jsx      selection, capped result log, theme
src-tauri/
  src/lib.rs     Tauri command surface
  src/algorithms/<family>/
  app-icon.svg   master for `npx tauri icon`, which writes icons/
public/
  instructions.json      per-algorithm sample payloads
  folder_structure.json  the sidebar tree
  logomark.svg           favicon; the in-app mark is components/Logomark.jsx
```

## Licence

MIT — see [LICENSE](LICENSE). Copyright (c) 2026 Mehrnoosh Yaghoubi Kalourazi.
