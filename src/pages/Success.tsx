import { invoke } from "@tauri-apps/api";
import { useNavigate, useParams } from "react-router-dom";
import { fromBase64 } from "../utils";

export default function Success() {
  const { outputFolder } = useParams();
  const router = useNavigate();

  const openOutputFolder = async () => {
    if (!outputFolder) {
      return;
    }

    await invoke("open_file_explorer", {
      path: fromBase64(outputFolder),
    });
  };

  const menu = () => {
    return router("menu");
  };

  return (
    <div className="h-screen flex flex-col justify-center items-center">
      <h1 className="text-white text-2xl font-bold text-center">
        Video Compressed Successfully
      </h1>
      <div className="flex flex-row space-x-2 mt-2">
        <a
          onClick={menu}
          href="/"
          className="bg-gray-800 text-gray-300 px-2 py-1.5"
        >
          Compress Another Video
        </a>
        <button
          onClick={openOutputFolder}
          className="bg-gray-800 text-gray-300 px-2 py-1.5"
        >
          Open Video Folder
        </button>
      </div>
    </div>
  );
}
