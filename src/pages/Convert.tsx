import { useParams } from "react-router-dom";
import { Buffer } from "buffer";
import { useEffect, useMemo } from "react";
import { message } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api";
import debounce from "lodash.debounce";

export default function Convert() {
  const params = useParams();

  const convertVideo = async () => {
    console.log("converting");
    if (!params.filePath) {
      message("No file");
      return;
    }

    const filePath = Buffer.from(params.filePath, "base64").toString("binary");

    await invoke("convert_video", { input: filePath, targetSize: 7.8 });
    message("done");
  };

  const debouncedEventHandler = useMemo(() => debounce(convertVideo, 300), []);
  useEffect(() => {
    debouncedEventHandler();
  }, []);
  return (
    <div>
      <h1>hello</h1>
      <h1 className="text-white">
        {Buffer.from(
          params.filePath ? params.filePath : "hello",
          "base64"
        ).toString("binary")}
      </h1>
    </div>
  );
}
