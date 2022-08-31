import { useParams } from "react-router-dom";
import { Buffer } from "buffer";
import { useEffect, useMemo, useState } from "react";
import { message } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import debounce from "lodash.debounce";

export default function Convert() {
  const [convertStatus, setConvertStatus] = useState<string>("Converting");
  const params = useParams();

  const convertVideo = async () => {
    try {
      console.log("converting");
      if (!params.filePath) {
        message("No file");
        return;
      }

      const filePath = Buffer.from(params.filePath, "base64").toString(
        "binary"
      );

      await invoke("convert_video", { input: filePath, targetSize: 7.8 });
      setConvertStatus("Successfully converted");
      // open folder
    } catch (err) {
      setConvertStatus(
        "An unexpected error has occured. Check the console for details."
      );
    }
  };

  const debouncedEventHandler = useMemo(() => debounce(convertVideo, 300), []);
  useEffect(() => {
    debouncedEventHandler();
  }, []);
  return (
    <div className="h-screen flex justify-center items-center">
      <h1 className="text-white text-2xl font-bold text-center">
        {convertStatus}
      </h1>
    </div>
  );
}
