import { create } from 'zustand';

export const useAppStore = create((set) => ({
  selectedAlgorithm: null,
  setSelectedAlgorithm: (algo) => set({ selectedAlgorithm: algo }),
  results: [],
  setResults: (result) => set({results: result})
}));

export default useAppStore;
