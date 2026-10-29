// Parameter specs for the "Random input" form.
//
// These used to be a chain of `window.prompt()` calls, which does not work
// here at all: prompt() is not implemented in the Linux WebKit webview Tauri
// uses, so it returns null and every generator bailed out immediately. The
// form in RandomInputForm renders these specs instead.
//
// `invoke` entries call a Rust generator (registered in lib.rs). `local`
// entries build the payload in JavaScript, which is appropriate for the
// algorithms whose inputs are a handful of numbers.

const ARRAY_PARAMS = [
  { key: "count", label: "Length", type: "int", default: 24, min: 1, max: 10000 },
  {
    key: "dataType",
    label: "Type",
    type: "select",
    default: "i32",
    options: ["i32", "f64", "String"],
  },
  { key: "min", label: "Min", type: "float", default: 0 },
  { key: "max", label: "Max", type: "float", default: 1000 },
];

const arrayGenerator = {
  params: ARRAY_PARAMS,
  invoke: "generate_random_array",
  args: (v) => ({
    count: v.count,
    // The Rust DataType enum deserialises from lowercase tags.
    dataType: String(v.dataType).toLowerCase(),
    min: v.min,
    max: v.max,
  }),
};

/** Fisher-Yates, so the shuffle is uniform. */
function shuffled(n) {
  const a = Array.from({ length: n }, (_, i) => i);
  for (let i = a.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [a[i], a[j]] = [a[j], a[i]];
  }
  return a;
}

const randInt = (lo, hi) => lo + Math.floor(Math.random() * (hi - lo + 1));

const SUDOKU_PUZZLE = [
  [5, 3, 0, 0, 7, 0, 0, 0, 0],
  [6, 0, 0, 1, 9, 5, 0, 0, 0],
  [0, 9, 8, 0, 0, 0, 0, 6, 0],
  [8, 0, 0, 0, 6, 0, 0, 0, 3],
  [4, 0, 0, 8, 0, 3, 0, 0, 1],
  [7, 0, 0, 0, 2, 0, 0, 0, 6],
  [0, 6, 0, 0, 0, 0, 2, 8, 0],
  [0, 0, 0, 4, 1, 9, 0, 0, 5],
  [0, 0, 0, 0, 8, 0, 0, 7, 9],
];

