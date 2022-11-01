import { open, message } from "@tauri-apps/api/dialog";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { useEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Buffer } from "buffer";
import debounce from "lodash.debounce";
import { appDir } from "@tauri-apps/api/path";
import { toBase64 } from "../utils";

export default function Menu() {
  const [isDrag, setIsDrag] = useState<boolean>(false);
  const navigate = useNavigate();

  const handleNavigate = (filePath: string) => {
    const encode = toBase64(filePath);
    navigate(`/convert/${encode}`);
  };

  const debouncedEventHandler = useMemo(
    () => debounce(handleNavigate, 300),
    []
  );

  useEffect(() => {
    let unlisten: UnlistenFn;
    let unlistenFileDrop: UnlistenFn;
    let unlistenFileDropCancelled: UnlistenFn;

    const startFileDropHover = async () => {
      unlistenFileDrop = await listen(
        "tauri://file-drop-hover",
        (event: any) => {
          setIsDrag(true);
        }
      );
    };

    const startFileDropCancelled = async () => {
      unlistenFileDrop = await listen(
        "tauri://file-drop-cancelled",
        (event: any) => {
          setIsDrag(false);
        }
      );
    };

    const startFileDrop = async () => {
      unlisten = await listen("tauri://file-drop", (event: any) => {
        if (!event.payload) {
          return;
        }

        if (!Array.isArray(event.payload)) {
          return;
        }

        debouncedEventHandler(event.payload[0]);
      });
    };

    startFileDrop();
    startFileDropHover();
    startFileDropCancelled();

    return function cleanup() {
      if (unlisten) {
        unlisten();
      }

      if (unlistenFileDrop) {
        unlistenFileDrop();
      }

      if (unlistenFileDropCancelled) {
        unlistenFileDropCancelled();
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

    debouncedEventHandler(selected);
  };

  return (
    <div
      onClick={fileClick}
      className={`h-screen flex justify-center items-center cursor-pointer ${
        isDrag ? "bg-gray-900 blur duration-100 transition-all ease-out" : ""
      }`}
    >
      <h1 className="text-white text-2xl font-bold text-center">
        Drag and Drop Video to Compress
      </h1>
    </div>
  );
}
