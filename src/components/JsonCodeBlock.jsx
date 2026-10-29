import { memo, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Editor from "react-simple-code-editor";
import Prism from "prismjs";
import "prismjs/components/prism-json";
import { Dices, Play, Upload, X } from "lucide-react";
import { useAppStore } from "../store";
import { generatorFor } from "./generators";

/// Beyond this many characters the editor stops syntax-highlighting.
/// Prism re-tokenises the whole document on every keystroke, so highlighting a
/// large payload makes typing unusable.
const HIGHLIGHT_LIMIT = 40_000;

/// Hard ceiling on what the editor will hold at all.
const MAX_EDITOR_CHARS = 2_000_000;

const escapeHtml = (s) =>
  s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");

let instructionsPromise = null;
function loadInstructions() {
  if (!instructionsPromise) {
    instructionsPromise = fetch("/instructions.json")
      .then((res) => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .catch((err) => {
        instructionsPromise = null;
        throw err;
      });
  }
  return instructionsPromise;
}

/** Inline parameter form, replacing a chain of unsupported prompt() calls. */
function RandomInputForm({ spec, algorithm, onGenerate, onCancel, busy }) {
  const [values, setValues] = useState(() =>
    Object.fromEntries(spec.params.map((p) => [p.key, p.default]))
  );

  // Reset the fields when the algorithm changes under us.
  useEffect(() => {
    setValues(Object.fromEntries(spec.params.map((p) => [p.key, p.default])));
  }, [spec]);

  const update = (key, raw, type) => {
    setValues((prev) => ({
      ...prev,
      [key]: type === "select" ? raw : raw === "" ? "" : Number(raw),
    }));
  };

  const invalid = spec.params.some((p) => {
    if (p.type === "select") return false;
    const v = values[p.key];
    if (v === "" || Number.isNaN(v)) return true;
    if (p.min !== undefined && v < p.min) return true;
    if (p.max !== undefined && v > p.max) return true;
    return false;
  });

  return (
    <form
      className="surface p-4"
      style={{ background: "var(--bg-subtle)" }}
      onSubmit={(e) => {
        e.preventDefault();
        if (!invalid && !busy) onGenerate(values);
      }}
    >
      <div className="flex items-center justify-between mb-3">
        <span className="label">Random input — {algorithm}</span>
        <button
          type="button"
          className="titlebar-btn"
          onClick={onCancel}
          aria-label="Close"
        >
          <X size={13} />
        </button>
      </div>

      <div className="grid grid-cols-2 gap-3">
        {spec.params.map((p) => (
          <label key={p.key} className="flex flex-col gap-1">
            <span className="label">{p.label}</span>
            {p.type === "select" ? (
              <select
                className="input"
                value={values[p.key]}
                onChange={(e) => update(p.key, e.target.value, p.type)}
              >
                {p.options.map((o) => (
                  <option key={o} value={o}>
                    {o}
                  </option>
                ))}
              </select>
            ) : (
              <input
                className="input num"
                type="number"
                step={p.type === "float" ? "any" : 1}
                min={p.min}
                max={p.max}
                value={values[p.key]}
                onChange={(e) => update(p.key, e.target.value, p.type)}
              />
            )}
          </label>
        ))}
      </div>

      <div className="flex justify-end gap-2 mt-4">
        <button type="button" className="btn" onClick={onCancel}>
          Cancel
        </button>
        <button type="submit" className="btn btn-primary" disabled={invalid || busy}>
          {busy ? "Generating…" : "Generate"}
        </button>
      </div>
    </form>
  );
}

function JsonCodeBlock() {
  const [code, setCode] = useState("");
  const [status, setStatus] = useState(null);
  const [isExecuting, setIsExecuting] = useState(false);
  const [isGenerating, setIsGenerating] = useState(false);
  const [showForm, setShowForm] = useState(false);
  const [cursor, setCursor] = useState({ line: 1, col: 1 });

  const fileInputRef = useRef(null);
  const selectedAlgorithm = useAppStore((s) => s.selectedAlgorithm);
  const addResult = useAppStore((s) => s.addResult);

  const generator = useMemo(() => generatorFor(selectedAlgorithm), [selectedAlgorithm]);

  // Load the sample payload for whichever algorithm is selected.
  // The previous version also ran a setInterval here that toggled an unused
  // `isFocused` flag every 800ms, re-rendering the editor and re-running Prism
  // over the whole document several times a second. That was the lag.
  useEffect(() => {
    if (!selectedAlgorithm) return;
    let alive = true;
    loadInstructions()
      .then((data) => {
        if (!alive) return;
        const sample = data[selectedAlgorithm];
        if (sample != null) {
          setCode(sample);
          setStatus(null);
        }
      })
      .catch((err) => alive && setStatus({ ok: false, message: err.message }));
    return () => {
      alive = false;
    };
  }, [selectedAlgorithm]);

  useEffect(() => {
    setShowForm(false);
  }, [selectedAlgorithm]);

  const tooLargeToHighlight = code.length > HIGHLIGHT_LIMIT;

  // Memoised so a re-render for an unrelated reason does not re-tokenise.
  const highlight = useCallback(
    (source) =>
      source.length > HIGHLIGHT_LIMIT
        ? escapeHtml(source)
        : Prism.highlight(source, Prism.languages.json, "json"),
    []
  );

  const trackCursor = useCallback((e) => {
    const ta = e.target;
    if (typeof ta.selectionStart !== "number") return;
    const upto = ta.value.slice(0, ta.selectionStart);
    const lines = upto.split("\n");
    setCursor({ line: lines.length, col: lines[lines.length - 1].length + 1 });
  }, []);

  const handleExecute = useCallback(async () => {
    if (!selectedAlgorithm) {
      setStatus({ ok: false, message: "Select an algorithm first." });
      return;
    }

    let parsed;
    try {
      parsed = JSON.parse(code);
    } catch (err) {
      setStatus({ ok: false, message: `Invalid JSON — ${err.message}` });
      return;
    }

    setIsExecuting(true);
    setStatus(null);
    const startedAt = performance.now();

    try {
      const response = await invoke("execute_json", {
        jsonData: parsed,
        selectedAlgo: selectedAlgorithm,
      });
      const roundTrip = performance.now() - startedAt;

      addResult({
        id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
        algorithm: response.algorithm ?? selectedAlgorithm,
        durationNs: response.duration_ns ?? 0,
        roundTripMs: roundTrip,
        time: new Date().toLocaleTimeString(),
        payload: response.result,
      });
      setStatus({ ok: true, message: "Done" });
    } catch (err) {
      setStatus({ ok: false, message: String(err) });
    } finally {
      setIsExecuting(false);
    }
  }, [code, selectedAlgorithm, addResult]);

  const handleGenerate = useCallback(
    async (values) => {
      if (!generator) return;
      setIsGenerating(true);
      setStatus(null);
      try {
        const payload = generator.invoke
          ? await invoke(generator.invoke, generator.args(values))
          : generator.local(values);

        const text = JSON.stringify(payload, null, 2);
        if (text.length > MAX_EDITOR_CHARS) {
          setStatus({
            ok: false,
            message:
              "That input is too large to display. Use the Compare page to " +
              "benchmark large datasets — it keeps the data in Rust.",
          });
          return;
        }

        setCode(text);
        setShowForm(false);
        setStatus({
          ok: true,
          message: text.length > HIGHLIGHT_LIMIT
            ? "Generated. Highlighting is off for this size."
            : "Generated",
        });
      } catch (err) {
        setStatus({ ok: false, message: String(err) });
      } finally {
        setIsGenerating(false);
      }
    },
    [generator]
  );

  const handleImport = useCallback((e) => {
    const file = e.target.files?.[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = (event) => {
      const text = String(event.target.result ?? "");
      if (text.length > MAX_EDITOR_CHARS) {
        setStatus({ ok: false, message: "That file is too large to open here." });
        return;
      }
      try {
        JSON.parse(text);
        setCode(text);
        setStatus({ ok: true, message: `Imported ${file.name}` });
      } catch (err) {
        setStatus({ ok: false, message: `Invalid JSON — ${err.message}` });
      }
    };
    reader.onerror = () =>
      setStatus({ ok: false, message: "Could not read that file." });
    reader.readAsText(file);

    // Reset so re-importing the same file fires a change event again.
    e.target.value = "";
  }, []);

  return (
    <div className="flex flex-col h-full min-h-0">
      <div className="flex items-center justify-between mb-3">
        <span className="label">Input</span>
        <span className="tag">{selectedAlgorithm ?? "no algorithm selected"}</span>
      </div>

      {showForm && generator && (
        <div className="mb-3">
          <RandomInputForm
            spec={generator}
            algorithm={selectedAlgorithm}
            busy={isGenerating}
            onGenerate={handleGenerate}
            onCancel={() => setShowForm(false)}
          />
        </div>
      )}

      <div
        className="surface flex-1 min-h-0 flex flex-col overflow-hidden"
        style={{ background: "var(--bg-inset)" }}
      >
        <div
          className="flex items-center justify-between px-3 py-1.5"
          style={{ borderBottom: "1px solid var(--border)" }}
        >
          <span className="label">input.json</span>
          {tooLargeToHighlight && (
            <span className="label" title="Prism is disabled above 40,000 characters">
              highlighting off
            </span>
          )}
        </div>

        <div className="flex-1 min-h-0 overflow-auto">
          <Editor
            value={code}
            onValueChange={setCode}
            highlight={highlight}
            padding={16}
            placeholder="Select an algorithm, or paste a JSON payload here."
            onClick={trackCursor}
            onKeyUp={trackCursor}
            onSelect={trackCursor}
            className="editor-shell"
            style={{ minHeight: "100%", color: "var(--fg)" }}
          />
        </div>

        <div
          className="flex items-center justify-between px-3 py-1.5 text-xs"
          style={{ borderTop: "1px solid var(--border)", color: "var(--fg-faint)" }}
        >
          <div className="flex items-center gap-3">
            <span>JSON</span>
            <span className="num">{code.length.toLocaleString()} chars</span>
            {status && (
              <span style={{ color: status.ok ? "var(--fg-muted)" : "var(--bad)" }}>
                {status.message}
              </span>
            )}
          </div>
          <span className="num">
            Ln {cursor.line}, Col {cursor.col}
          </span>
        </div>
      </div>

      <div className="flex items-center gap-2 mt-3">
        <input
          ref={fileInputRef}
          type="file"
          accept=".json,application/json"
          onChange={handleImport}
          className="hidden"
        />
        <button
          className="btn"
          onClick={() => fileInputRef.current?.click()}
          title="Import a JSON file"
        >
          <Upload size={13} />
          Import
        </button>
        <button
          className="btn"
          onClick={() => setShowForm((v) => !v)}
          disabled={!generator}
          title={
            generator
              ? "Generate a random input"
              : "No generator for this algorithm"
          }
        >
          <Dices size={13} />
          Random
        </button>
        <div className="flex-1" />
        <button
          className="btn btn-primary"
          onClick={handleExecute}
          disabled={isExecuting || !selectedAlgorithm}
        >
          <Play size={13} />
          {isExecuting ? "Running…" : "Run"}
        </button>
      </div>
    </div>
  );
}

export default memo(JsonCodeBlock);
