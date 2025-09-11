import { useState } from "react";
import Header from "../components/titlebar.jsx";
import JsonCodeBlock from "./../components/JsonCodeBlock.jsx";
import { ChevronDown, ChevronRight, Home } from "lucide-react";
import { NavLink } from "react-router-dom";
import Panel from "./../components/Panel.jsx";
import { useAppStore } from "./../store.jsx";

const Dashboard = () => {
  const [inputValue, setInputValue] = useState("");
  const results = useAppStore((state) => state.results);
  const [algorithmsVisible, setAlgorithmsVisible] = useState(false);

  return (
    <main className="font-[Montserrat] h-screen  overflow-hidden  ">
      <Header />

      {/* Glowing orb decor */}

      <div className="relative flex h-full">
        {/* Left Panel */}
        <div className="w-80 backdrop-blur-md p-4 h-full  border-r border-gray-800 overflow-y-auto pb-[5em]">
          <h2 className="text-xl font-bold mb-6 text-emerald-300 flex items-center">
            <span className="w-3 h-3 bg-emerald-300 rounded-full mr-2 animate-pulse"></span>
            Algorithms
          </h2>

          <ul className="space-y-3">
            <li className="group">
              <NavLink
                to="/"
                className="text-sm flex items-center p-2 text-gray-300 hover:text-[oklch(91%_0.096_180.426)] hover:bg-gray-700/50 rounded-lg transition-all duration-200 border-l-2 border-transparent group-hover:border-[oklch(91%_0.096_180.426)]"
              >
                <Home className="w-5 h-5  mr-4 transition-opacity" />
                Home Page
              </NavLink>
            </li>

            <li
              className="group"
              onClick={() => setAlgorithmsVisible(!algorithmsVisible)}
            >
              <a className="text-sm flex items-center p-2 w-full text-gray-300 hover:text-[oklch(91%_0.096_180.426)] hover:bg-gray-700/50 rounded-lg transition-all duration-200 border-l-2 border-transparent group-hover:border-[oklch(91%_0.096_180.426)]">
                <span className="w-2 h-2 bg-[oklch(91%_0.096_180.426)] rounded-full mr-3 opacity-0 group-hover:opacity-100 transition-opacity"></span>
                Algorithms
                {algorithmsVisible ? (
                  <ChevronDown size={15} className="ml-auto" />
                ) : (
                  <ChevronRight size={15} className="ml-auto" />
                )}
              </a>
            </li>

            {algorithmsVisible && <Panel />}
          </ul>
        </div>

        {/* Main Content Area */}
        <div className="h-full w-full flex flex-col p-4 justify-between">
          <div className="flex space-x-6">
            {/* Input Panel */}
            <div className="flex-1 backdrop-blur-md rounded-xl border border-gray-800 p-5 shadow-lg shadow-emerald-500/5 ">
              <JsonCodeBlock />
            </div>

            {/* Results and Graph */}
            <div className="flex-1 space-y-6">
              {/* Results Panel */}
              <div className="h-full backdrop-blur-md rounded-xl border border-gray-800 p-5 shadow-lg shadow-emerald-500/5">
                <h3 className="font mb-4 text-emerald-300 flex items-center">
                  <span className="w-2 h-2 bg-emerald-300 rounded-full mr-2"></span>
                  PROCESSING RESULTS
                </h3>
                <div className="space-y-3 max-h-[40vh] overflow-y-auto">
                  {results.length > 0 ? (
                    results.map((result, index) => (
                      <div
                        key={index}
                        className="bg-gray-900/50 border border-gray-700 rounded-lg px-4 py-3 flex items-start"
                      >
                        <div className="flex-shrink-0 mt-1">
                          <div className="w-2 h-2 bg-cyan-100 rounded-full animate-pulse"></div>
                        </div>
                        <div className="ml-3">
                          <div className="text-sm font-mono text-cyan-100">
                            {result.text}
                          </div>
                          <div>{result.algo}</div>
                          <div className="text-xs text-gray-500 mt-1 flex justify-between w-full">
                            <h3>{result.time}</h3>
                            <h3>{result.duration}</h3>
                          </div>
                        </div>
                      </div>
                    ))
                  ) : (
                    <div className="flex flex-col items-center justify-center text-center py-8 text-gray-500">
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="36"
                        height="36"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="1.3"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        className="lucide lucide-frown-icon"
                      >
                        <circle cx="12" cy="12" r="10" />
                        <path d="M16 16s-1.5-2-4-2-4 2-4 2" />
                        <line x1="9" y1="9" x2="9.01" y2="9" />
                        <line x1="15" y1="9" x2="15.01" y2="9" />
                      </svg>
                      <p className="mt-2">No results generated yet</p>
                      <p className="text-sm mt-1">
                        Run an algorithm to see results
                      </p>
                    </div>
                  )}
                </div>
              </div>

              {/* Graph Panel */}
            </div>
          </div>
        </div>
      </div>
    </main>
  );
};

export default Dashboard;
