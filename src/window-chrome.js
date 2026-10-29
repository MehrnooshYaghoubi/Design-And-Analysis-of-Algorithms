// Keeps `data-window` on <html> in step with the real window state, so the
// stylesheet can drop the shell's rounded corners while maximised. Without
// this the radius cuts four transparent notches out of a window that is
// supposed to be flush with the screen edges.
//
// Everything here is guarded: in a plain browser tab there is no Tauri window
// to ask, and the app must still render. It then stays "normal", which is the
// right answer for a browser anyway.

import { getCurrentWindow } from "@tauri-apps/api/window";

function set(maximized) {
  document.documentElement.dataset.window = maximized ? "maximized" : "normal";
}

/**
 * Starts tracking the window state.
 *
 * @returns a function that stops tracking.
 */
export function watchWindowState() {
  set(false);

  let unlisten = null;
  let cancelled = false;

  (async () => {
    try {
      const win = getCurrentWindow();
      const sync = async () => set(await win.isMaximized());
      await sync();

      // Covers the titlebar's own maximise button as well as a double-click
      // on the drag region or a window-manager keybinding, none of which
      // report through the same path.
      const stop = await win.onResized(sync);
      if (cancelled) stop();
      else unlisten = stop;
    } catch {
      // Not running under Tauri; the default already applied.
    }
  })();

  return () => {
    cancelled = true;
    if (unlisten) unlisten();
  };
}
