/** Nanoseconds to a short human string with a sensible unit. */
export function formatDuration(nanos) {
  if (nanos == null || Number.isNaN(nanos)) return "—";
  if (nanos < 1_000) return `${nanos} ns`;
  if (nanos < 1_000_000) return `${(nanos / 1_000).toFixed(2)} µs`;
  if (nanos < 1_000_000_000) return `${(nanos / 1_000_000).toFixed(2)} ms`;
  if (nanos < 60_000_000_000) return `${(nanos / 1_000_000_000).toFixed(2)} s`;
  return `${(nanos / 60_000_000_000).toFixed(2)} min`;
}

/** Picks one unit for a whole series, so bars stay comparable. */
export function pickUnit(maxNanos) {
  if (!maxNanos || maxNanos < 1_000) return { unit: "ns", divisor: 1 };
  if (maxNanos < 1_000_000) return { unit: "µs", divisor: 1_000 };
  if (maxNanos < 1_000_000_000) return { unit: "ms", divisor: 1_000_000 };
  return { unit: "s", divisor: 1_000_000_000 };
}

export const formatNumber = (n) =>
  typeof n === "number" ? n.toLocaleString() : String(n);
