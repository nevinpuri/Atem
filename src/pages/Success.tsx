import { invoke, shell } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Buffer } from "buffer";

export default function Success() {
  const params = useParams();
  const router = useNavigate();

  const [outputFolder, setOutputFolder] = useState<string>();

  useEffect(() => {
    if (!params.outputFolder) {
      return;
    }

    setOutputFolder(
      Buffer.from(params.outputFolder, "base64").toString("binary")
    );
  }, []);

  const openOutputFolder = async () => {
    if (!params.outputFolder) {
      return;
    }

    console.log(Buffer.from(params.outputFolder, "base64").toString("binary"));

    if (!outputFolder) {
      return;
    }

    await invoke("open_file_explorer", { path: outputFolder });
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
