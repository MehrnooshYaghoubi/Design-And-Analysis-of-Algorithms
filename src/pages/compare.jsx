import { NavLink } from "react-router-dom";
import Graph from "../components/graph";
import Header from "../components/titlebar";
import Panel from "../components/Panel";
import {
  ChevronDown,
  ChevronRight,
  Home,
  Play,
  Square,
  SquareFunction,
  X,
} from "lucide-react";
import { useState, useRef, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export default function Compare() {
  const rangeRef = useRef(null);
  const datatypeRef = useRef(null);
  const minRef = useRef(null);
  const maxRef = useRef(null);
  const [algorithmsVisible, setAlgorithmsVisible] = useState(false);
  const [selectedAlgos, setSelectedAlgos] = useState([]);
  const [data, setData] = useState([]);
  const [isRunning, setRunning] = useState(false);

  const formatNano = (nanos) => {
    if (nanos < 1_000_000) {
      // Less than 1ms
      return { duration: nanos, unit: "ns" };
    } else if (nanos < 1_000_000_000) {
      // Less than 1s
      return { duration: (nanos / 1_000_000).toFixed(2), unit: "ms" };
    } else if (nanos < 60_000_000_000) {
      // Less than 1 min
      return { duration: (nanos / 1_000_000_000).toFixed(2), unit: "s" };
    } else if (nanos < 3_600_000_000_000) {
      // Less than 1 hr
      return { duration: (nanos / 60_000_000_000).toFixed(2), unit: "min" };
    } else {
      return {
        duration: (nanos / 3_600_000_000_000).toFixed(2),
        unit: "h",
      };
    }
  };

  useEffect(() => {
    let unlisten;

    listen("algorithm-finished", (event) => {
      const { name, duration_ns } = event.payload;
      const duration_ms = parseFloat((duration_ns / 1_000_000).toFixed(4));
      console.log({ name, duration_ms });

      setData((prev) => {
        const exists = prev.some((e) => e.name === name);
        if (exists) {
          return prev.map((e) =>
            e.name === name ? { ...e, duration: duration_ms } : e
          );
        } else {
          return [...prev, { name, duration: duration_ms }];
        }
      });
    }).then((f) => {
      unlisten = f;
    });

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const onPlayClick = async () => {
    setData([]);
    const range = rangeRef.current.value;
    let data_type = datatypeRef.current.value;
    const min = minRef.current.value;
    const max = maxRef.current.value;

    if (isRunning) {
      return;
    }

    if (!range || !data_type || !min || !max) {
      window.alert("all fields are required");
      return;
    }

    if (selectedAlgos.length == 0) {
      window.alert("you should select an algorithm");
      return;
    }

    if (selectedAlgos.some((e) => !e.includes("sort"))) {
      window.alert("Selected Algorithm Is Not Supported Yet");
      return;
    }

    data_type = data_type.toLowerCase();

    if (selectedAlgos.includes("radix sort") && data_type != "i32") {
      window.alert(
        "Selected Algorithm Radix Sort Only Support i32 (Integer Type)"
      );
      return;
    }

    // Sending Data TO Backend
    try {
      setRunning(true);
      for (let algo of selectedAlgos) {
        const inputData = {
          range: parseInt(range),
          data_type: data_type,
          min: parseFloat(min),
          max: parseFloat(max),
          selected_algorithm: algo,
        };
        console.log({ inputData });
        await invoke("generate_random", { jsonData: inputData });
      }
      console.log("Command sent, streaming results...");
    } catch (err) {
      console.error("Error calling generate_random:", err);
    }
    setRunning(false);
  };

  const onCancel = async () => {
    await invoke("cancel");
    setRunning(false);
  };

  return (
    <div>
      <Header />
      <div className="h-[calc(100vh-50px)] flex">
        {/* Sidebar Bar */}
        <div className="w-80 backdrop-blur-md p-4 h-full  border-r border-gray-800 overflow-y-auto pb-[5em]">
          <h2 className="text-xl font-bold mb-6 text-emerald-300 flex items-center">
            <span className="w-3 h-3 bg-emerald-300 rounded-full mr-2 animate-pulse"></span>
            Compare Algortihms
          </h2>

          <ul className="space-y-3">
            <li className="group">
              <NavLink
                to="/"
                className="text-sm flex items-center p-2 text-gray-300 hover:text-[oklch(91%_0.096_180.426)] hover:bg-gray-700/50 rounded-lg transition-all duration-200 border-l-2 border-transparent group-hover:border-[oklch(91%_0.096_180.426)]"
              >
                <Home className="w-5 h-5  mr-4 transition-opacity" />
                Home Page
              </NavLink>
            </li>

            <li
              className="group"
              onClick={() => setAlgorithmsVisible(!algorithmsVisible)}
            >
              <a className="text-sm flex items-center p-2 w-full text-gray-300 hover:text-[oklch(91%_0.096_180.426)] hover:bg-gray-700/50 rounded-lg transition-all duration-200 border-l-2 border-transparent group-hover:border-[oklch(91%_0.096_180.426)]">
                <span className="w-2 h-2 bg-[oklch(91%_0.096_180.426)] rounded-full mr-3 opacity-0 group-hover:opacity-100 transition-opacity"></span>
                Algorithms
                {algorithmsVisible ? (
                  <ChevronDown size={15} className="ml-auto" />
                ) : (
                  <ChevronRight size={15} className="ml-auto" />
                )}
              </a>
            </li>

            {algorithmsVisible && (
              <Panel
                click={(n) => {
                  setSelectedAlgos((prev) =>
                    prev.includes(n) ? prev : [...prev, n]
                  );
                }}
              />
            )}
          </ul>
        </div>
        {/* Graph and info */}
        <div className="flex flex-col justify-between w-full p-5 font-[Montserrat]">
          <Graph data={data} />

          <div className="w-full backdrop-blur-md rounded-xl border flex justify-between border-gray-800 p-5 shadow-lg shadow-emerald-500/5 min-w-[400px] h-fit">
            {/* Selected Algos */}
            <div className="max-w-[50%]">
              <h3 className="font-bold text-lg ">Control Panel</h3>
              <h4 className="flex items-center mt-3 mb-6 font-bold">
                <SquareFunction className="mr-1 " /> Selected Algoritms
              </h4>
              {/* Selected Algos */}
              <div>
                {selectedAlgos.length > 0 ? (
                  <div className="flex flex-wrap gap-2">
                    {selectedAlgos.map((e) => (
                      <div
                        className="flex items-center bg-emerald-100 py-1 rounded-lg text-sm font-[Roboto] px-2 text-emerald-600"
                        onClick={() =>
                          setSelectedAlgos((prev) =>
                            prev.filter((x) => x !== e)
                          )
                        }
                      >
                        <X
                          size={12}
                          className="mr-1 cursor-pointer hover:opacity-60"
                        />
                        {e}
                      </div>
                    ))}
                  </div>
                ) : (
                  <h4 className="bg-red-100 text-red-500 font-mono py-1 px-3 text-sm select-none rounded-xl w-fit text-nowrap">
                    No Algorithm Selected
                  </h4>
                )}
              </div>
            </div>
            {/* Buttons And Input */}
            <div className="flex flex-col mt-5 font-[Roboto]">
              <div className="w-fit grid gap-2 justify-end items-end grid-rows-2 grid-cols-2">
                {/* Range */}
                <input
                  ref={rangeRef}
                  type="number"
                  id="range"
                  placeholder="Range"
                  className="border-1 mt-2 rounded-lg border-gray-800 px-2 py-1 w-full col-start-1 col-end-3"
                />
                <input
                  ref={datatypeRef}
                  type="text"
                  id="range"
                  placeholder="Data Type (String, I32, F64)"
                  className="border-1 mt-2 rounded-lg border-gray-800 px-2 py-1 w-full col-start-1 col-end-3"
                />
                <input
                  ref={minRef}
                  type="number"
                  placeholder="Min"
                  id="range"
                  className="border-1 mt-2 rounded-lg border-gray-800 px-2 py-1 w-[150px]"
                />
                <input
                  ref={maxRef}
                  type="number"
                  id="range"
                  placeholder="Max"
                  className="border-1 mt-2 rounded-lg border-gray-800 px-2 py-1 w-[150px]"
                />
              </div>
              {/* Buttons */}
              <div className="mt-3 flex gap-2 justify-end">
                <p>{isRunning ? "Running..." : ""}</p>
                <button
                  className="font-[Montserrat] text-sm bg-blue-100 text-blue-500 px-3 py-2 rounded-full flex items-center cursor-pointer hover:bg-blue-200"
                  onClick={onPlayClick}
                >
                  Start
                  <Play size={14} className="ml-2" />
                </button>
                <button
                  className="font-[Montserrat] text-sm bg-amber-100 text-amber-500 px-3 py-2 rounded-full flex items-center cursor-pointer hover:bg-amber-200"
                  onClick={onCancel}
                >
                  Stop
                  <Square size={14} className="ml-2" />
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
