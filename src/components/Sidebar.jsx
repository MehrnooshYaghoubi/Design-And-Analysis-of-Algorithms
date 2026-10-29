import { memo, useState } from "react";
import { NavLink } from "react-router-dom";
import { ChevronDown, ChevronRight, Home, GitCompare, FlaskConical } from "lucide-react";
import Panel from "./Panel";

/** The shared left column: navigation plus the algorithm tree. */
function Sidebar({ title, onSelect, isEnabled, selected, children }) {
  const [treeOpen, setTreeOpen] = useState(true);

  return (
    <nav
      className="w-72 shrink-0 h-full overflow-y-auto"
      style={{ borderRight: "1px solid var(--border)" }}
      aria-label="Algorithms"
    >
      <div className="p-4">
        <h2 className="label mb-3">{title}</h2>

        <ul>
          <li>
            <NavLink to="/" className="nav-item">
              <Home size={14} />
              Home
            </NavLink>
          </li>
          <li>
            <NavLink to="/algorithms" className="nav-item">
              <FlaskConical size={14} />
              Laboratory
            </NavLink>
          </li>
          <li>
            <NavLink to="/compare" className="nav-item">
              <GitCompare size={14} />
              Compare
            </NavLink>
          </li>
        </ul>

        <hr className="rule my-4" />

        <ul>
          <li>
            <button
              type="button"
              className="nav-item"
              onClick={() => setTreeOpen((v) => !v)}
              aria-expanded={treeOpen}
            >
              {treeOpen ? <ChevronDown size={13} /> : <ChevronRight size={13} />}
              <span className="flex-1">Algorithms</span>
            </button>
          </li>
          {treeOpen && (
            <Panel onSelect={onSelect} isEnabled={isEnabled} selected={selected} />
          )}
        </ul>

        {children}
      </div>
    </nav>
  );
}

export default memo(Sidebar);
