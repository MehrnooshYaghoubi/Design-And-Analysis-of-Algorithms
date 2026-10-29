import { NavLink } from "react-router-dom";
import { ArrowRight, GitCompare } from "lucide-react";
import Titlebar from "./components/titlebar";

const FAMILIES = [
  { name: "Sorting", count: 9 },
  { name: "Divide & conquer", count: 4 },
  { name: "Greedy", count: 5 },
  { name: "Graph", count: 4 },
  { name: "Dynamic programming", count: 5 },
  { name: "Backtracking", count: 4 },
  { name: "Branch & bound", count: 2 },
];

export default function App() {
  const total = FAMILIES.reduce((sum, f) => sum + f.count, 0);

  return (
    <main className="flex flex-col h-full">
      <Titlebar />

      <div className="flex-1 min-h-0 overflow-y-auto">
        <div className="max-w-4xl mx-auto px-6 py-16">
          <p className="label">Algorithms laboratory</p>

          <h1
            className="mt-6 font-semibold tracking-tight"
            style={{
              fontFamily: "var(--font-display)",
              fontSize: "var(--step-5)",
              lineHeight: 1.05,
            }}
          >
            KlugNetz
          </h1>

          <p
            className="mt-6 max-w-2xl"
            style={{ fontSize: "var(--step-2)", color: "var(--fg-muted)" }}
          >
            {total} algorithms implemented in Rust, run natively and measured
            precisely. Feed one a JSON payload and read the result, or put
            several against the same dataset and compare how they scale.
          </p>

          <div className="flex flex-wrap gap-3 mt-10">
            <NavLink to="/algorithms" className="btn btn-primary">
              Open the laboratory
              <ArrowRight size={14} />
            </NavLink>
            <NavLink to="/compare" className="btn">
              <GitCompare size={14} />
              Compare performance
            </NavLink>
          </div>

          <hr className="rule mt-16" />

          <section className="mt-8">
            <h2 className="label mb-4">Families</h2>
            <ul
              className="grid gap-x-8 gap-y-0"
              style={{ gridTemplateColumns: "repeat(auto-fit, minmax(14rem, 1fr))" }}
            >
              {FAMILIES.map((family) => (
                <li
                  key={family.name}
                  className="flex items-baseline justify-between py-2.5"
                  style={{ borderBottom: "1px solid var(--border)" }}
                >
                  <span>{family.name}</span>
                  <span className="num" style={{ color: "var(--fg-faint)" }}>
                    {family.count}
                  </span>
                </li>
              ))}
            </ul>
          </section>

          <hr className="rule mt-12" />

          <footer
            className="mt-8 flex flex-wrap items-baseline justify-between gap-4 pb-8"
            style={{ color: "var(--fg-faint)" }}
          >
            <p>
              Created by{" "}
              <span style={{ color: "var(--fg)", fontWeight: 500 }}>
                Mehrnoosh Yaghoubi Kalourazi
              </span>
            </p>
            <p className="text-xs">Built with Rust, Tauri and React</p>
          </footer>
        </div>
      </div>
    </main>
  );
}
