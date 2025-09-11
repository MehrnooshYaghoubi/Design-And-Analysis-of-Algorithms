// src/main.jsx
import React from "react";
import ReactDOM from "react-dom/client";
import { HashRouter, Routes, Route } from "react-router-dom";
import App from "./App";
import Algorithms from "./pages/Algorithms";
import Compare from "./pages/compare";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <HashRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/algorithms" element={<Algorithms />} />
        <Route path="/compare" element={<Compare />} />
      </Routes>
    </HashRouter>
  </React.StrictMode>
);
