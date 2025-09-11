import { useAppStore } from "./../store";
import { useEffect, useState } from "react";

export default function Panel({ click = null }) {
  const [structure, setStructure] = useState({});
  const [openFolders, setOpenFolders] = useState({});
  const selectedAlgorithm = useAppStore((state) => state.selectedAlgorithm);
  const setSelectedAlgorithm = useAppStore(
    (state) => state.setSelectedAlgorithm
  );

  useEffect(() => {
    fetch("/folder_structure.json")
      .then((res) => res.json())
      .then((data) => setStructure(data))
      .catch((err) => console.error("Error loading JSON:", err));
  }, []);

  const toggleFolder = (folder) => {
    setOpenFolders((prev) => ({ ...prev, [folder]: !prev[folder] }));
  };

  return (
    <>
      {Object.entries(structure).map(([folder, files]) => (
        <li key={folder}>
          {/* Group on the clickable div only */}
          <div
            onClick={() => toggleFolder(folder)}
            className="text-sm flex items-center p-2 text-gray-300 hover:text-violet-500 hover:bg-gray-700/50 rounded-lg transition-all duration-200 border-l-2 border-transparent hover:border-[oklch(91%_0.096_180.426)] cursor-pointer"
          >
            <span className="w-2 h-2 bg-violet-800 rounded-full mr-3 opacity-0 group-hover:opacity-100 transition-opacity"></span>
            <span className="flex-1 capitalize">
              {folder.replace(/_/g, " ")}
            </span>
            <span>{openFolders[folder] ? "▾" : "▸"}</span>
          </div>

          {openFolders[folder] &&
            files.map((file) => {
              const name = file.replace(".rs", "").replace(/_/g, " ");
              const isSelected = name === selectedAlgorithm;
              return (
                <li key={file}>
                  <div
                    onClick={(e) => {
                      e.stopPropagation(); // prevent folder toggle
                      click ? click(name) : setSelectedAlgorithm(name);
                    }}
                    className={
                      isSelected
                        ? "group ml-6 flex items-center p-2 text-violet-500 bg-gray-700/50 rounded-lg transition-all duration-200 border-l-2 border-[oklch(91%_0.096_180.426)] cursor-pointer text-sm my-1"
                        : "group ml-6 flex items-center p-2 text-gray-300 hover:text-violet-500 hover:bg-gray-700/50 rounded-lg transition-all duration-200 border-l-2 border-transparent hover:border-[oklch(91%_0.096_180.426)] cursor-pointer text-sm my-1"
                    }
                  >
                    <span className=" w-2 h-2 bg-violet-800 rounded-full mr-3 opacity-0 group-hover:opacity-100 transition-opacity"></span>
                    {name}
                  </div>
                </li>
              );
            })}
        </li>
      ))}
    </>
  );
}
