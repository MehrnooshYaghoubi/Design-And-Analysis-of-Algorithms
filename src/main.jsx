import React from "react";
import ReactDOM from "react-dom/client";
import { HashRouter, Routes, Route, Navigate } from "react-router-dom";
import "./css/App.css";
import App from "./App";
import Algorithms from "./pages/Algorithms";
import Compare from "./pages/compare";
import { watchWindowState } from "./window-chrome";

// Started outside React: it belongs to the window, not to any one route, and
// survives StrictMode's double mount this way.
watchWindowState();

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    {/* The window has no OS frame, so this element *is* the window: it draws
        the background, the hairline edge and the rounded corners. */}
    <div className="app-shell">
      <HashRouter>
        <Routes>
          <Route path="/" element={<App />} />
          <Route path="/algorithms" element={<Algorithms />} />
          <Route path="/compare" element={<Compare />} />
          {/* Anything else returns home rather than rendering a blank window. */}
          <Route path="*" element={<Navigate to="/" replace />} />
        </Routes>
      </HashRouter>
    </div>
  </React.StrictMode>
);
