import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useAppStore } from "./../store";
import Editor from "react-simple-code-editor";
import Prism from "prismjs";
import "prismjs/components/prism-json";
import { Dices } from "lucide-react";

const JsonCodeBlock = () => {
  const [code, setCode] = useState("");
  const [activeLine, setActiveLine] = useState(null);
  const [isFocused, setIsFocused] = useState(false);
  const [executionResult, setExecutionResult] = useState(null);
  const [isExecuting, setIsExecuting] = useState(false);
  const currentAlgo = useAppStore((state) => state.selectedAlgorithm);
  const Results = useAppStore((state) => state.results);
  const setResults = useAppStore((state) => state.setResults);
  const selectedAlgorithm = useAppStore((state) => state.selectedAlgorithm);

  // Simulate cursor animation
  useEffect(() => {
    fetch("/instructions.json")
      .then((res) => res.json())
      .then((data) => setStructure(data))
      .catch((err) => console.error("Error loading JSON:", err));
    const interval = setInterval(() => {
      setIsFocused((prev) => !prev);
    }, 800);
    return () => clearInterval(interval);
  }, []);

  useEffect(() => {
    fetch("/instructions.json")
      .then((res) => res.json())
      .then((data) => {
        if (selectedAlgorithm !== null) {
          if (data[selectedAlgorithm] != null) {
            setCode(data[selectedAlgorithm]);
          }
        }
      })
      .catch((err) => console.error("Error loading JSON:", err));
  }, [selectedAlgorithm]);

  const handleExecute = async () => {
    setIsExecuting(true);
    setExecutionResult(null);
    if (currentAlgo === null) {
      setExecutionResult({
        success: false,
        message: "No Algorithm is selected!",
      });
      setIsExecuting(false);
      return;
    }
    try {
      const parsedJson = JSON.parse(code);

      const copyOfselectedAlgo = selectedAlgorithm;
      const start = performance.now();
      const result = await invoke("execute_json", {
        jsonData: parsedJson,
        selectedAlgo: currentAlgo,
      });
      const end = performance.now();
      const executionTime = end - start;
      setExecutionResult({
        success: true,
        message: "Success",
      });
      console.log({
        text: result,
        time: new Date().toLocaleTimeString(),
        algo: copyOfselectedAlgo,
        duration: executionTime.toFixed(2) + " ms",
      });
      setResults([
        ...Results,
        {
          text: result,
          time: new Date().toLocaleTimeString(),
          algo: copyOfselectedAlgo,
          duration: executionTime.toFixed(2) + " ms",
        },
      ]);
    } catch (error) {
      setExecutionResult({
        success: false,
        message: error.toString(),
      });
    } finally {
      setIsExecuting(false);
    }
  };

  const handleImport = (e) => {
    const file = e.target.files[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const fileContent = event.target.result;
        JSON.parse(fileContent);
        setCode(fileContent);
        setExecutionResult({
          success: true,
          message: "File imported successfully!",
        });
      } catch (error) {
        setExecutionResult({
          success: false,
          message: "Invalid JSON file: " + error.message,
        });
      }
    };
    reader.readAsText(file);
  };

  function generateRandomKruskalInput(
    numVertices = 4,
    numEdges = 5,
    maxWeight = 20
  ) {
    const edges = new Set();
    const edgeList = [];

    // Ensure we don't have more edges than possible in an undirected graph
    const maxPossibleEdges = (numVertices * (numVertices - 1)) / 2;
    numEdges = Math.min(numEdges, maxPossibleEdges);

    while (edgeList.length < numEdges) {
      const src = Math.floor(Math.random() * numVertices);
      let dest = Math.floor(Math.random() * numVertices);

      // Ensure src != dest and the edge hasn't already been added (undirected, so (1,2) == (2,1))
      while (
        dest === src ||
        edges.has(`${Math.min(src, dest)}-${Math.max(src, dest)}`)
      ) {
        dest = Math.floor(Math.random() * numVertices);
      }

      const weight = Math.floor(Math.random() * maxWeight) + 1;

      edges.add(`${Math.min(src, dest)}-${Math.max(src, dest)}`);
      edgeList.push({ src, dest, weight });
    }

    return {
      num_vertices: numVertices,
      edges: edgeList,
    };
  }

  function generateRandomKnapsackInput(
    numItems = 5,
    maxValue = 100,
    maxWeight = 50,
    capacity = 100.0
  ) {
    const items = [];

    for (let i = 0; i < numItems; i++) {
      const value = parseFloat((Math.random() * maxValue + 1).toFixed(2));
      const weight = parseFloat((Math.random() * maxWeight + 1).toFixed(2));
      items.push({ value, weight });
    }

    return {
      capacity: parseFloat(capacity.toFixed(2)),
      items,
    };
  }

  function generateRandomHuffmanInput(numSymbols = 6) {
    const result = {};
    const alphabet = "abcdefghijklmnopqrstuvwxyz";
    const usedChars = new Set();

    while (Object.keys(result).length < numSymbols) {
      const char = alphabet[Math.floor(Math.random() * alphabet.length)];
      if (!usedChars.has(char)) {
        usedChars.add(char);
        const frequency = Math.floor(Math.random() * 50) + 1; // frequency between 1 and 50
        result[char] = frequency;
      }
    }

    return result;
  }

  function generateRandomPrimInput(numVertices = 5, edgeProbability = 0.5) {
    const edges = [];

    for (let i = 0; i < numVertices; i++) {
      for (let j = i + 1; j < numVertices; j++) {
        if (Math.random() < edgeProbability) {
          edges.push({
            src: i,
            dest: j,
            weight: Math.floor(Math.random() * 20) + 1, // weight between 1 and 20
          });
        }
      }
    }

    return {
      num_vertices: numVertices,
      edges: edges,
    };
  }

  function generateRandomActivities(
    numActivities = 5,
    maxStart = 10,
    maxDuration = 5
  ) {
    const activities = [];

    for (let i = 0; i < numActivities; i++) {
      const start = Math.floor(Math.random() * maxStart);
      const duration = Math.floor(Math.random() * maxDuration) + 1;
      const finish = start + duration;

      activities.push({ start, finish });
    }

    // Optional: sort by finish time to simulate greedy approach input
    activities.sort((a, b) => a.finish - b.finish);

    return activities;
  }

  function generateRandomPointsJSON(count = 10, min = -100, max = 100) {
    const points = [];

    for (let i = 0; i < count; i++) {
      const x = +(Math.random() * (max - min) + min).toFixed(2);
      const y = +(Math.random() * (max - min) + min).toFixed(2);
      points.push({ x, y });
    }

    return JSON.stringify(points, null, 2); // Pretty-printed JSON
  }

  const onRandomClick = async () => {
    if (selectedAlgorithm === "kruskals algorithm") {
      const numVertices = prompt(
        "Enter the number of vertices (default: 4)",
        "4"
      );
      const numEdges = prompt("Enter the number of edges (default: 5)", "5");
      const maxWeight = prompt(
        "Enter the maximum weight for edges (default: 20)",
        "20"
      );
      if (numVertices === null || numEdges === null || maxWeight === null)
        return;
      if (numVertices === "" || numEdges === "" || maxWeight === "") return;
      if (isNaN(numVertices) || isNaN(numEdges) || isNaN(maxWeight)) {
        setExecutionResult({
          success: false,
          message: "All inputs must be numbers.",
        });
        return;
      }
      // const randomInput = generateRandomKruskalInput(
      //   parseInt(numVertices),
      //   parseInt(numEdges),
      //   parseInt(maxWeight)
      // );

      const result = await invoke("generate_random_kruskal_input", {
        num_vertices: numVertices,
        num_edges: numEdges,
        max_weight: maxWeight,
      });

      setCode(JSON.stringify(result, null, 2));
      return;
    }

    if (selectedAlgorithm === "fractional knapsack") {
      const numItems = prompt("Enter the number of items (default: 5)", "5");
      const maxValue = prompt("Enter the maximum value (default: 100)", "100");
      const maxWeight = prompt("Enter the maximum weight (default: 50)", "50");
      const capacity = prompt(
        "Enter the knapsack capacity (default: 100.0)",
        "100.0"
      );
      if (
        numItems === null ||
        maxValue === null ||
        maxWeight === null ||
        capacity === null
      )
        return;
      if (
        numItems === "" ||
        maxValue === "" ||
        maxWeight === "" ||
        capacity === ""
      )
        return;
      if (
        isNaN(numItems) ||
        isNaN(maxValue) ||
        isNaN(maxWeight) ||
        isNaN(capacity)
      ) {
        setExecutionResult({
          success: false,
          message: "All inputs must be numbers.",
        });
        return;
      }
      // const randomInput = generateRandomKnapsackInput(
      //   parseInt(numItems),
      //   parseFloat(maxValue),
      //   parseFloat(maxWeight),
      //   parseFloat(capacity)
      // );

      const result = await invoke("generate_random_knapsack_input", {
        num_items: numItems,
        max_value: maxValue,
        max_weight: maxWeight,
        capacity: capacity,
      });

      setCode(JSON.stringify(result, null, 2));
      return;
    }

    if (selectedAlgorithm === "huffman coding") {
      const numSymbols = prompt(
        "Enter the number of symbols (default: 6)",
        "6"
      );
      if (numSymbols === null || numSymbols === "") return;
      if (isNaN(numSymbols)) {
        setExecutionResult({
          success: false,
          message: "Number of symbols must be a number.",
        });
        return;
      }
      // const randomInput = generateRandomHuffmanInput(parseInt(numSymbols));
      const result = await invoke("generate_random_huffman_input", {
        numSymbols: parseInt(numSymbols),
      });

      setCode(JSON.stringify(result, null, 2));
      return;
    }

    if (selectedAlgorithm === "prims algorithm") {
      const numVertices = prompt(
        "Enter the number of vertices (default: 5)",
        "5"
      );
      const edgeProbability = prompt(
        "Enter the edge probability (default: 0.5)",
        "0.5"
      );
      if (numVertices === null || edgeProbability === null) return;
      if (numVertices === "" || edgeProbability === "") return;
      if (isNaN(numVertices) || isNaN(edgeProbability)) {
        setExecutionResult({
          success: false,
          message: "All inputs must be numbers.",
        });
        return;
      }
      // const randomInput = generateRandomPrimInput(
      //   parseInt(numVertices),
      //   parseFloat(edgeProbability)
      // );

      const result = await invoke("generate_random_prim_input", {
        numVertices: parseInt(numVertices),
        edgeProbability: parseFloat(edgeProbability),
      });

      setCode(JSON.stringify(result, null, 2));
      return;
    }

    if (selectedAlgorithm === "activity selection") {
      const numActivities = prompt(
        "Enter the number of activities (default: 5)",
        "5"
      );
      const maxStart = prompt(
        "Enter the maximum start time (default: 10)",
        "10"
      );
      const maxDuration = prompt(
        "Enter the maximum duration (default: 5)",
        "5"
      );
      if (numActivities === null || maxStart === null || maxDuration === null)
        return;
      if (numActivities === "" || maxStart === "" || maxDuration === "") return;
      if (isNaN(numActivities) || isNaN(maxStart) || isNaN(maxDuration)) {
        setExecutionResult({
          success: false,
          message: "All inputs must be numbers.",
        });
        return;
      }
      // const randomInput = generateRandomActivities(
      //   parseInt(numActivities),
      //   parseInt(maxStart),
      //   parseInt(maxDuration)
      // );

      // const result = await invoke("generate_random_activities", {
      //   num_activities: numActivities,
      //   max_start: maxStart,
      //   max_duration: maxDuration,
      // });
      const result = await invoke("generate_random_activities", {
        numActivities: parseInt(numActivities),
        maxStart: parseInt(maxStart),
        maxDuration: parseInt(maxDuration),
      });

      setCode(JSON.stringify(result, null, 2));
      return;
    }

    if (selectedAlgorithm === "closest pair of points") {
      const count = prompt("Enter the number of points (default: 10)", "10");
      const min = prompt(
        "Enter the minimum coordinate value (default: -100)",
        "-100"
      );
      const max = prompt(
        "Enter the maximum coordinate value (default: 100)",
        "100"
      );
      if (count === null || min === null || max === null) return;
      if (count === "" || min === "" || max === "") return;
      if (isNaN(count) || isNaN(min) || isNaN(max)) {
        setExecutionResult({
          success: false,
          message: "All inputs must be numbers.",
        });
        return;
      }
      // const randomInput = generateRandomPointsJSON(
      //   parseInt(count),
      //   parseFloat(min),
      //   parseFloat(max)
      // );
      const result = await invoke("generate_random_points_closest_json", {
        count: parseInt(count),
        min: parseFloat(min),
        max: parseFloat(max),
      });

      setCode(result);
      return;
    }

    const range = prompt("Inter The Length Of Random Input", "10");
    if (range === null || range === "") return;

    const type = prompt(
      "Inter The Type Of Random Input (i32, f64, String)",
      "i32"
    );

    if (!["i32", "f64", "String"].includes(type)) {
      setExecutionResult({
        success: false,
        message: "Invalid type selected. Please choose i32, f64, or String.",
      });
      return;
    }
    let LOWER_BOUND = 0;
    let UPPER_BOUND = 1000000;

    if (type == "i32" || type == "f65") {
      const bounds = prompt(
        "Enter the lower and upper bounds (e.g., 0,1000000)",
        "0,1000000"
      );

      if (bounds === null || bounds === "") return;
      if (!bounds.includes(",")) {
        setExecutionResult({
          success: false,
          message: "Invalid bounds format. Use 'lower,upper'.",
        });
        return;
      }
      if (bounds.split(",").length !== 2) {
        setExecutionResult({
          success: false,
          message: "Please provide exactly two bounds.",
        });
        return;
      }
      if (isNaN(bounds.split(",")[0]) || isNaN(bounds.split(",")[1])) {
        setExecutionResult({
          success: false,
          message: "Bounds must be numbers.",
        });
        return;
      }

      LOWER_BOUND = parseInt(bounds.split(",")[0]);
      UPPER_BOUND = parseInt(bounds.split(",")[1]);
    }

    if (type === null || type === "") return;

    // const randomArray = Array.from({ length: parseInt(range) }, () => {
    //   if (type === "i32") {
    //     return (
    //       Math.floor(Math.random() * (UPPER_BOUND - LOWER_BOUND + 1)) +
    //       LOWER_BOUND
    //     );
    //   } else if (type === "f64") {
    //     return Math.random() * (UPPER_BOUND - LOWER_BOUND) + LOWER_BOUND;
    //   } else if (type === "String") {
    //     const chars =
    //       "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    //     let result = "";
    //     for (let i = 0; i < 10; i++) {
    //       result += chars.charAt(Math.floor(Math.random() * chars.length));
    //     }
    //     return result;
    //   }
    // });
    const result = await invoke("generate_random_array", {
      count: parseInt(range), // Ensure the range is an integer
      dataType: type, // Pass the type as a string ("i32", "f64", or "String")
      min: LOWER_BOUND, // Pass the lower bound
      max: UPPER_BOUND, // Pass the upper bound
    });

    // const toBeJson = { type: type, arr: randomArray };

    setCode(JSON.stringify(result, null, 2));
  };

  return (
    <div className="relative">
      {/* Floating window effect */}
      <div className="absolute -inset-2 bg-blue-500/10 rounded-xl blur-lg opacity-30 -z-10"></div>

      {/* Main container */}
      <div className="code-block-container bg-[#0A0E15] rounded-lg border border-gray-700/80 overflow-hidden shadow-2xl shadow-blue-500/10 transition-all duration-300 hover:shadow-blue-500/20">
        {/* Window header */}
        <div className="window-header bg-[#0A0E12] px-4 py-2 flex items-center border-b border-gray-700/50">
          <div className="window-controls flex space-x-2 mr-4">
            <div className="w-3 h-3 rounded-full bg-[#ff5f56]"></div>
            <div className="w-3 h-3 rounded-full bg-[#ffbd2e]"></div>
            <div className="w-3 h-3 rounded-full bg-[#27c93f]"></div>
          </div>
          <div className="text-xs text-gray-400 font-medium">settings.json</div>
          <div className="ml-auto flex space-x-1.5">
            <div className="w-2 h-2 rounded-full bg-gray-600/80"></div>
            <div className="w-2 h-2 rounded-full bg-gray-600/80"></div>
            <div className="w-2 h-2 rounded-full bg-gray-600/80"></div>
          </div>
        </div>
        <div className="overflow-y-auto h-[400px]">
          <Editor
            value={code}
            onValueChange={setCode}
            highlight={(code) =>
              Prism.highlight(code, Prism.languages.json, "json")
            }
            padding={10}
            placeholder="Code Here"
            onClick={(e) => {
              const textarea = e.target;
              const lineNumber = (
                textarea.value
                  .substr(0, textarea.selectionStart)
                  .match(/\n/g) || []
              ).length;
              setActiveLine(lineNumber);
            }}
            className="w-full  bg-[#0A0E15] text-gray-300 p-4 font-mono text-[17px] leading-snug outline-none resize-none"
          />
        </div>

        {/* Status bar */}
        <div className="bg-gray-950 text-white px-3 py-1.5 flex items-center justify-between text-xs">
          <div className="flex items-center space-x-4">
            <span>JSON</span>
            <span className="opacity-80">UTF-8</span>
            {executionResult && (
              <span
                className={`${
                  executionResult.success ? "text-green-400" : "text-red-400"
                }`}
              >
                {executionResult.message}
              </span>
            )}
          </div>
          <div className="flex items-center space-x-2">
            <span className="opacity-90">
              Ln {activeLine !== null ? activeLine + 1 : "--"}
            </span>
            <span className="opacity-70">•</span>
            <span className="opacity-90">
              Col{" "}
              {activeLine !== null
                ? code.substring(0, code.indexOf("\n", activeLine)).length + 1
                : "--"}
            </span>
          </div>
        </div>

        {/* Floating action buttons */}
        <div className="absolute right-4 bottom-16 flex space-x-3">
          <label className="px-3 py-1.5 bg-gray-700/90 hover:bg-gray-600/90 text-gray-300 rounded-md text-xs font-medium flex items-center shadow-md transition-all duration-200 hover:shadow-lg hover:translate-y-[-1px] cursor-pointer">
            <input
              type="file"
              accept=".json"
              onChange={handleImport}
              className="hidden"
            />
            <svg
              className="w-3.5 h-3.5 mr-1.5 opacity-80"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
              />
            </svg>
            Import
          </label>
          <button
            onClick={onRandomClick}
            className={`px-3 py-1.5 ${
              isExecuting ? "bg-[#3c2489]" : "bg-[#5439ac] hover:bg-[#5439ac]"
            } text-white rounded-md text-xs font-medium flex items-center shadow-md transition-all duration-200 hover:shadow-lg hover:translate-y-[-1px]`}
          >
            <Dices size={14} className="mr-2" />
            Random Input
          </button>
          <button
            onClick={handleExecute}
            disabled={isExecuting}
            className={`px-3 py-1.5 ${
              isExecuting ? "bg-[#005a8c]" : "bg-[#007acc] hover:bg-[#006bb3]"
            } text-white rounded-md text-xs font-medium flex items-center shadow-md transition-all duration-200 hover:shadow-lg hover:translate-y-[-1px]`}
          >
            {isExecuting ? (
              <>
                <svg
                  className="animate-spin -ml-1 mr-1.5 h-3.5 w-3.5 text-white"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                  ></circle>
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  ></path>
                </svg>
                Executing...
              </>
            ) : (
              <>
                <svg
                  className="w-3.5 h-3.5 mr-1.5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
                  />
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
                Execute
              </>
            )}
          </button>
        </div>
      </div>

      {/* Execution result panel - only show errors */}
      {executionResult && !executionResult.success && (
        <div className="mt-4 p-4 rounded-lg bg-red-900/20 border border-red-800/50">
          <div className="font-medium mb-2 text-sm text-red-400">
            ❌ Execution Failed
          </div>
          <div className="text-xs text-red-200">{executionResult.message}</div>
        </div>
      )}
    </div>
  );
};

export default JsonCodeBlock;
