import { Trash2 } from "lucide-react";
import Titlebar from "../components/titlebar";
import Sidebar from "../components/Sidebar";
import JsonCodeBlock from "../components/JsonCodeBlock";
import ResultCard from "../components/ResultCard";
import { useAppStore, MAX_RESULTS } from "../store";

export default function Algorithms() {
  const results = useAppStore((s) => s.results);
  const clearResults = useAppStore((s) => s.clearResults);

  return (
    <main className="flex flex-col h-full">
      <Titlebar />

      <div className="flex flex-1 min-h-0">
        <Sidebar title="Laboratory" />

        <div className="flex-1 min-h-0 grid grid-cols-1 lg:grid-cols-2 gap-5 p-5">
          <section className="min-h-0 flex flex-col">
            <JsonCodeBlock />
          </section>

          <section className="min-h-0 flex flex-col">
            <div className="flex items-center justify-between mb-3">
              <span className="label">
                Results
                {results.length > 0 && (
                  <span className="num" style={{ color: "var(--fg-faint)" }}>
                    {"  "}
                    {results.length}
                    {results.length >= MAX_RESULTS ? ` (last ${MAX_RESULTS})` : ""}
                  </span>
                )}
              </span>
              {results.length > 0 && (
                <button
                  className="btn btn-icon"
                  onClick={clearResults}
                  title="Clear results"
                  aria-label="Clear results"
                >
                  <Trash2 size={13} />
                </button>
              )}
            </div>

            <div className="flex-1 min-h-0 overflow-y-auto flex flex-col gap-3 pr-1">
              {results.length === 0 ? (
                <div
                  className="surface flex-1 grid place-items-center p-8 text-center"
                  style={{ color: "var(--fg-faint)" }}
                >
                  <div>
                    <p>No results yet.</p>
                    <p className="mt-1 text-xs">
                      Pick an algorithm, then press Run.
                    </p>
                  </div>
                </div>
              ) : (
                results.map((result) => (
                  <ResultCard key={result.id} result={result} />
                ))
              )}
            </div>
          </section>
        </div>
      </div>
    </main>
  );
}
