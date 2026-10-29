import { create } from "zustand";
import { applyTheme, readTheme } from "./theme";

/// Results are capped: an unbounded log of runs grows without limit and every
/// entry stays mounted in the DOM, which is what made the results panel get
/// slower the longer the app was open.
export const MAX_RESULTS = 50;

export const useAppStore = create((set) => ({
  selectedAlgorithm: null,
  setSelectedAlgorithm: (algo) => set({ selectedAlgorithm: algo }),

  results: [],
  // Functional update: the previous version closed over a stale `results`
  // array, so two quick runs could drop the first result.
  addResult: (result) =>
    set((state) => ({
      results: [result, ...state.results].slice(0, MAX_RESULTS),
    })),
  clearResults: () => set({ results: [] }),

  theme: readTheme(),
  toggleTheme: () =>
    set((state) => {
      const next = state.theme === "dark" ? "light" : "dark";
      applyTheme(next);
      return { theme: next };
    }),
}));

export default useAppStore;
