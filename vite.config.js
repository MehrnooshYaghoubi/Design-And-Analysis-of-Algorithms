import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [react(), tailwindcss()],

  build: {
    // The app ships inside a known webview, so there is no need to transpile
    // down for older browsers.
    target: "chrome110",
    // Smaller payload, and the app is not debugged from the shipped bundle.
    sourcemap: false,
    cssCodeSplit: false,
    rollupOptions: {
      output: {
        // Vendor code changes rarely; splitting it keeps the app chunk small
        // and lets the webview cache the rest between builds.
        manualChunks: {
          react: ["react", "react-dom", "react-router-dom"],
          editor: ["react-simple-code-editor", "prismjs"],
        },
      },
    },
  },

  // Pre-bundle these once instead of on first navigation.
  optimizeDeps: {
    include: ["react", "react-dom", "react-router-dom", "zustand", "prismjs"],
  },

  // Vite options tailored for Tauri development and only applied in
  // `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
