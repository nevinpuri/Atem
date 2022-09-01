import { useEffect, useState } from "react";
import { Link, useNavigate, useParams } from "react-router-dom";

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
        <button className="bg-gray-800 text-gray-300 px-2 py-1.5">
          Open Video Folder
        </button>
      </div>
    </div>
  );
}
