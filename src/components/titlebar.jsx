import { Minus, Square, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import "../css/titlebar.css";

export default function Header() {
  let appWindow;
  try {
    appWindow = getCurrentWindow();
  } catch (error) {
    console.error("Error getting current window:", error);
  }

  document
    .getElementById("titlebar-minimize")
    ?.addEventListener("click", () => appWindow.minimize());
  document
    .getElementById("titlebar-maximize")
    ?.addEventListener("click", () => appWindow.toggleMaximize());
  document
    .getElementById("titlebar-close")
    ?.addEventListener("click", () => appWindow.close());

  return (
    <header
      data-tauri-drag-region
      className="flex w-full justify-between py-3 pl-10 pr-4 border-b-1 border-gray-800 solid select-none drag-region items-center h-[50px] z-100"
    >
      <div className="flex">
        <img
          src="./Logomark.svg"
          alt="logo"
          className="mr-3 w-[20px] aspect-auto"
        />
        <h3>KlugNetz</h3>
      </div>
      <div className="flex items-center justify-around">
        <button
          className="mr-4 hover:text-red-400 transition duration-300 ease-in-out cursor-pointer w-4 cursp"
          onClick={() => appWindow.minimize()}
        >
          <Minus size={16} />
        </button>
        <button
          className="mr-4 hover:text-red-400 transition duration-300 ease-in-out cursor-pointer w-4"
          onClick={() => appWindow.toggleMaximize()}
        >
          <Square size={15} />
        </button>
        <button onClick={() => appWindow.close}>
          <X
            size={16}
            className=" hover:text-red-400 transition duration-300 ease-in-out cursor-pointer w-4"
          />
        </button>
      </div>
    </header>
  );
}