export const GENERATORS = {
  // --- arrays ---
  "bubble sort": arrayGenerator,
  "bucket sort": arrayGenerator,
  "heap sort": arrayGenerator,
  "insertion sort": arrayGenerator,
  "merge sort": arrayGenerator,
  "quick sort": arrayGenerator,
  "quick sort 3 partition": arrayGenerator,
  "selection sort": arrayGenerator,
  "radix sort": {
    // Radix sort is integer-only, so the type is not offered.
    params: ARRAY_PARAMS.filter((p) => p.key !== "dataType"),
    invoke: "generate_random_array",
    args: (v) => ({
      count: v.count,
      dataType: "i32",
      min: v.min,
      max: v.max,
    }),
  },

  "binary search": {
    params: [
      { key: "count", label: "Length", type: "int", default: 16, min: 1, max: 10000 },
      { key: "max", label: "Max value", type: "int", default: 100, min: 1 },
    ],
    // Binary search needs sorted input, so this sorts before handing it over.
    local: (v) => {
      const arr = Array.from({ length: v.count }, () => randInt(0, v.max)).sort(
        (a, b) => a - b
      );
      const target =
        arr.length > 0 && Math.random() < 0.75
          ? arr[randInt(0, arr.length - 1)]
          : randInt(0, v.max);
      return { type: "i32", arr, target };
    },
  },

  // --- graphs and greedy (Rust generators) ---
  "kruskals algorithm": {
    params: [
      { key: "numVertices", label: "Vertices", type: "int", default: 6, min: 2, max: 500 },
      { key: "numEdges", label: "Edges", type: "int", default: 9, min: 1 },
      { key: "maxWeight", label: "Max weight", type: "int", default: 20, min: 1 },
    ],
    invoke: "generate_random_kruskal_input",
    args: (v) => v,
  },
  "prims algorithm": {
    params: [
      { key: "numVertices", label: "Vertices", type: "int", default: 6, min: 2, max: 500 },
      {
        key: "edgeProbability",
        label: "Edge probability",
        type: "float",
        default: 0.5,
        min: 0,
        max: 1,
      },
    ],
    invoke: "generate_random_prim_input",
    args: (v) => v,
  },
  "fractional knapsack": {
    params: [
      { key: "numItems", label: "Items", type: "int", default: 6, min: 1, max: 10000 },
      { key: "maxValue", label: "Max value", type: "float", default: 100 },
      { key: "maxWeight", label: "Max weight", type: "float", default: 50 },
      { key: "capacity", label: "Capacity", type: "float", default: 100 },
    ],
    invoke: "generate_random_knapsack_input",
    args: (v) => v,
  },
  "huffman coding": {
    params: [
      { key: "numSymbols", label: "Symbols", type: "int", default: 6, min: 1, max: 26 },
    ],
    invoke: "generate_random_huffman_input",
    args: (v) => v,
  },
  "activity selection": {
    params: [
      { key: "numActivities", label: "Activities", type: "int", default: 8, min: 1, max: 10000 },
      { key: "maxStart", label: "Max start", type: "int", default: 20, min: 1 },
      { key: "maxDuration", label: "Max duration", type: "int", default: 6, min: 1 },
    ],
    invoke: "generate_random_activities",
    args: (v) => v,
  },
  "closest pair of points": {
    params: [
      { key: "count", label: "Points", type: "int", default: 12, min: 2, max: 10000 },
      { key: "min", label: "Min coord", type: "float", default: -100 },
      { key: "max", label: "Max coord", type: "float", default: 100 },
    ],
    invoke: "generate_random_points_closest",
    args: (v) => v,
  },

  // --- traversals ---
  "breadth first search": {
    params: [
      { key: "numVertices", label: "Vertices", type: "int", default: 8, min: 1, max: 500 },
      {
        key: "edgeProbability",
        label: "Edge probability",
        type: "float",
        default: 0.35,
        min: 0,
        max: 1,
      },
    ],
    local: (v) => {
      const graph = Array.from({ length: v.numVertices }, () => []);
      for (let i = 0; i < v.numVertices; i++) {
        for (let j = i + 1; j < v.numVertices; j++) {
          if (Math.random() < v.edgeProbability) {
            graph[i].push(j);
            graph[j].push(i);
          }
        }
      }
      return { graph, start: 0 };
    },
  },

  "dijkstras algorithm": {
    params: [
      { key: "numVertices", label: "Vertices", type: "int", default: 6, min: 1, max: 500 },
      {
        key: "edgeProbability",
        label: "Edge probability",
        type: "float",
        default: 0.4,
        min: 0,
        max: 1,
      },
      { key: "maxCost", label: "Max cost", type: "int", default: 20, min: 1 },
    ],
    local: (v) => {
      const adj_list = Array.from({ length: v.numVertices }, () => []);
      for (let i = 0; i < v.numVertices; i++) {
        for (let j = 0; j < v.numVertices; j++) {
          if (i !== j && Math.random() < v.edgeProbability) {
            adj_list[i].push({ node: j, cost: randInt(1, v.maxCost) });
          }
        }
      }
      return { adj_list, start: 0 };
    },
  },

  "bellman ford": {
    params: [
      { key: "vertexCount", label: "Vertices", type: "int", default: 6, min: 1, max: 500 },
      { key: "edgeCount", label: "Edges", type: "int", default: 10, min: 0 },
      { key: "minWeight", label: "Min weight", type: "int", default: -4 },
      { key: "maxWeight", label: "Max weight", type: "int", default: 10 },
    ],
    local: (v) => {
      const edges = [];
      for (let i = 0; i < v.edgeCount; i++) {
        const from = randInt(0, v.vertexCount - 1);
        let to = randInt(0, v.vertexCount - 1);
        if (to === from) to = (to + 1) % v.vertexCount;
        edges.push({ from, to, weight: randInt(v.minWeight, v.maxWeight) });
      }
      return { vertex_count: v.vertexCount, start: 0, edges };
    },
  },

  // --- backtracking ---
  "n queens": {
    params: [
      { key: "n", label: "Board size", type: "int", default: 8, min: 1, max: 18 },
      { key: "maxSolutions", label: "Boards to return", type: "int", default: 4, min: 1, max: 200 },
    ],
    local: (v) => ({ n: v.n, max_solutions: v.maxSolutions }),
  },
  "sudoku solver": {
    params: [
      { key: "blanks", label: "Blank cells", type: "int", default: 51, min: 1, max: 64 },
    ],
    // Carved from a solved grid so the puzzle always has a solution.
    local: (v) => {
      const grid = SUDOKU_PUZZLE.map((r) => [...r]);
      const cells = shuffled(81);
      let removed = grid.flat().filter((x) => x === 0).length;
      for (const cell of cells) {
        if (removed >= v.blanks) break;
        const r = Math.floor(cell / 9);
        const c = cell % 9;
        if (grid[r][c] !== 0) {
          grid[r][c] = 0;
          removed++;
        }
      }
      return { grid };
    },
  },
  "subset sum": {
    params: [
      { key: "count", label: "Numbers", type: "int", default: 8, min: 1, max: 28 },
      { key: "maxValue", label: "Max value", type: "int", default: 30, min: 1 },
      { key: "maxSubsets", label: "Subsets to return", type: "int", default: 8, min: 1, max: 200 },
    ],
    local: (v) => {
      const numbers = Array.from({ length: v.count }, () => randInt(1, v.maxValue));
      // Target the sum of a real sub-selection, so a solution usually exists.
      const picks = shuffled(v.count).slice(0, Math.max(1, Math.ceil(v.count / 3)));
      const target = picks.reduce((s, i) => s + numbers[i], 0);
      return { numbers, target, max_subsets: v.maxSubsets };
    },
  },
  "hamiltonian cycle": {
    params: [
      { key: "numVertices", label: "Vertices", type: "int", default: 7, min: 1, max: 14 },
      { key: "extraEdges", label: "Extra edges", type: "int", default: 4, min: 0 },
    ],
    // Seeded with a full cycle plus extra chords, so one always exists.
    local: (v) => {
      const n = v.numVertices;
      const seen = new Set();
      const edges = [];
      const add = (a, b) => {
        if (a === b) return;
        const key = `${Math.min(a, b)}-${Math.max(a, b)}`;
        if (seen.has(key)) return;
        seen.add(key);
        edges.push([a, b]);
      };
      const ring = shuffled(n);
      for (let i = 0; i < n; i++) add(ring[i], ring[(i + 1) % n]);
      for (let i = 0; i < v.extraEdges; i++) add(randInt(0, n - 1), randInt(0, n - 1));
      return { num_vertices: n, edges };
    },
  },

  // --- branch and bound ---
  "traveling salesman": {
    params: [
      { key: "cities", label: "Cities", type: "int", default: 8, min: 1, max: 16 },
      { key: "maxCost", label: "Max distance", type: "int", default: 60, min: 1 },
    ],
    // Symmetric matrix with a zero diagonal.
    local: (v) => {
      const n = v.cities;
      const m = Array.from({ length: n }, () => Array(n).fill(0));
      for (let i = 0; i < n; i++) {
        for (let j = i + 1; j < n; j++) {
          const d = randInt(1, v.maxCost);
          m[i][j] = d;
          m[j][i] = d;
        }
      }
      return { matrix: m };
    },
  },
  "knapsack optimization": {
    params: [
      { key: "numItems", label: "Items", type: "int", default: 12, min: 1, max: 2000 },
      { key: "maxValue", label: "Max value", type: "int", default: 100, min: 1 },
      { key: "maxWeight", label: "Max weight", type: "int", default: 40, min: 1 },
      { key: "capacity", label: "Capacity", type: "float", default: 100, min: 0 },
    ],
    local: (v) => ({
      capacity: v.capacity,
      items: Array.from({ length: v.numItems }, () => ({
        value: randInt(1, v.maxValue),
        weight: randInt(1, v.maxWeight),
      })),
    }),
  },

  // --- dynamic programming ---
  "knapsack 01": {
    params: [
      { key: "numItems", label: "Items", type: "int", default: 10, min: 1, max: 2000 },
      { key: "maxValue", label: "Max value", type: "int", default: 100, min: 1 },
      { key: "maxWeight", label: "Max weight", type: "int", default: 30, min: 1 },
      { key: "capacity", label: "Capacity", type: "int", default: 80, min: 0 },
    ],
    local: (v) => ({
      capacity: v.capacity,
      items: Array.from({ length: v.numItems }, () => ({
        value: randInt(1, v.maxValue),
        weight: randInt(1, v.maxWeight),
      })),
    }),
  },
  "longest common subsequence": {
    params: [
      { key: "lengthA", label: "Length of A", type: "int", default: 12, min: 0, max: 4000 },
      { key: "lengthB", label: "Length of B", type: "int", default: 12, min: 0, max: 4000 },
      { key: "alphabet", label: "Alphabet size", type: "int", default: 4, min: 1, max: 26 },
    ],
    local: (v) => {
      const letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".slice(0, v.alphabet);
      const word = (len) =>
        Array.from(
          { length: len },
          () => letters[randInt(0, letters.length - 1)]
        ).join("");
      return { a: word(v.lengthA), b: word(v.lengthB) };
    },
  },
  "longest increasing subsequence": {
    params: [
      { key: "count", label: "Numbers", type: "int", default: 16, min: 0, max: 10000 },
      { key: "maxValue", label: "Max value", type: "int", default: 100, min: 1 },
    ],
    local: (v) => ({
      numbers: Array.from({ length: v.count }, () => randInt(-v.maxValue, v.maxValue)),
    }),
  },
  "matrix chain multiplication": {
    params: [
      { key: "matrices", label: "Matrices", type: "int", default: 6, min: 1, max: 200 },
      { key: "maxDim", label: "Max dimension", type: "int", default: 40, min: 1 },
    ],
    local: (v) => ({
      // n matrices need n + 1 dimensions.
      dimensions: Array.from({ length: v.matrices + 1 }, () => randInt(2, v.maxDim)),
    }),
  },
  "floyd warshall": {
    params: [
      { key: "numVertices", label: "Vertices", type: "int", default: 6, min: 0, max: 300 },
      {
        key: "edgeProbability",
        label: "Edge probability",
        type: "float",
        default: 0.4,
        min: 0,
        max: 1,
      },
      { key: "maxWeight", label: "Max weight", type: "int", default: 20, min: 1 },
    ],
    local: (v) => {
      const n = v.numVertices;
      const matrix = Array.from({ length: n }, (_, i) =>
        Array.from({ length: n }, (_, j) => {
          if (i === j) return 0;
          // null is "no edge"; JSON cannot express infinity.
          return Math.random() < v.edgeProbability ? randInt(1, v.maxWeight) : null;
        })
      );
      return { matrix };
    },
  },
};

export function generatorFor(algorithm) {
  return algorithm ? GENERATORS[algorithm] ?? null : null;
}
