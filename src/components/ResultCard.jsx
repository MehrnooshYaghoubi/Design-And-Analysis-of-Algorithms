import { memo, useMemo, useState } from "react";
import { formatDuration, formatNumber } from "../format";

/// JSON beyond this many characters is collapsed behind a toggle. Rendering a
/// large payload into the DOM is exactly what made the results panel freeze.
const COLLAPSE_CHARS = 1_200;

/// Even expanded, never hand the DOM more than this.
const HARD_CAP = 20_000;

/** A sorting result: head/tail preview plus a verified `sorted` flag. */
function SortSummary({ data }) {
  const preview = (values) => values.map((v) => JSON.stringify(v)).join(", ");
  return (
    <div className="flex flex-col gap-2">
      <div className="flex flex-wrap gap-2">
        <span className="tag">{formatNumber(data.length)} elements</span>
        <span className="tag">{data.sorted ? "verified sorted" : "NOT sorted"}</span>
        {data.cancelled && <span className="tag">cancelled</span>}
      </div>
      <div
        className="num text-xs p-2 overflow-x-auto"
        style={{
          background: "var(--bg-inset)",
          border: "1px solid var(--border)",
          borderRadius: "var(--radius)",
          color: "var(--fg-muted)",
        }}
      >
        <div className="whitespace-pre-wrap break-all">
          {preview(data.head)}
          {data.truncated && (
            <>
              <span style={{ color: "var(--fg-faint)" }}>
                {"  …  "}
                {formatNumber(data.length - data.head.length - data.tail.length)}
                {" more  …  "}
              </span>
              {preview(data.tail)}
            </>
          )}
        </div>
      </div>
    </div>
  );
}

/** Anything else: pretty JSON, collapsed when long. */
function JsonSummary({ payload }) {
  const [expanded, setExpanded] = useState(false);

  const text = useMemo(() => {
    try {
      return JSON.stringify(payload, null, 2);
    } catch {
      return String(payload);
    }
  }, [payload]);

  const long = text.length > COLLAPSE_CHARS;
  const shown = expanded ? text.slice(0, HARD_CAP) : text.slice(0, COLLAPSE_CHARS);

  return (
    <div className="flex flex-col gap-2">
      <pre
        className="num text-xs p-2 overflow-x-auto whitespace-pre-wrap break-all"
        style={{
          background: "var(--bg-inset)",
          border: "1px solid var(--border)",
          borderRadius: "var(--radius)",
          color: "var(--fg-muted)",
          margin: 0,
        }}
      >
        {shown}
        {!expanded && long && "\n…"}
      </pre>
      {long && (
        <div className="flex items-center gap-3">
          <button className="btn" onClick={() => setExpanded((v) => !v)}>
            {expanded ? "Collapse" : `Show more (${formatNumber(text.length)} chars)`}
          </button>
          {expanded && text.length > HARD_CAP && (
            <span className="label">truncated at {formatNumber(HARD_CAP)} chars</span>
          )}
        </div>
      )}
    </div>
  );
}

function ResultCard({ result }) {
  const { payload } = result;
  // The sorting handlers return this exact shape.
  const isSort =
    payload &&
    typeof payload === "object" &&
    "sorted" in payload &&
    Array.isArray(payload.head);

  return (
    <article
      className="surface p-3"
      style={{ background: "var(--bg-subtle)" }}
    >
      <header className="flex items-baseline justify-between gap-3 mb-2">
        <h3 className="font-medium capitalize truncate">{result.algorithm}</h3>
        <span className="num text-xs" style={{ color: "var(--fg-faint)" }}>
          {result.time}
        </span>
      </header>

      <div className="flex flex-wrap items-center gap-x-4 gap-y-1 mb-3 text-xs">
        <span>
          <span className="label">compute </span>
          <span className="num">{formatDuration(result.durationNs)}</span>
        </span>
        <span style={{ color: "var(--fg-faint)" }}>
          <span className="label">round trip </span>
          <span className="num">{result.roundTripMs.toFixed(2)} ms</span>
        </span>
      </div>

      {isSort ? <SortSummary data={payload} /> : <JsonSummary payload={payload} />}
    </article>
  );
}

export default memo(ResultCard);
