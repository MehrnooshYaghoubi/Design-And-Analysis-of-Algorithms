import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Play, Square, X } from "lucide-react";
import Titlebar from "../components/titlebar";
import Sidebar from "../components/Sidebar";
import Graph from "../components/graph";

const DATA_TYPES = ["i32", "f64", "String"];

/// Only the sorting family can be benchmarked: `generate_random` builds an
/// array and applies a sort. The previous version let you select anything and
/// then refused with an alert after the fact.
const isSortable = (name) => name.includes("sort");

/// A guard against picking a quadratic sort on a huge array by accident.
const QUADRATIC = ["bubble sort", "insertion sort", "selection sort"];
const QUADRATIC_WARN_AT = 200_000;

export default function Compare() {
  const [selectedAlgos, setSelectedAlgos] = useState([]);
  const [results, setResults] = useState([]);
  const [isRunning, setRunning] = useState(false);
  const [progress, setProgress] = useState(null);
  const [error, setError] = useState(null);

  const [settings, setSettings] = useState({
    range: 20000,
    dataType: "i32",
    min: 0,
    max: 1000000,
  });

  // Set when Stop is pressed, so the loop can break between algorithms.
  const stopped = useRef(false);

  useEffect(
    () => () => {
      // Leaving the page must not leave a benchmark running.
      stopped.current = true;
      invoke("cancel").catch(() => {});
    },
    []
  );

  const toggleAlgo = useCallback((name) => {
    setSelectedAlgos((prev) =>
      prev.includes(name) ? prev.filter((n) => n !== name) : [...prev, name]
    );
  }, []);

  const update = (key, value) =>
    setSettings((prev) => ({ ...prev, [key]: value }));

  const validation = useMemo(() => {
    const { range, dataType, min, max } = settings;
    if (!Number.isFinite(range) || range < 1) return "Range must be at least 1.";
    if (!Number.isFinite(min) || !Number.isFinite(max))
      return "Min and max must be numbers.";
    if (min > max) return "Min must not exceed max.";
    if (selectedAlgos.length === 0) return "Select at least one sorting algorithm.";
    if (selectedAlgos.includes("radix sort") && dataType !== "i32")
      return "Radix sort only supports the i32 type.";
    return null;
  }, [settings, selectedAlgos]);

  const slowWarning = useMemo(() => {
    const picked = selectedAlgos.filter((a) => QUADRATIC.includes(a));
    if (picked.length && settings.range >= QUADRATIC_WARN_AT) {
      return `${picked.join(", ")} ${
        picked.length > 1 ? "are" : "is"
      } quadratic; ${settings.range.toLocaleString()} elements will take a while. Stop is available.`;
    }
    return null;
  }, [selectedAlgos, settings.range]);

  const onStart = useCallback(async () => {
    if (isRunning || validation) return;

    setError(null);
    setResults([]);
    setRunning(true);
    stopped.current = false;

    try {
      // Cleared once per batch, not per algorithm. The old code reset the flag
      // inside every run, so pressing Stop only skipped one algorithm and the
      // rest started anyway.
      await invoke("reset_cancel");

      for (const [index, algo] of selectedAlgos.entries()) {
        if (stopped.current) break;
        setProgress({ algo, index: index + 1, total: selectedAlgos.length });

        // Genuinely awaited now: the command used to return as soon as the
        // worker thread was spawned, so every algorithm raced at once and the
        // reported timings were contended nonsense.
        const result = await invoke("generate_random", {
          jsonData: {
            range: settings.range,
            data_type: settings.dataType.toLowerCase(),
            min: settings.min,
            max: settings.max,
            selected_algorithm: algo,
          },
        });

        setResults((prev) => [
          ...prev.filter((r) => r.name !== result.name),
          {
            name: result.name,
            durationNs: result.duration_ns,
            length: result.length,
            cancelled: result.cancelled,
          },
        ]);

        if (result.cancelled) break;
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setProgress(null);
      setRunning(false);
    }
  }, [isRunning, validation, selectedAlgos, settings]);

  const onStop = useCallback(async () => {
    stopped.current = true;
    try {
      await invoke("cancel");
    } catch (err) {
      setError(String(err));
    }
  }, []);

  return (
    <main className="flex flex-col h-full">
      <Titlebar />

      <div className="flex flex-1 min-h-0">
        <Sidebar
          title="Compare"
          onSelect={toggleAlgo}
          isEnabled={isSortable}
          selected={selectedAlgos}
        />

        <div className="flex-1 min-h-0 flex flex-col gap-5 p-5">
          <div className="flex-1 min-h-0">
            <Graph data={results} />
          </div>

          <section className="surface p-5 shrink-0">
            <div className="flex flex-wrap gap-8">
              {/* Selection */}
              <div className="flex-1 min-w-[16rem]">
                <h2 className="label mb-3">
                  Selected · {selectedAlgos.length}
                </h2>
                {selectedAlgos.length === 0 ? (
                  <p style={{ color: "var(--fg-faint)" }}>
                    Choose sorting algorithms from the sidebar.
                  </p>
                ) : (
                  <div className="flex flex-wrap gap-2">
                    {selectedAlgos.map((name) => (
                      <button
                        key={name}
                        className="tag"
                        onClick={() => toggleAlgo(name)}
                        title={`Remove ${name}`}
                      >
                        <X size={11} />
                        {name}
                      </button>
                    ))}
                  </div>
                )}
              </div>

              {/* Settings */}
              <div className="shrink-0">
                <h2 className="label mb-3">Dataset</h2>
                <div className="grid grid-cols-2 gap-3" style={{ width: "22rem" }}>
                  <label className="flex flex-col gap-1 col-span-2">
                    <span className="label">Elements</span>
                    <input
                      className="input num"
                      type="number"
                      min={1}
                      value={settings.range}
                      onChange={(e) => update("range", Number(e.target.value))}
                    />
                  </label>

                  <label className="flex flex-col gap-1 col-span-2">
                    <span className="label">Type</span>
                    <select
                      className="input"
                      value={settings.dataType}
                      onChange={(e) => update("dataType", e.target.value)}
                    >
                      {DATA_TYPES.map((t) => (
                        <option key={t} value={t}>
                          {t}
                        </option>
                      ))}
                    </select>
                  </label>

                  <label className="flex flex-col gap-1">
                    <span className="label">Min</span>
                    <input
                      className="input num"
                      type="number"
                      value={settings.min}
                      onChange={(e) => update("min", Number(e.target.value))}
                    />
                  </label>

                  <label className="flex flex-col gap-1">
                    <span className="label">Max</span>
                    <input
                      className="input num"
                      type="number"
                      value={settings.max}
                      onChange={(e) => update("max", Number(e.target.value))}
                    />
                  </label>
                </div>
              </div>
            </div>

            <hr className="rule my-4" />

            <div className="flex items-center gap-4">
              <div className="flex-1 text-xs" style={{ color: "var(--fg-muted)" }}>
                {error ? (
                  <span style={{ color: "var(--bad)" }}>{error}</span>
                ) : progress ? (
                  <span>
                    Running {progress.algo} ({progress.index}/{progress.total})…
                  </span>
                ) : (
                  validation ?? slowWarning ?? "Ready."
                )}
              </div>

              <button
                className="btn btn-primary"
                onClick={onStart}
                disabled={isRunning || !!validation}
                title={validation ?? "Run the benchmark"}
              >
                <Play size={13} />
                Start
              </button>
              <button className="btn" onClick={onStop} disabled={!isRunning}>
                <Square size={13} />
                Stop
              </button>
            </div>
          </section>
        </div>
      </div>
    </main>
  );
}
