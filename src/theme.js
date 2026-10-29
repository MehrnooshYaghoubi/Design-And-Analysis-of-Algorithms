// Theme persistence. Every localStorage access is guarded: in a private
// window, or with site data blocked, reads and writes can throw, and the app
// must still render.

const KEY = "klugnetz-theme";

export function readTheme() {
  try {
    const saved = localStorage.getItem(KEY);
    if (saved === "dark" || saved === "light") return saved;
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";
  } catch {
    return "dark";
  }
}

export function applyTheme(theme) {
  document.documentElement.dataset.theme = theme;
  try {
    localStorage.setItem(KEY, theme);
  } catch {
    // Not persisting is acceptable; the current session still looks right.
  }
}
