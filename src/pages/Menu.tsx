import { open, message } from "@tauri-apps/api/dialog";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { Buffer } from "buffer";

export default function Menu() {
  const navigate = useNavigate();
  useEffect(() => {
    let unlisten: UnlistenFn;
    const startFileDrop = async () => {
      console.log("hello");
      unlisten = await listen("tauri://file-drop", (event: any) => {
        if (!event.payload) {
          return;
        }

        if (!Array.isArray(event.payload)) {
          return;
        }

        console.log(event.payload[0]);
        handleNavigate(event.payload[0]);
      });
    };

    startFileDrop();

    return function cleanup() {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  const fileClick = async (e: any) => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Video",
          extensions: ["mp4", "mkv", "mov", "m4a"],
        },
      ],
    });

    if (Array.isArray(selected)) {
      await message("Please only select one file");
      return;
    } else if (selected === null) {
      return;
    }

    handleNavigate(selected);
  };

  const handleNavigate = (filePath: string) => {
    const encode = Buffer.from(filePath, "binary").toString("base64");
    navigate(`/convert/${encode}`);
  };
  return (
    <div
      onClick={fileClick}
      className="h-screen flex justify-center items-center cursor-pointer"
    >
      <h1 className="text-white text-2xl font-bold text-center">
        Drag and Drop Files to Compress
      </h1>
    </div>
  );
}