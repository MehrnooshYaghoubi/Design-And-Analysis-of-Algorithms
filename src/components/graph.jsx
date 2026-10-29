import { memo, useMemo, useState } from "react";
import { Table2, BarChart3 } from "lucide-react";
import { formatDuration, formatNumber, pickUnit } from "../format";
import { assignHues, pastel } from "../palette";
import { useAppStore } from "../store";

/** A small round chip in a series colour. */
function Swatch({ color }) {
  return (
    <span
      aria-hidden="true"
      className="inline-block shrink-0"
      style={{
        width: 8,
        height: 8,
        borderRadius: "50%",
        background: color,
      }}
    />
  );
}

/// Benchmark results as horizontal bars.
///
/// Every bar gets a pastel fill keyed to the *algorithm name*, not to its row,
/// so a colour belongs to one algorithm and survives a reordering: the bar for
/// quick sort is the same hue whether it lands first or last. (An earlier
/// version indexed the palette by array position, which encoded nothing -- the
/// colour changed whenever the ranking did; see palette.js.) The ordering
/// (fastest first) and the value labels still carry the comparison, so the
/// colour is identity only, and the swatch on each row is its own legend.
///
/// Rendered as plain HTML rather than a chart library: a handful of bars needs
/// no reconciliation layer, it inherits the theme tokens directly, and it drops
/// a heavy dependency off the render path.
function Graph({ data }) {
  const [view, setView] = useState("chart");
  const [hovered, setHovered] = useState(null);
  // Pastels need a different lightness on white than on near-black.
  const theme = useAppStore((s) => s.theme);

  const rows = useMemo(() => {
    const sorted = [...data].sort((a, b) => a.durationNs - b.durationNs);
    const max = sorted.length ? Math.max(...sorted.map((d) => d.durationNs)) : 0;
    const min = sorted.length ? sorted[0].durationNs : 0;
    // One unit for the whole series, so bar lengths stay comparable.
    const { unit, divisor } = pickUnit(max);
    // Keyed by name over the whole set, so the hues are decided once per
    // result set rather than per row.
    const hues = assignHues(sorted.map((d) => d.name));
    return {
      unit,
      divisor,
      max,
      hues,
      items: sorted.map((d) => ({
        ...d,
        // Guard against a zero max producing NaN width.
        pct: max > 0 ? (d.durationNs / max) * 100 : 0,
        scaled: d.durationNs / divisor,
        times: min > 0 ? d.durationNs / min : 1,
        hue: hues.get(d.name),
      })),
    };
  }, [data]);

  const hasData = rows.items.length > 0;

  return (
    <section className="surface flex flex-col min-h-0" aria-label="Benchmark results">
      <header
        className="flex items-center justify-between px-4 py-2"
        style={{ borderBottom: "1px solid var(--border)" }}
      >
        <div className="flex items-baseline gap-3">
          <h2 className="label">Elapsed time</h2>
          {hasData && (
            <span className="label" style={{ color: "var(--fg-faint)" }}>
              fastest first · {rows.unit}
            </span>
          )}
        </div>

        <div className="flex gap-1">
          <button
            className="btn btn-icon"
            onClick={() => setView("chart")}
            aria-pressed={view === "chart"}
            title="Chart view"
            style={
              view === "chart"
                ? { background: "var(--bg-subtle)", borderColor: "var(--border)" }
                : undefined
            }
          >
            <BarChart3 size={13} />
          </button>
          {/* A table view, so the data is never gated behind the graphic. */}
          <button
            className="btn btn-icon"
            onClick={() => setView("table")}
            aria-pressed={view === "table"}
            title="Table view"
            style={
              view === "table"
                ? { background: "var(--bg-subtle)", borderColor: "var(--border)" }
                : undefined
            }
          >
            <Table2 size={13} />
          </button>
        </div>
      </header>

      <div className="flex-1 min-h-0 overflow-y-auto p-4">
        {!hasData ? (
          <div
            className="h-full grid place-items-center text-center py-10"
            style={{ color: "var(--fg-faint)" }}
          >
            <div>
              <p>No benchmarks yet.</p>
              <p className="mt-1 text-xs">
                Select sorting algorithms, set a range, then press Start.
              </p>
            </div>
          </div>
        ) : view === "chart" ? (
          <div className="flex flex-col gap-1.5">
            {rows.items.map((item) => (
              <div
                key={item.name}
                className="grid items-center gap-3"
                style={{ gridTemplateColumns: "minmax(7rem, 11rem) 1fr auto" }}
                onMouseEnter={() => setHovered(item.name)}
                onMouseLeave={() => setHovered(null)}
              >
                <span
                  className="flex items-center gap-2 text-xs capitalize truncate"
                  style={{ color: "var(--fg-muted)" }}
                  title={item.name}
                >
                  {/* The swatch doubles as the legend, so no separate key. */}
                  <Swatch color={pastel(item.hue, theme)} />
                  <span className="truncate">{item.name}</span>
                </span>

                {/* Track + bar. Rounded data-end, square at the baseline so
                    the bars still read against a common axis. */}
                <div
                  className="relative"
                  style={{
                    height: 20,
                    background: "var(--bg-inset)",
                    borderLeft: "1px solid var(--border-strong)",
                    borderRadius: "0 var(--radius-sm) var(--radius-sm) 0",
                  }}
                >
                  <div
                    style={{
                      width: `${Math.max(item.pct, 0.5)}%`,
                      height: "100%",
                      background: pastel(item.hue, theme, hovered === item.name),
                      borderRadius: "0 var(--radius-sm) var(--radius-sm) 0",
                      transition: "background var(--dur) ease",
                    }}
                  />
                  {hovered === item.name && (
                    <div
                      role="tooltip"
                      className="absolute left-2 top-full mt-1 z-10 px-2 py-1 text-xs whitespace-nowrap"
                      style={{
                        background: "var(--bg-raised)",
                        border: "1px solid var(--border-strong)",
                        borderRadius: "var(--radius)",
                        boxShadow: "var(--shadow)",
                        color: "var(--fg)",
                      }}
                    >
                      <Swatch color={pastel(item.hue, theme, true)} />{" "}
                      <span className="capitalize">{item.name}</span>
                      {" · "}
                      <span className="num">{formatDuration(item.durationNs)}</span>
                      {item.length ? (
                        <>
                          {" · "}
                          <span className="num">{formatNumber(item.length)}</span>
                          {" items"}
                        </>
                      ) : null}
                      {item.times > 1.01 && (
                        <>
                          {" · "}
                          <span className="num">{item.times.toFixed(1)}×</span>
                          {" slowest-relative"}
                        </>
                      )}
                      {item.cancelled && " · cancelled"}
                    </div>
                  )}
                </div>

                {/* Value at the tip: every bar is labelled, so no axis ticks. */}
                <span className="num text-xs tabular-nums" style={{ minWidth: "5rem", textAlign: "right" }}>
                  {item.scaled < 10
                    ? item.scaled.toFixed(2)
                    : item.scaled.toFixed(0)}
                  <span style={{ color: "var(--fg-faint)" }}> {rows.unit}</span>
                </span>
              </div>
            ))}
          </div>
        ) : (
          <table className="w-full text-xs">
            <thead>
              <tr style={{ borderBottom: "1px solid var(--border)" }}>
                <th className="text-left py-1.5 label">Algorithm</th>
                <th className="text-right py-1.5 label">Items</th>
                <th className="text-right py-1.5 label">Elapsed</th>
                <th className="text-right py-1.5 label">Relative</th>
              </tr>
            </thead>
            <tbody>
              {rows.items.map((item) => (
                <tr key={item.name} style={{ borderBottom: "1px solid var(--border)" }}>
                  <td className="py-1.5 capitalize">
                    <span className="flex items-center gap-2">
                      <Swatch color={pastel(item.hue, theme)} />
                      {item.name}
                    </span>
                  </td>
                  <td className="py-1.5 text-right num">
                    {item.length ? formatNumber(item.length) : "—"}
                  </td>
                  <td className="py-1.5 text-right num">
                    {formatDuration(item.durationNs)}
                  </td>
                  <td className="py-1.5 text-right num">
                    {item.times.toFixed(2)}×
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>
    </section>
  );
}

export default memo(Graph);
