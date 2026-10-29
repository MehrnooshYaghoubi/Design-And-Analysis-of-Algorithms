import { memo, useCallback, useEffect, useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";
import { useAppStore } from "../store";

/// Module-level cache. The sidebar is mounted by two separate pages, and the
/// previous version re-fetched this file on every mount and every navigation.
let structurePromise = null;

function loadStructure() {
  if (!structurePromise) {
    structurePromise = fetch("/folder_structure.json")
      .then((res) => {
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        return res.json();
      })
      .catch((err) => {
        // Clear the cache so a later mount can retry a transient failure.
        structurePromise = null;
        throw err;
      });
  }
  return structurePromise;
}

export const algorithmNameOf = (file) =>
  file.replace(/\.rs$/, "").replace(/_/g, " ");

/**
 * The algorithm tree.
 *
 * @param onSelect   called with the algorithm name; falls back to the store
 * @param isEnabled  optional predicate to grey out algorithms a page cannot run
 * @param selected   name(s) to mark active; defaults to the store's selection
 */
function Panel({ onSelect, isEnabled, selected }) {
  const [structure, setStructure] = useState(null);
  const [error, setError] = useState(null);
  const [openFolders, setOpenFolders] = useState({});

  const storeSelected = useAppStore((s) => s.selectedAlgorithm);
  const setStoreSelected = useAppStore((s) => s.setSelectedAlgorithm);

  useEffect(() => {
    let alive = true;
    loadStructure()
      .then((data) => alive && setStructure(data))
      .catch((err) => alive && setError(err.message));
    return () => {
      alive = false;
    };
  }, []);

  const toggleFolder = useCallback((folder) => {
    setOpenFolders((prev) => ({ ...prev, [folder]: !prev[folder] }));
  }, []);

  const activeNames = selected ?? storeSelected;
  const isActive = (name) =>
    Array.isArray(activeNames) ? activeNames.includes(name) : activeNames === name;

  if (error) {
    return (
      <li className="px-3 py-2" style={{ color: "var(--fg-faint)" }}>
        Could not load the algorithm list ({error}).
      </li>
    );
  }

  if (!structure) {
    return (
      <li className="px-3 py-2" style={{ color: "var(--fg-faint)" }}>
        Loading…
      </li>
    );
  }

  return (
    <>
      {Object.entries(structure).map(([folder, files]) => {
        const open = !!openFolders[folder];
        return (
          <li key={folder}>
            <button
              type="button"
              className="nav-item"
              onClick={() => toggleFolder(folder)}
              aria-expanded={open}
            >
              {open ? <ChevronDown size={13} /> : <ChevronRight size={13} />}
              <span className="flex-1 capitalize">
                {folder.replace(/_/g, " ")}
              </span>
              <span className="num" style={{ color: "var(--fg-faint)" }}>
                {files.length}
              </span>
            </button>

            {/* A nested list, not <li> inside <li>, which was invalid markup. */}
            {open && (
              <ul className="mb-1">
                {files.map((file) => {
                  const name = algorithmNameOf(file);
                  const enabled = isEnabled ? isEnabled(name) : true;
                  return (
                    <li key={`${folder}/${file}`}>
                      <button
                        type="button"
                        disabled={!enabled}
                        title={
                          enabled
                            ? name
                            : `${name} cannot be benchmarked on this page`
                        }
                        className={`nav-item pl-9 ${isActive(name) ? "is-active" : ""}`}
                        onClick={() =>
                          onSelect ? onSelect(name) : setStoreSelected(name)
                        }
                      >
                        <span className="truncate">{name}</span>
                      </button>
                    </li>
                  );
                })}
              </ul>
            )}
          </li>
        );
      })}
    </>
  );
}

export default memo(Panel);
