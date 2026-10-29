import { memo, useCallback } from "react";
import { Minus, Square, X, Moon, Sun } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useAppStore } from "../store";
import Logomark from "./Logomark";

/// Resolved once per call rather than at module scope, so a failure in a
/// non-Tauri context (a plain browser tab) degrades to a no-op instead of
/// throwing during render.
function withWindow(action) {
  return async () => {
    try {
      await action(getCurrentWindow());
    } catch (error) {
      console.error("Window action failed:", error);
    }
  };
}

function Titlebar() {
  const theme = useAppStore((s) => s.theme);
  const toggleTheme = useAppStore((s) => s.toggleTheme);

  const minimize = useCallback(withWindow((w) => w.minimize()), []);
  const maximize = useCallback(withWindow((w) => w.toggleMaximize()), []);
  // This is the bug behind "the cross button sometimes does not work": the
  // old handler was `() => appWindow.close` -- it read the method and threw
  // it away without calling it. The missing `core:window:allow-close`
  // permission would have blocked it even once called.
  const close = useCallback(withWindow((w) => w.close()), []);

  const nextTheme = theme === "dark" ? "light" : "dark";

  return (
    <header className="titlebar" data-tauri-drag-region>
      <div className="flex items-center gap-3" data-tauri-drag-region>
        <Logomark size={16} className="opacity-80" />
        <span
          className="font-semibold tracking-tight"
          style={{ fontFamily: "var(--font-display)" }}
        >
          KlugNetz
        </span>
        <span className="label hidden sm:inline">Algorithms Laboratory</span>
      </div>

      <div className="flex items-center gap-1">
        <button
          className="titlebar-btn"
          onClick={toggleTheme}
          title={`Switch to ${nextTheme} mode`}
          aria-label={`Switch to ${nextTheme} mode`}
        >
          {theme === "dark" ? <Sun size={14} /> : <Moon size={14} />}
        </button>

        <div className="w-px h-4 mx-1" style={{ background: "var(--border)" }} />

        <button
          className="titlebar-btn"
          onClick={minimize}
          title="Minimise"
          aria-label="Minimise"
        >
          <Minus size={14} />
        </button>
        <button
          className="titlebar-btn"
          onClick={maximize}
          title="Maximise"
          aria-label="Maximise"
        >
          <Square size={12} />
        </button>
        <button
          className="titlebar-btn close"
          onClick={close}
          title="Close"
          aria-label="Close"
        >
          <X size={14} />
        </button>
      </div>
    </header>
  );
}

export default memo(Titlebar);
